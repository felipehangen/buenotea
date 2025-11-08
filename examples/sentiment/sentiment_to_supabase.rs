// Example demonstrating sentiment analysis with Supabase storage
// This shows how to calculate QSS scores and save them to the database

use sentiment_backend::sentiment::QSSCalculator;
use sentiment_backend::database::{SentimentStorage, ApiUrls};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load environment variables
    dotenv().ok();

    println!("=== QSS Sentiment Analysis with Supabase Storage ===\n");

    // Initialize sentiment calculator
    let calculator = QSSCalculator::new();
    
    // Initialize database storage
    let storage = SentimentStorage::from_env()?;
    
    // Test database connection
    println!("🔗 Testing Supabase connection...");
    storage.test_connection().await?;
    println!("✅ Connected to Supabase successfully!\n");

    // Analyze multiple stocks and save to database
    let symbols = vec!["AAPL", "MSFT", "GOOGL", "TSLA", "NVDA"];
    
    for symbol in symbols {
        println!("📊 Analyzing {} and saving to database...", symbol);
        
        // Calculate sentiment
        let qss_result = calculator.calculate_qss(symbol).await?;
        
        // Create API URLs (will be populated with real data from the calculator)
        let api_urls = ApiUrls {
            earnings_api_url: Some(format!("https://www.alphavantage.co/query?function=EARNINGS_ESTIMATES&symbol={}", symbol)),
            earnings_api_source: Some("Alpha Vantage".to_string()),
            earnings_data_available: true,
            earnings_raw_data: Some(serde_json::json!({"source": "alpha_vantage", "symbol": symbol, "timestamp": chrono::Utc::now()})),
            
            price_data_api_url: Some(format!("https://financialmodelingprep.com/api/v3/historical-price-full/{}", symbol)),
            price_data_api_source: Some("FMP".to_string()),
            price_data_available: true,
            price_data_raw_data: Some(serde_json::json!({"source": "fmp", "symbol": symbol, "timestamp": chrono::Utc::now()})),
            
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
        match storage.store_sentiment_result(symbol, &qss_result, &api_urls).await {
            Ok(record) => {
                println!("✅ {} saved to database with ID: {}", symbol, record.id.unwrap_or(0));
                println!("   QSS Score: {:.3} | Signal: {:?} | Confidence: {:.1}%", 
                    qss_result.qss_score, 
                    qss_result.trading_signal,
                    qss_result.confidence_score * 100.0
                );
            }
            Err(e) => {
                println!("❌ Failed to save {}: {}", symbol, e);
            }
        }
        
        println!();
    }

    // Retrieve and display latest results
    println!("📋 Retrieving latest sentiment data from database...\n");
    
    let latest_records = storage.get_all_latest_sentiment().await?;
    
    if latest_records.is_empty() {
        println!("No records found in database.");
    } else {
        println!("Found {} sentiment records:\n", latest_records.len());
        
        for record in latest_records {
            println!("📈 {} (ID: {})", record.symbol, record.id.unwrap_or(0));
            println!("   Date: {}", record.analysis_date.format("%Y-%m-%d %H:%M:%S UTC"));
            println!("   QSS Score: {:.3}", record.qss_score);
            println!("   Trading Signal: {}", record.trading_signal);
            println!("   Confidence: {:.1}%", record.confidence_score * 100.0);
            println!("   Data Coverage: {:.1}%", record.data_coverage_percentage);
            println!("   GPT Explanation: {}", record.gpt_explanation);
            println!();
        }
    }

    println!("🎉 Sentiment analysis and storage completed successfully!");
    
    Ok(())
}
