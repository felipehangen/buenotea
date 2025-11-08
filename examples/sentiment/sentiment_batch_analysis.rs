// Batch sentiment analysis for all stocks in the invite_list table
// This script fetches all stock tickers from the database and runs sentiment analysis on each one

use sentiment_backend::sentiment::QSSCalculator;
use sentiment_backend::database::{SentimentStorage, InviteListStorage, ApiUrls};
use sentiment_backend::invite_list::models::InviteListRecord;
use dotenv::dotenv;
use std::time::Instant;
use tokio::time::{sleep, Duration};
use tracing::{info, warn, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load environment variables
    dotenv().ok();

    println!("=== Batch Sentiment Analysis for All Invite List Stocks ===\n");

    // Initialize services
    let sentiment_calculator = QSSCalculator::new();
    let sentiment_storage = SentimentStorage::from_env()?;
    let invite_list_storage = InviteListStorage::from_env()?;
    
    // Test database connections
    println!("🔗 Testing database connections...");
    sentiment_storage.test_connection().await?;
    println!("✅ Sentiment database connected successfully!");
    
    // Fetch all stock symbols from invite_list table
    println!("📋 Fetching all stock symbols from invite_list table...");
    let stock_symbols = invite_list_storage.get_all_stock_symbols().await?;
    
    if stock_symbols.is_empty() {
        println!("❌ No stocks found in invite_list table. Please run invite list analysis first.");
        return Ok(());
    }
    
    println!("✅ Found {} stocks to analyze", stock_symbols.len());
    println!("📊 Stock symbols: {}", stock_symbols.join(", "));
    println!();

    // Configuration for batch processing
    let batch_size = 5; // Process 5 stocks at a time to avoid rate limits
    let delay_between_batches = Duration::from_secs(2); // 2 second delay between batches
    let delay_between_stocks = Duration::from_millis(500); // 500ms delay between individual stocks
    
    let total_batches = (stock_symbols.len() + batch_size - 1) / batch_size;
    let mut successful_analyses = 0;
    let mut failed_analyses = 0;
    let mut skipped_analyses = 0;
    
    let start_time = Instant::now();
    
    println!("🚀 Starting batch sentiment analysis...");
    println!("📊 Processing {} stocks in {} batches of {}", stock_symbols.len(), total_batches, batch_size);
    println!();

    // Process stocks in batches
    for (batch_index, stock_batch) in stock_symbols.chunks(batch_size).enumerate() {
        let batch_number = batch_index + 1;
        println!("📦 Processing batch {}/{} ({} stocks)", batch_number, total_batches, stock_batch.len());
        
        // Process each stock in the current batch
        for (stock_index, symbol) in stock_batch.iter().enumerate() {
            let stock_number = batch_index * batch_size + stock_index + 1;
            println!("  📈 [{}/{}] Analyzing {}...", stock_number, stock_symbols.len(), symbol);
            
            let analysis_start = Instant::now();
            
            // Check if we already have recent sentiment data for this stock
            match sentiment_storage.get_latest_sentiment(symbol).await {
                Ok(Some(existing_record)) => {
                    let hours_since_analysis = chrono::Utc::now()
                        .signed_duration_since(existing_record.analysis_date)
                        .num_hours();
                    
                    if hours_since_analysis < 1 { // Skip if analyzed within last hour
                        println!("    ⏭️  Skipping {} - analyzed {} hours ago", symbol, hours_since_analysis);
                        skipped_analyses += 1;
                        continue;
                    }
                }
                Ok(None) => {
                    // No existing data, proceed with analysis
                }
                Err(e) => {
                    warn!("    ⚠️  Could not check existing data for {}: {}", symbol, e);
                    // Continue with analysis anyway
                }
            }
            
            // Perform sentiment analysis
            match sentiment_calculator.calculate_qss(symbol).await {
                Ok(qss_result) => {
                    let analysis_duration = analysis_start.elapsed();
                    println!("    ✅ {} analyzed in {:.2}s", symbol, analysis_duration.as_secs_f64());
                    println!("       QSS Score: {:.3} | Signal: {:?} | Confidence: {:.1}%", 
                        qss_result.qss_score, 
                        qss_result.trading_signal,
                        qss_result.confidence_score * 100.0
                    );
                    
                    // Create API URLs for storage
                    let api_urls = ApiUrls {
                        earnings_api_url: Some(format!("https://www.alphavantage.co/query?function=EARNINGS_ESTIMATES&symbol={}", symbol)),
                        earnings_api_source: Some("Alpha Vantage".to_string()),
                        earnings_data_available: true,
                        earnings_raw_data: Some(serde_json::json!({
                            "source": "alpha_vantage", 
                            "symbol": symbol, 
                            "timestamp": chrono::Utc::now(),
                            "qss_score": qss_result.qss_score
                        })),
                        
                        price_data_api_url: Some(format!("https://financialmodelingprep.com/api/v3/historical-price-full/{}", symbol)),
                        price_data_api_source: Some("FMP".to_string()),
                        price_data_available: true,
                        price_data_raw_data: Some(serde_json::json!({
                            "source": "fmp", 
                            "symbol": symbol, 
                            "timestamp": chrono::Utc::now(),
                            "rsi": qss_result.meta.rsi_14
                        })),
                        
                        short_interest_api_url: None,
                        short_interest_api_source: None,
                        short_interest_data_available: false,
                        short_interest_raw_data: None,
                        
                        options_flow_api_url: None,
                        options_flow_api_source: None,
                        options_flow_data_available: false,
                        options_flow_raw_data: None,
                    };
                    
                    // Store in database
                    match sentiment_storage.store_sentiment_result(symbol, &qss_result, &api_urls).await {
                        Ok(record) => {
                            println!("    💾 Saved to database with ID: {}", record.id.unwrap_or(0));
                            successful_analyses += 1;
                        }
                        Err(e) => {
                            error!("    ❌ Failed to save {} to database: {}", symbol, e);
                            failed_analyses += 1;
                        }
                    }
                }
                Err(e) => {
                    error!("    ❌ Failed to analyze {}: {}", symbol, e);
                    failed_analyses += 1;
                }
            }
            
            // Add delay between stocks to avoid rate limits
            if stock_index < stock_batch.len() - 1 {
                sleep(delay_between_stocks).await;
            }
        }
        
        // Add delay between batches
        if batch_index < total_batches - 1 {
            println!("    ⏳ Waiting {}s before next batch...", delay_between_batches.as_secs());
            sleep(delay_between_batches).await;
        }
        
        println!();
    }
    
    let total_duration = start_time.elapsed();
    
    // Print summary
    println!("🎉 Batch sentiment analysis completed!");
    println!("📊 Summary:");
    println!("   • Total stocks processed: {}", stock_symbols.len());
    println!("   • Successful analyses: {}", successful_analyses);
    println!("   • Failed analyses: {}", failed_analyses);
    println!("   • Skipped (recent data): {}", skipped_analyses);
    println!("   • Total time: {:.2}s", total_duration.as_secs_f64());
    println!("   • Average time per stock: {:.2}s", total_duration.as_secs_f64() / stock_symbols.len() as f64);
    
    if successful_analyses > 0 {
        println!("\n📋 Retrieving latest sentiment data from database...");
        
        match sentiment_storage.get_all_latest_sentiment().await {
            Ok(records) => {
                println!("✅ Found {} sentiment records in database", records.len());
                
                // Show top 10 by QSS score
                let mut sorted_records = records;
                sorted_records.sort_by(|a, b| b.qss_score.partial_cmp(&a.qss_score).unwrap());
                
                println!("\n🏆 Top 10 stocks by QSS score:");
                for (i, record) in sorted_records.iter().take(10).enumerate() {
                    println!("   {}. {} - QSS: {:.3} | Signal: {} | Confidence: {:.1}%", 
                        i + 1,
                        record.symbol,
                        record.qss_score,
                        record.trading_signal,
                        record.confidence_score * 100.0
                    );
                }
            }
            Err(e) => {
                error!("❌ Failed to retrieve sentiment data: {}", e);
            }
        }
    }
    
    Ok(())
}


