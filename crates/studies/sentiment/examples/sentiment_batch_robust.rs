// Robust batch sentiment analysis with retry logic and comprehensive error handling
// This script processes all stocks from the invite_list table with retry mechanisms

use buenotea_sentiment::QSSCalculator;
use buenotea_database::{SentimentStorage, InviteListStorage, ApiUrls};
use buenotea_invite_list::models::InviteListRecord;
use dotenv::dotenv;
use std::time::Instant;
use tokio::time::{sleep, Duration};
use tokio::sync::Semaphore;
use tracing::{info, warn, error, debug};
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
    retry_count: u32,
}

#[derive(Debug, Clone)]
struct BatchConfig {
    max_concurrent: usize,
    max_retries: u32,
    retry_delay_ms: u64,
    skip_recent_hours: i64,
    delay_between_requests_ms: u64,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            max_concurrent: 3,
            max_retries: 3,
            retry_delay_ms: 2000,
            skip_recent_hours: 1,
            delay_between_requests_ms: 500,
        }
    }
}

async fn analyze_single_stock_with_retry(
    symbol: String,
    calculator: Arc<QSSCalculator>,
    storage: Arc<SentimentStorage>,
    semaphore: Arc<Semaphore>,
    config: BatchConfig,
) -> AnalysisResult {
    let _permit = semaphore.acquire().await.unwrap();
    
    let start_time = Instant::now();
    let symbol_clone = symbol.clone();
    let mut retry_count = 0;
    
    // Check if we should skip recent analysis
    match storage.get_latest_sentiment(&symbol).await {
        Ok(Some(existing_record)) => {
            let hours_since_analysis = chrono::Utc::now()
                .signed_duration_since(existing_record.analysis_date)
                .num_hours();
            
            if hours_since_analysis < config.skip_recent_hours {
                return AnalysisResult {
                    symbol: symbol_clone,
                    success: false,
                    qss_score: None,
                    trading_signal: None,
                    confidence_score: None,
                    error_message: Some(format!("Skipped - analyzed {} hours ago", hours_since_analysis)),
                    analysis_time_ms: 0,
                    retry_count: 0,
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
    
    // Retry logic for sentiment analysis
    let mut last_error = None;
    
    while retry_count <= config.max_retries {
        if retry_count > 0 {
            let delay = Duration::from_millis(config.retry_delay_ms * retry_count as u64);
            debug!("Retrying {} in {}ms (attempt {}/{})", symbol, delay.as_millis(), retry_count, config.max_retries);
            sleep(delay).await;
        }
        
        // Add delay between requests to avoid rate limits
        if retry_count == 0 {
            sleep(Duration::from_millis(config.delay_between_requests_ms)).await;
        }
        
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
                        "qss_score": qss_result.qss_score,
                        "retry_count": retry_count
                    })),
                    
                    price_data_api_url: Some(format!("https://financialmodelingprep.com/api/v3/historical-price-full/{}", symbol)),
                    price_data_api_source: Some("FMP".to_string()),
                    price_data_available: true,
                    price_data_raw_data: Some(serde_json::json!({
                        "source": "fmp", 
                        "symbol": symbol, 
                        "timestamp": chrono::Utc::now(),
                        "rsi": qss_result.meta.rsi_14,
                        "retry_count": retry_count
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
                
                // Store in database with retry logic
                let mut db_retry_count = 0;
                let max_db_retries = 2;
                
                while db_retry_count <= max_db_retries {
                    match storage.store_sentiment_result(&symbol, &qss_result, &api_urls).await {
                        Ok(record) => {
                            info!("✅ {} analyzed and saved (ID: {}, retries: {})", 
                                symbol, record.id.unwrap_or(0), retry_count);
                            return AnalysisResult {
                                symbol: symbol_clone,
                                success: true,
                                qss_score: Some(qss_result.qss_score),
                                trading_signal: Some(format!("{:?}", qss_result.trading_signal)),
                                confidence_score: Some(qss_result.confidence_score),
                                error_message: None,
                                analysis_time_ms: analysis_duration.as_millis() as u64,
                                retry_count,
                            };
                        }
                        Err(e) => {
                            db_retry_count += 1;
                            if db_retry_count <= max_db_retries {
                                warn!("Database save failed for {} (attempt {}): {}, retrying...", 
                                    symbol, db_retry_count, e);
                                sleep(Duration::from_millis(1000)).await;
                            } else {
                                error!("❌ Failed to save {} to database after {} attempts: {}", 
                                    symbol, max_db_retries + 1, e);
                                return AnalysisResult {
                                    symbol: symbol_clone,
                                    success: false,
                                    qss_score: Some(qss_result.qss_score),
                                    trading_signal: Some(format!("{:?}", qss_result.trading_signal)),
                                    confidence_score: Some(qss_result.confidence_score),
                                    error_message: Some(format!("Database save failed: {}", e)),
                                    analysis_time_ms: analysis_duration.as_millis() as u64,
                                    retry_count,
                                };
                            }
                        }
                    }
                }
            }
            Err(e) => {
                retry_count += 1;
                last_error = Some(e);
                
                if retry_count <= config.max_retries {
                    warn!("Analysis failed for {} (attempt {}): {}, retrying...", 
                        symbol, retry_count, last_error.as_ref().unwrap());
                } else {
                    error!("❌ Failed to analyze {} after {} attempts: {}", 
                        symbol, config.max_retries + 1, last_error.as_ref().unwrap());
                }
            }
        }
    }
    
    let analysis_duration = start_time.elapsed();
    AnalysisResult {
        symbol: symbol_clone,
        success: false,
        qss_score: None,
        trading_signal: None,
        confidence_score: None,
        error_message: Some(format!("Analysis failed after {} retries: {}", 
            config.max_retries, last_error.unwrap())),
        analysis_time_ms: analysis_duration.as_millis() as u64,
        retry_count,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load environment variables
    dotenv().ok();

    println!("=== Robust Parallel Batch Sentiment Analysis ===\n");

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

    // Configuration for robust processing
    let config = BatchConfig {
        max_concurrent: 3,
        max_retries: 3,
        retry_delay_ms: 2000,
        skip_recent_hours: 1,
        delay_between_requests_ms: 500,
    };
    
    let semaphore = Arc::new(Semaphore::new(config.max_concurrent));
    
    println!("🚀 Starting robust parallel sentiment analysis...");
    println!("📊 Configuration:");
    println!("   • Max concurrent analyses: {}", config.max_concurrent);
    println!("   • Max retries per stock: {}", config.max_retries);
    println!("   • Retry delay: {}ms", config.retry_delay_ms);
    println!("   • Skip recent analyses: {} hours", config.skip_recent_hours);
    println!("   • Delay between requests: {}ms", config.delay_between_requests_ms);
    println!();

    let start_time = Instant::now();
    
    let total_stocks = stock_symbols.len();
    
    // Create tasks for all stocks
    let mut tasks = Vec::new();
    for symbol in stock_symbols {
        let calculator = sentiment_calculator.clone();
        let storage = sentiment_storage.clone();
        let semaphore = semaphore.clone();
        let config = config.clone();
        
        let task = tokio::spawn(async move {
            analyze_single_stock_with_retry(symbol, calculator, storage, semaphore, config).await
        });
        
        tasks.push(task);
    }
    
    // Wait for all tasks to complete and collect results
    let mut results = Vec::new();
    let mut completed = 0;
    
    for task in tasks {
        match task.await {
            Ok(result) => {
                completed += 1;
                results.push(result.clone());
                
                let status = if result.success { 
                    format!("✅ Success (QSS: {:.3}, retries: {})", 
                        result.qss_score.unwrap_or(0.0), result.retry_count)
                } else if result.error_message.as_ref().map_or(false, |msg| msg.contains("Skipped")) {
                    format!("⏭️  Skipped")
                } else {
                    format!("❌ Failed: {} (retries: {})", 
                        result.error_message.as_ref().unwrap_or(&"Unknown error".to_string()), 
                        result.retry_count)
                };
                
                println!("📈 [{}/{}] {} - {}", completed, total_stocks, result.symbol, status);
            }
            Err(e) => {
                completed += 1;
                error!("Task failed: {}", e);
                results.push(AnalysisResult {
                    symbol: format!("Unknown_{}", completed),
                    success: false,
                    qss_score: None,
                    trading_signal: None,
                    confidence_score: None,
                    error_message: Some(format!("Task failed: {}", e)),
                    analysis_time_ms: 0,
                    retry_count: 0,
                });
            }
        }
    }
    
    let total_duration = start_time.elapsed();
    
    // Analyze results
    let successful: Vec<_> = results.iter().filter(|r| r.success).collect();
    let _failed: Vec<_> = results.iter().filter(|r| !r.success).collect();
    let skipped: Vec<_> = results.iter().filter(|r| r.error_message.as_ref().map_or(false, |msg| msg.contains("Skipped"))).collect();
    let actual_failures: Vec<_> = results.iter().filter(|r| !r.success && !r.error_message.as_ref().map_or(false, |msg| msg.contains("Skipped"))).collect();
    
    // Calculate statistics
    let total_retries: u32 = results.iter().map(|r| r.retry_count).sum();
    let avg_retries = if results.len() > 0 { total_retries as f64 / results.len() as f64 } else { 0.0 };
    let avg_analysis_time = if results.len() > 0 { 
        results.iter().map(|r| r.analysis_time_ms).sum::<u64>() as f64 / results.len() as f64 
    } else { 0.0 };
    
    // Print comprehensive summary
    println!("\n🎉 Robust parallel batch sentiment analysis completed!");
    println!("📊 Summary:");
    println!("   • Total stocks processed: {}", results.len());
    println!("   • Successful analyses: {}", successful.len());
    println!("   • Failed analyses: {}", actual_failures.len());
    println!("   • Skipped (recent data): {}", skipped.len());
    println!("   • Total retries: {}", total_retries);
    println!("   • Average retries per stock: {:.2}", avg_retries);
    println!("   • Total time: {:.2}s", total_duration.as_secs_f64());
    println!("   • Average time per stock: {:.2}s", avg_analysis_time / 1000.0);
    
    // Show successful results sorted by QSS score
    if !successful.is_empty() {
        let mut successful_sorted = successful.clone();
        successful_sorted.sort_by(|a, b| b.qss_score.partial_cmp(&a.qss_score).unwrap());
        
        println!("\n🏆 Top 10 successful analyses by QSS score:");
        for (i, result) in successful_sorted.iter().take(10).enumerate() {
            println!("   {}. {} - QSS: {:.3} | Signal: {} | Confidence: {:.1}% | Retries: {}", 
                i + 1,
                result.symbol,
                result.qss_score.unwrap_or(0.0),
                result.trading_signal.as_ref().unwrap_or(&"Unknown".to_string()),
                result.confidence_score.unwrap_or(0.0) * 100.0,
                result.retry_count
            );
        }
    }
    
    // Show failed analyses
    if !actual_failures.is_empty() {
        println!("\n❌ Failed analyses:");
        for result in actual_failures.iter() {
            println!("   • {}: {} (retries: {})", 
                result.symbol, 
                result.error_message.as_ref().unwrap_or(&"Unknown error".to_string()),
                result.retry_count);
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
