// Force re-run sentiment analysis on all stocks with comprehensive rate limiting
// This script ignores recent analysis checks and re-analyzes all stocks

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
    delay_between_requests_ms: u64,
    delay_between_batches_ms: u64,
    batch_size: usize,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            max_concurrent: 2, // Reduced concurrent requests
            max_retries: 5, // More retries
            retry_delay_ms: 3000, // Longer retry delay
            delay_between_requests_ms: 1000, // 1 second between requests
            delay_between_batches_ms: 5000, // 5 seconds between batches
            batch_size: 3, // Smaller batches
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
    
    // Add delay between requests to avoid rate limits
    sleep(Duration::from_millis(config.delay_between_requests_ms)).await;
    
    // Retry logic for sentiment analysis
    let mut last_error = None;
    
    while retry_count <= config.max_retries {
        if retry_count > 0 {
            let delay = Duration::from_millis(config.retry_delay_ms * (retry_count as u64));
            debug!("Retrying {} in {}ms (attempt {}/{})", symbol, delay.as_millis(), retry_count, config.max_retries);
            sleep(delay).await;
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
                        "retry_count": retry_count,
                        "force_rerun": true
                    })),
                    
                    price_data_api_url: Some(format!("https://financialmodelingprep.com/api/v3/historical-price-full/{}", symbol)),
                    price_data_api_source: Some("FMP".to_string()),
                    price_data_available: true,
                    price_data_raw_data: Some(serde_json::json!({
                        "source": "fmp", 
                        "symbol": symbol, 
                        "timestamp": chrono::Utc::now(),
                        "rsi": qss_result.meta.rsi_14,
                        "retry_count": retry_count,
                        "force_rerun": true
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
                let max_db_retries = 3;
                
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
                                sleep(Duration::from_millis(2000)).await;
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

    println!("=== Force Re-run Sentiment Analysis with Comprehensive Rate Limiting ===\n");

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

    // Configuration for comprehensive rate limiting
    let config = BatchConfig {
        max_concurrent: 2, // Only 2 concurrent requests
        max_retries: 5, // More retries
        retry_delay_ms: 3000, // 3 second retry delay
        delay_between_requests_ms: 1000, // 1 second between requests
        delay_between_batches_ms: 5000, // 5 seconds between batches
        batch_size: 3, // Process 3 stocks per batch
    };
    
    let semaphore = Arc::new(Semaphore::new(config.max_concurrent));
    
    println!("🚀 Starting force re-run sentiment analysis...");
    println!("📊 Configuration:");
    println!("   • Max concurrent analyses: {}", config.max_concurrent);
    println!("   • Max retries per stock: {}", config.max_retries);
    println!("   • Retry delay: {}ms", config.retry_delay_ms);
    println!("   • Delay between requests: {}ms", config.delay_between_requests_ms);
    println!("   • Delay between batches: {}ms", config.delay_between_batches_ms);
    println!("   • Batch size: {}", config.batch_size);
    println!("   • Force re-run: YES (ignoring recent analysis)");
    println!();

    let start_time = Instant::now();
    let total_stocks = stock_symbols.len();
    let total_batches = (total_stocks + config.batch_size - 1) / config.batch_size;
    
    let mut all_results = Vec::new();
    let mut successful_analyses = 0;
    let mut failed_analyses = 0;
    
    // Process stocks in batches
    for (batch_index, stock_batch) in stock_symbols.chunks(config.batch_size).enumerate() {
        let batch_number = batch_index + 1;
        println!("📦 Processing batch {}/{} ({} stocks)", batch_number, total_batches, stock_batch.len());
        
        // Create tasks for current batch
        let mut tasks = Vec::new();
        for symbol in stock_batch.iter() {
            let symbol = symbol.clone();
            let calculator = sentiment_calculator.clone();
            let storage = sentiment_storage.clone();
            let semaphore = semaphore.clone();
            let config = config.clone();
            
            let task = tokio::spawn(async move {
                analyze_single_stock_with_retry(symbol, calculator, storage, semaphore, config).await
            });
            
            tasks.push(task);
        }
        
        // Wait for all tasks in current batch to complete
        for (task_index, task) in tasks.into_iter().enumerate() {
            let stock_index = batch_index * config.batch_size + task_index + 1;
            
            match task.await {
                Ok(result) => {
                    all_results.push(result.clone());
                    
                    let status = if result.success { 
                        successful_analyses += 1;
                        format!("✅ Success (QSS: {:.3}, retries: {})", 
                            result.qss_score.unwrap_or(0.0), result.retry_count)
                    } else {
                        failed_analyses += 1;
                        format!("❌ Failed: {} (retries: {})", 
                            result.error_message.as_ref().unwrap_or(&"Unknown error".to_string()), 
                            result.retry_count)
                    };
                    
                    println!("  📈 [{}/{}] {} - {}", stock_index, total_stocks, result.symbol, status);
                }
                Err(e) => {
                    error!("Task failed for stock {}: {}", stock_index, e);
                    failed_analyses += 1;
                }
            }
        }
        
        // Add delay between batches
        if batch_index < total_batches - 1 {
            println!("    ⏳ Waiting {}s before next batch...", config.delay_between_batches_ms / 1000);
            sleep(Duration::from_millis(config.delay_between_batches_ms)).await;
        }
        
        println!();
    }
    
    let total_duration = start_time.elapsed();
    
    // Calculate statistics
    let total_retries: u32 = all_results.iter().map(|r| r.retry_count).sum();
    let avg_retries = if all_results.len() > 0 { total_retries as f64 / all_results.len() as f64 } else { 0.0 };
    let avg_analysis_time = if all_results.len() > 0 { 
        all_results.iter().map(|r| r.analysis_time_ms).sum::<u64>() as f64 / all_results.len() as f64 
    } else { 0.0 };
    
    // Print comprehensive summary
    println!("🎉 Force re-run sentiment analysis completed!");
    println!("📊 Summary:");
    println!("   • Total stocks processed: {}", total_stocks);
    println!("   • Successful analyses: {}", successful_analyses);
    println!("   • Failed analyses: {}", failed_analyses);
    println!("   • Total retries: {}", total_retries);
    println!("   • Average retries per stock: {:.2}", avg_retries);
    println!("   • Total time: {:.2}s", total_duration.as_secs_f64());
    println!("   • Average time per stock: {:.2}s", avg_analysis_time / 1000.0);
    
    // Show successful results sorted by QSS score
    if successful_analyses > 0 {
        let successful: Vec<_> = all_results.iter().filter(|r| r.success).collect();
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
    let failed: Vec<_> = all_results.iter().filter(|r| !r.success).collect();
    if !failed.is_empty() {
        println!("\n❌ Failed analyses:");
        for result in failed.iter() {
            println!("   • {}: {} (retries: {})", 
                result.symbol, 
                result.error_message.as_ref().unwrap_or(&"Unknown error".to_string()),
                result.retry_count);
        }
    }
    
    // Retrieve and display latest sentiment data from database
    if successful_analyses > 0 {
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
