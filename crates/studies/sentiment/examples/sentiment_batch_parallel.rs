// Parallel batch sentiment analysis for all stocks in the invite_list table
// This script uses parallel processing to analyze multiple stocks simultaneously

use buenotea_sentiment::QSSCalculator;
use buenotea_database::{SentimentStorage, InviteListStorage, ApiUrls};
use buenotea_invite_list::models::InviteListRecord;
use dotenv::dotenv;
use std::time::Instant;
use tokio::time::{sleep, Duration};
use tokio::sync::Semaphore;
use tracing::{info, warn, error};
use std::sync::Arc;

#[derive(Debug, Clone)]
struct AnalysisResult {
    symbol: String,
    success: bool,
    qss_score: Option<f64>,
    trading_signal: Option<String>,
    confidence_score: Option<f64>,
    error_message: Option<String>,
    analysis_time_ms: u64,
}

async fn analyze_single_stock(
    symbol: String,
    calculator: Arc<QSSCalculator>,
    storage: Arc<SentimentStorage>,
    semaphore: Arc<Semaphore>,
    skip_recent: bool,
) -> AnalysisResult {
    let _permit = semaphore.acquire().await.unwrap();
    
    let start_time = Instant::now();
    let symbol_clone = symbol.clone();
    
    // Check if we should skip recent analysis
    if skip_recent {
        match storage.get_latest_sentiment(&symbol).await {
            Ok(Some(existing_record)) => {
                let hours_since_analysis = chrono::Utc::now()
                    .signed_duration_since(existing_record.analysis_date)
                    .num_hours();
                
                if hours_since_analysis < 1 {
                    return AnalysisResult {
                        symbol: symbol_clone,
                        success: false,
                        qss_score: None,
                        trading_signal: None,
                        confidence_score: None,
                        error_message: Some(format!("Skipped - analyzed {} hours ago", hours_since_analysis)),
                        analysis_time_ms: 0,
                    };
                }
            }
            Ok(None) => {
                // No existing data, proceed with analysis
            }
            Err(e) => {
                warn!("Could not check existing data for {}: {}", symbol, e);
                // Continue with analysis anyway
            }
        }
    }
    
    // Perform sentiment analysis
    match calculator.calculate_qss(&symbol).await {
        Ok(qss_result) => {
            let analysis_duration = start_time.elapsed();
            
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
            match storage.store_sentiment_result(&symbol, &qss_result, &api_urls).await {
                Ok(record) => {
                    info!("✅ {} analyzed and saved (ID: {})", symbol, record.id.unwrap_or(0));
                    AnalysisResult {
                        symbol: symbol_clone,
                        success: true,
                        qss_score: Some(qss_result.qss_score),
                        trading_signal: Some(format!("{:?}", qss_result.trading_signal)),
                        confidence_score: Some(qss_result.confidence_score),
                        error_message: None,
                        analysis_time_ms: analysis_duration.as_millis() as u64,
                    }
                }
                Err(e) => {
                    error!("❌ Failed to save {} to database: {}", symbol, e);
                    AnalysisResult {
                        symbol: symbol_clone,
                        success: false,
                        qss_score: Some(qss_result.qss_score),
                        trading_signal: Some(format!("{:?}", qss_result.trading_signal)),
                        confidence_score: Some(qss_result.confidence_score),
                        error_message: Some(format!("Database save failed: {}", e)),
                        analysis_time_ms: analysis_duration.as_millis() as u64,
                    }
                }
            }
        }
        Err(e) => {
            let analysis_duration = start_time.elapsed();
            error!("❌ Failed to analyze {}: {}", symbol, e);
            AnalysisResult {
                symbol: symbol_clone,
                success: false,
                qss_score: None,
                trading_signal: None,
                confidence_score: None,
                error_message: Some(format!("Analysis failed: {}", e)),
                analysis_time_ms: analysis_duration.as_millis() as u64,
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load environment variables
    dotenv().ok();

    println!("=== Parallel Batch Sentiment Analysis for All Invite List Stocks ===\n");

    // Initialize services
    let sentiment_calculator = Arc::new(QSSCalculator::new());
    let sentiment_storage = Arc::new(SentimentStorage::from_env()?);
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

    // Configuration for parallel processing
    let max_concurrent = 3; // Process up to 3 stocks simultaneously
    let semaphore = Arc::new(Semaphore::new(max_concurrent));
    let skip_recent = true; // Skip stocks analyzed within the last hour
    
    println!("🚀 Starting parallel sentiment analysis...");
    println!("📊 Processing {} stocks with max {} concurrent analyses", stock_symbols.len(), max_concurrent);
    println!("⏭️  Skipping stocks analyzed within the last hour: {}", skip_recent);
    println!();

    let start_time = Instant::now();
    
    // Create tasks for all stocks
    let mut tasks = Vec::new();
    for symbol in stock_symbols {
        let calculator = sentiment_calculator.clone();
        let storage = sentiment_storage.clone();
        let semaphore = semaphore.clone();
        
        let task = tokio::spawn(async move {
            analyze_single_stock(symbol, calculator, storage, semaphore, skip_recent).await
        });
        
        tasks.push(task);
    }
    
    // Wait for all tasks to complete and collect results
    let mut results = Vec::new();
    for (i, task) in tasks.into_iter().enumerate() {
        match task.await {
            Ok(result) => {
                results.push(result);
                println!("📈 [{}/{}] {} - {}", 
                    i + 1, 
                    stock_symbols.len(), 
                    result.symbol,
                    if result.success { 
                        format!("✅ Success (QSS: {:.3})", result.qss_score.unwrap_or(0.0))
                    } else {
                        format!("❌ Failed: {}", result.error_message.unwrap_or("Unknown error".to_string()))
                    }
                );
            }
            Err(e) => {
                error!("Task failed: {}", e);
                results.push(AnalysisResult {
                    symbol: format!("Unknown_{}", i),
                    success: false,
                    qss_score: None,
                    trading_signal: None,
                    confidence_score: None,
                    error_message: Some(format!("Task failed: {}", e)),
                    analysis_time_ms: 0,
                });
            }
        }
    }
    
    let total_duration = start_time.elapsed();
    
    // Analyze results
    let successful: Vec<_> = results.iter().filter(|r| r.success).collect();
    let failed: Vec<_> = results.iter().filter(|r| !r.success).collect();
    let skipped: Vec<_> = results.iter().filter(|r| r.error_message.as_ref().map_or(false, |msg| msg.contains("Skipped"))).collect();
    
    // Print summary
    println!("\n🎉 Parallel batch sentiment analysis completed!");
    println!("📊 Summary:");
    println!("   • Total stocks processed: {}", results.len());
    println!("   • Successful analyses: {}", successful.len());
    println!("   • Failed analyses: {}", failed.len() - skipped.len());
    println!("   • Skipped (recent data): {}", skipped.len());
    println!("   • Total time: {:.2}s", total_duration.as_secs_f64());
    println!("   • Average time per stock: {:.2}s", total_duration.as_secs_f64() / results.len() as f64);
    
    // Show successful results sorted by QSS score
    if !successful.is_empty() {
        let mut successful_sorted = successful.clone();
        successful_sorted.sort_by(|a, b| b.qss_score.partial_cmp(&a.qss_score).unwrap());
        
        println!("\n🏆 Top 10 successful analyses by QSS score:");
        for (i, result) in successful_sorted.iter().take(10).enumerate() {
            println!("   {}. {} - QSS: {:.3} | Signal: {} | Confidence: {:.1}%", 
                i + 1,
                result.symbol,
                result.qss_score.unwrap_or(0.0),
                result.trading_signal.as_ref().unwrap_or(&"Unknown".to_string()),
                result.confidence_score.unwrap_or(0.0) * 100.0
            );
        }
    }
    
    // Show failed analyses
    if !failed.is_empty() && failed.len() > skipped.len() {
        println!("\n❌ Failed analyses:");
        for result in failed.iter().filter(|r| !r.error_message.as_ref().map_or(false, |msg| msg.contains("Skipped"))) {
            println!("   • {}: {}", result.symbol, result.error_message.as_ref().unwrap_or(&"Unknown error".to_string()));
        }
    }
    
    // Retrieve and display latest sentiment data from database
    if !successful.is_empty() {
        println!("\n📋 Retrieving latest sentiment data from database...");
        
        match sentiment_storage.get_all_latest_sentiment().await {
            Ok(records) => {
                println!("✅ Found {} sentiment records in database", records.len());
                
                // Show top 5 by QSS score
                let mut sorted_records = records;
                sorted_records.sort_by(|a, b| b.qss_score.partial_cmp(&a.qss_score).unwrap());
                
                println!("\n🏆 Top 5 stocks by QSS score in database:");
                for (i, record) in sorted_records.iter().take(5).enumerate() {
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


