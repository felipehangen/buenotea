// Example: Run timing analysis on safe S&P 500 stocks and save to Supabase
// This analyzes stocks from the invite_list and saves timing signals to timing_history

use buenotea_timing::{TTSCalculator, TimingStorage, create_timing_record_with_tracking};
use buenotea_infrastructure::DatabaseClient;
use buenotea_core::Result;
use tokio;
use dotenv;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing for logging
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(tracing::Level::INFO)
            .finish()
    ).ok();

    println!("🚀 Starting S&P 500 Timing Analysis (Safe Stocks Only)...\n");

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

    let safe_stocks: Vec<Value> = response.json().await?;
    println!("✅ Found {} safe stocks to analyze\n", safe_stocks.len());

    // Step 2: Run timing analysis on each stock
    println!("🔍 Running timing analysis on {} stocks...", safe_stocks.len());
    println!("⏳ This will take several minutes...\n");

    let mut calculator = TTSCalculator::new();
    let mut all_records = Vec::new();
    let mut signal_counts = std::collections::HashMap::new();

    for (index, stock_data) in safe_stocks.iter().enumerate() {
        let symbol = stock_data["symbol"].as_str().unwrap_or("UNKNOWN");
        
        // Progress indicator every 50 stocks
        if index % 50 == 0 || index < 10 {
            println!("  [{}/{}] Analyzing {}...", 
                     index + 1, safe_stocks.len(), symbol);
        }

        // Run timing analysis
        match calculator.calculate_tts_with_tracking(symbol).await {
            Ok((result, tracking)) => {
                let signal = format!("{:?}", result.trading_signal);
                *signal_counts.entry(signal.clone()).or_insert(0) += 1;

                // Create database record
                let record = create_timing_record_with_tracking(result, tracking);
                all_records.push(record);

                // Show progress for first 10 and every 50th
                if index % 50 == 0 || index < 10 {
                    println!("    {} - Signal: {}", 
                             symbol, signal);
                }
            }
            Err(e) => {
                println!("    ⚠️  Failed to analyze {}: {}", symbol, e);
            }
        }

        // Progress indicator every 50 stocks
        if index % 50 == 0 {
            println!("  📊 Progress: {}/{} stocks analyzed ({}%)", 
                     index + 1, safe_stocks.len(), 
                     ((index + 1) * 100) / safe_stocks.len());
        }

        // Small delay to respect rate limits
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    println!("\n📊 Analysis Summary:");
    println!("  Total analyzed: {}", all_records.len());
    println!("  Signal Distribution:");
    for (signal, count) in &signal_counts {
        let percentage = (*count as f64 / all_records.len() as f64) * 100.0;
        println!("    {}: {} ({:.1}%)", signal, count, percentage);
    }

    // Step 3: Save to database in optimized batches
    println!("\n💾 Saving {} records to Supabase in optimized batches...", all_records.len());
    
    let storage = TimingStorage::new(db_client);

    // Use smaller batch sizes to avoid timeouts (optimized from invite_list experience)
    const BATCH_SIZE: usize = 15;
    let mut total_saved = 0;
    let mut total_failed = 0;

    for (batch_num, chunk) in all_records.chunks(BATCH_SIZE).enumerate() {
        println!("  📦 Saving batch {} ({} records)...", batch_num + 1, chunk.len());
        
        // Retry logic for each batch
        let mut retry_count = 0;
        let max_retries = 3;
        let mut batch_success = false;

        while retry_count < max_retries && !batch_success {
            match storage.store_multiple_records(chunk).await {
                Ok(ids) => {
                    total_saved += ids.len();
                    batch_success = true;
                    println!("    ✅ Batch {} saved successfully ({} records)", batch_num + 1, ids.len());
                }
                Err(e) => {
                    retry_count += 1;
                    println!("    ❌ Batch {} attempt {} failed: {}", batch_num + 1, retry_count, e);
                    
                    if retry_count < max_retries {
                        println!("    🔄 Retrying batch {} in 5 seconds...", batch_num + 1);
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    } else {
                        total_failed += chunk.len();
                        println!("    💥 Batch {} failed after {} attempts", batch_num + 1, max_retries);
                    }
                }
            }
        }

        // Delay between batches to avoid overwhelming Supabase
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    }
    
    println!("\n🎉 Batch Save Complete!");
    println!("  ✅ Successfully saved: {} records", total_saved);
    println!("  ❌ Failed to save: {} records", total_failed);
    println!("  📊 Success rate: {:.1}%", 
             (total_saved as f64 / all_records.len() as f64) * 100.0);
    
    // Display trading signals summary
    println!("\n📈 Trading Signals Summary:");
    let mut sorted_signals: Vec<_> = signal_counts.iter().collect();
    sorted_signals.sort_by(|a, b| b.1.cmp(a.1));
    
    for (signal, count) in sorted_signals {
        let percentage = (*count as f64 / all_records.len() as f64) * 100.0;
        let emoji = match signal.as_str() {
            "StrongBuy" => "🚀",
            "Buy" => "📈",
            "Neutral" => "➡️",
            "Sell" => "📉",
            "StrongSell" => "💥",
            _ => "❓",
        };
        println!("  {} {}: {} stocks ({:.1}%)", emoji, signal, count, percentage);
    }

    println!("\n🎉 Timing analysis completed successfully!");
    println!("💡 You can now query the 'timing' table in Supabase to see latest signals.");
    println!("💡 Use 'timing_history' to track signal changes over time.");
    println!("\n📚 Useful queries:");
    println!("  - Get all Buy signals: SELECT * FROM get_stocks_by_signal('Buy');");
    println!("  - Find recent signal changes: SELECT * FROM get_timing_signal_changes(7);");
    println!("  - Get AAPL history: SELECT * FROM get_timing_history('AAPL', 90);");

    Ok(())
}

