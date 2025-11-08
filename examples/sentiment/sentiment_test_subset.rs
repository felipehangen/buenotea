// Test sentiment analysis on a small subset of stocks
// This script tests the sentiment analysis on just a few stocks to verify everything works

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

    println!("=== Test Sentiment Analysis on Subset of Stocks ===\n");

    // Initialize services
    let sentiment_calculator = QSSCalculator::new();
    let sentiment_storage = SentimentStorage::from_env()?;
    let invite_list_storage = InviteListStorage::from_env()?;
    
    // Test database connections
    println!("🔗 Testing database connections...");
    sentiment_storage.test_connection().await?;
    println!("✅ Sentiment database connected successfully!");
    
    // Get a small subset of stocks for testing
    let test_symbols = vec!["AAPL", "MSFT", "GOOGL"]; // Test with just 3 major stocks
    println!("📋 Testing with {} stocks: {}", test_symbols.len(), test_symbols.join(", "));
    println!();

    let start_time = Instant::now();
    let mut successful_analyses = 0;
    let mut failed_analyses = 0;
    
    // Test each stock
    for (i, symbol) in test_symbols.iter().enumerate() {
        println!("📈 [{}/{}] Testing {}...", i + 1, test_symbols.len(), symbol);
        
        let analysis_start = Instant::now();
        
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
                        "qss_score": qss_result.qss_score,
                        "test_run": true
                    })),
                    
                    price_data_api_url: Some(format!("https://financialmodelingprep.com/api/v3/historical-price-full/{}", symbol)),
                    price_data_api_source: Some("FMP".to_string()),
                    price_data_available: true,
                    price_data_raw_data: Some(serde_json::json!({
                        "source": "fmp", 
                        "symbol": symbol, 
                        "timestamp": chrono::Utc::now(),
                        "rsi": qss_result.meta.rsi_14,
                        "test_run": true
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
        
        // Add delay between requests to avoid rate limits
        if i < test_symbols.len() - 1 {
            println!("    ⏳ Waiting 1s before next stock...");
            sleep(Duration::from_secs(1)).await;
        }
        
        println!();
    }
    
    let total_duration = start_time.elapsed();
    
    // Print summary
    println!("🎉 Test sentiment analysis completed!");
    println!("📊 Summary:");
    println!("   • Total stocks tested: {}", test_symbols.len());
    println!("   • Successful analyses: {}", successful_analyses);
    println!("   • Failed analyses: {}", failed_analyses);
    println!("   • Total time: {:.2}s", total_duration.as_secs_f64());
    println!("   • Average time per stock: {:.2}s", total_duration.as_secs_f64() / test_symbols.len() as f64);
    
    if successful_analyses > 0 {
        println!("\n📋 Retrieving test sentiment data from database...");
        
        match sentiment_storage.get_all_latest_sentiment().await {
            Ok(records) => {
                println!("✅ Found {} sentiment records in database", records.len());
                
                // Show test results
                println!("\n🧪 Test results from database:");
                for record in records.iter().filter(|r| test_symbols.contains(&r.symbol.as_str())) {
                    println!("   • {} - QSS: {:.3} | Signal: {} | Confidence: {:.1}%", 
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
    
    if successful_analyses == test_symbols.len() {
        println!("\n✅ All tests passed! You can now run the full batch analysis.");
        println!("💡 To run the full batch analysis, use:");
        println!("   cargo run --example sentiment_batch_robust --release");
    } else {
        println!("\n⚠️  Some tests failed. Please check your API keys and database connection.");
        println!("💡 Make sure you have valid API keys for FMP, Alpha Vantage, and Finnhub.");
    }
    
    Ok(())
}


