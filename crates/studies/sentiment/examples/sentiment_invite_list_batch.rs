// Example: Run sentiment (QSS) analysis on safe S&P 500 stocks and save to Supabase
// This analyzes stocks from the invite_list and saves sentiment signals to sentiment_history

use buenotea_sentiment::{QSSCalculator, SentimentStorage, create_sentiment_record_with_tracking};
use buenotea_infrastructure::sentiment_models::ApiUrls;
use buenotea_infrastructure::DatabaseClient;
use buenotea_core::Result;
use tokio;
use dotenv;
use serde_json::Value;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing for logging
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(tracing::Level::INFO)
            .finish()
    ).ok();

    println!("🚀 Starting S&P 500 Sentiment (QSS) Analysis (Safe Stocks Only)...\n");

    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Step 1: Fetch safe stocks from invite_list
    println!("📈 Fetching safe stocks from invite_list...");
    let db_client = DatabaseClient::from_env()?;
    
    let url = format!(
        "{}/rest/v1/invite_list?select=symbol,company_name,safety_score&is_safe_to_trade=eq.true&order=safety_score.desc",
        db_client.config().supabase_url
    );
    
    let response = reqwest::Client::new()
        .get(&url)
        .header("apikey", &db_client.config().supabase_api_key)
        .header("Authorization", format!("Bearer {}", db_client.config().supabase_api_key))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(buenotea_core::Error::DatabaseError(format!(
            "Failed to fetch safe stocks: {}",
            response.status()
        )));
    }

    let mut safe_stocks: Vec<Value> = response.json().await?;
    
    // LIMIT TO FIRST 10 STOCKS FOR TESTING (remove this in production)
    safe_stocks.truncate(10);
    
    println!("✅ Found {} safe stocks to analyze (limited to 10 for testing)\n", safe_stocks.len());

    // Step 2: Run sentiment analysis on all stocks
    println!("🔍 Running sentiment analysis on {} stocks...", safe_stocks.len());
    println!("⏳ This will take several minutes...\n");

    let calculator = QSSCalculator::new();
    let mut results = Vec::new();
    let mut successful = 0;
    let mut failed = 0;

    for (i, stock) in safe_stocks.iter().enumerate() {
        let symbol = stock["symbol"].as_str().unwrap_or("UNKNOWN");
        
        // Progress indicator
        if i % 50 == 0 && i > 0 {
            println!("  📊 Progress: {}/{} stocks analyzed ({}%)", 
                i, safe_stocks.len(), (i * 100) / safe_stocks.len());
        }

        // Rate limiting
        if i > 0 && i % 5 == 0 {
            sleep(Duration::from_millis(500)).await;
        }

        println!("  [{}/ {}] Analyzing {}...", i + 1, safe_stocks.len(), symbol);
        
        match calculator.calculate_qss(symbol).await {
            Ok(result) => {
                println!("    {} - Signal: {:?}", symbol, result.trading_signal);
                results.push(result);
                successful += 1;
            }
            Err(e) => {
                eprintln!("    ❌ Failed to analyze {}: {}", symbol, e);
                failed += 1;
            }
        }
    }

    println!("\n📊 Analysis Summary:");
    println!("  Total analyzed: {}", safe_stocks.len());
    println!("  Successful: {}", successful);
    println!("  Failed: {}", failed);

    // Step 3: Calculate signal distribution
    let mut strong_buy = 0;
    let mut weak_buy = 0;
    let mut hold = 0;
    let mut weak_sell = 0;
    let mut strong_sell = 0;

    for result in &results {
        match result.trading_signal {
            buenotea_sentiment::models::TradingSignal::StrongBuy => strong_buy += 1,
            buenotea_sentiment::models::TradingSignal::WeakBuy => weak_buy += 1,
            buenotea_sentiment::models::TradingSignal::Hold => hold += 1,
            buenotea_sentiment::models::TradingSignal::WeakSell => weak_sell += 1,
            buenotea_sentiment::models::TradingSignal::StrongSell => strong_sell += 1,
        }
    }

    println!("  Signal Distribution:");
    println!("    Strong Buy: {} ({:.1}%)", strong_buy, (strong_buy as f64 / results.len() as f64) * 100.0);
    println!("    Weak Buy: {} ({:.1}%)", weak_buy, (weak_buy as f64 / results.len() as f64) * 100.0);
    println!("    Hold: {} ({:.1}%)", hold, (hold as f64 / results.len() as f64) * 100.0);
    println!("    Weak Sell: {} ({:.1}%)", weak_sell, (weak_sell as f64 / results.len() as f64) * 100.0);
    println!("    Strong Sell: {} ({:.1}%)", strong_sell, (strong_sell as f64 / results.len() as f64) * 100.0);

    // Step 4: Save results to database
    println!("\n💾 Saving {} records to Supabase in optimized batches...", results.len());
    
    let sentiment_storage = SentimentStorage::new(db_client);
    
    // Prepare records for batch insertion
    let mut records_to_save = Vec::new();
    for result in results {
        let api_urls = ApiUrls::default(); // Using default for now
        let gpt_explanation = format!(
            "{} shows {:?} sentiment with QSS score of {:.3}. Confidence: {:.1}%",
            result.symbol,
            result.trading_signal,
            result.qss_score,
            result.confidence_score * 100.0
        );
        
        let record = create_sentiment_record_with_tracking(
            result,
            api_urls,
            gpt_explanation,
        );
        records_to_save.push(record);
    }

    // Batch save with manual batching
    let batch_size = 15;
    let total_batches = (records_to_save.len() + batch_size - 1) / batch_size;
    let mut saved_count = 0;
    let mut failed_count = 0;
    
    for (i, batch) in records_to_save.chunks(batch_size).enumerate() {
        let batch_num = i + 1;
        println!("  📦 Saving batch {}/{} ({} records)...", batch_num, total_batches, batch.len());
        
        match sentiment_storage.store_multiple_records(batch).await {
            Ok(ids) => {
                println!("    ✅ Batch {} saved successfully ({} records)", batch_num, ids.len());
                saved_count += ids.len();
            }
            Err(e) => {
                eprintln!("    ❌ Batch {} failed: {}", batch_num, e);
                failed_count += batch.len();
            }
        }
        
        // Delay between batches to avoid rate limits
        if i < total_batches - 1 {
            sleep(Duration::from_secs(3)).await;
        }
    }
    
    println!("\n🎉 Batch Save Complete!");
    println!("  ✅ Successfully saved: {} records", saved_count);
    println!("  ❌ Failed to save: {} records", failed_count);
    println!("  📊 Success rate: {:.1}%", (saved_count as f64 / (saved_count + failed_count) as f64) * 100.0);

    println!("\n📈 Sentiment Signals Summary:");
    println!("  🟢 Strong Buy: {} stocks", strong_buy);
    println!("  🟡 Weak Buy: {} stocks", weak_buy);
    println!("  ⚪ Hold: {} stocks", hold);
    println!("  🟠 Weak Sell: {} stocks", weak_sell);
    println!("  🔴 Strong Sell: {} stocks", strong_sell);

    println!("\n🎉 Sentiment analysis completed successfully!");
    println!("💡 You can now query the 'sentiment' table in Supabase to see latest signals.");
    println!("💡 Use 'sentiment_history' to track sentiment changes over time.");
    
    println!("\n📚 Useful queries:");
    println!("  - Get all Strong Buy signals: SELECT * FROM sentiment WHERE trading_signal = 'StrongBuy';");
    println!("  - Find recent sentiment changes: SELECT * FROM get_sentiment_changes('AAPL', 30);");
    println!("  - Get AAPL history: SELECT * FROM get_sentiment_history('AAPL', 90);");

    Ok(())
}

