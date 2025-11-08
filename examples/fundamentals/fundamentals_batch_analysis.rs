// Batch fundamentals analysis for all stocks in invite_list table
// This example fetches all stock symbols from the invite_list table, runs fundamentals analysis,
// and saves the results to the fundamentals table in Supabase

use sentiment_backend::fundamentals::FundamentalsCalculator;
use sentiment_backend::database::fundamentals_storage::FundamentalsStorage;
use sentiment_backend::database::invite_list_storage::InviteListStorage;
use sentiment_backend::database::fundamentals_models::FundamentalsApiUrls;
use sentiment_backend::error::Result;
use tracing::{info, error, warn};
use std::time::Instant;
use tokio::time::{sleep, Duration};
use futures::future::join_all;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("🚀 Starting batch fundamentals analysis for all stocks in invite_list");
    
    // Load environment variables
    dotenv::dotenv().ok();

    // Create storage handlers
    let invite_list_storage = InviteListStorage::from_env()?;
    let fundamentals_storage = FundamentalsStorage::from_env()?;
    
    // Test database connections
    invite_list_storage.get_all_stock_symbols().await?;
    fundamentals_storage.test_connection().await?;

    // Create fundamentals calculator
    let calculator = FundamentalsCalculator::new();

    // Fetch all stock symbols from invite_list table
    info!("📊 Fetching all stock symbols from invite_list table...");
    let symbols = invite_list_storage.get_all_stock_symbols().await?;
    info!("✅ Found {} stocks to analyze", symbols.len());

    if symbols.is_empty() {
        warn!("⚠️  No stocks found in invite_list table. Please run invite_list analysis first.");
        return Ok(());
    }

    // Display the stocks to be analyzed
    info!("📋 Stocks to analyze:");
    for (i, symbol) in symbols.iter().enumerate() {
        if i < 10 {
            info!("  {}. {}", i + 1, symbol);
        } else if i == 10 {
            info!("  ... and {} more stocks", symbols.len() - 10);
            break;
        }
    }

    // Run batch analysis
    let start_time = Instant::now();
    let results = run_batch_fundamentals_analysis(&symbols, &calculator, &fundamentals_storage).await?;
    let total_time = start_time.elapsed();

    // Display summary
    display_batch_summary(&results, total_time);

    info!("🎉 Batch fundamentals analysis completed successfully!");
    Ok(())
}

/// Run fundamentals analysis on all symbols and save to database
async fn run_batch_fundamentals_analysis(
    symbols: &[String],
    calculator: &FundamentalsCalculator,
    storage: &FundamentalsStorage,
) -> Result<BatchResults> {
    let mut results = BatchResults::new();
    
    // Process stocks in batches to avoid overwhelming the system
    const BATCH_SIZE: usize = 5;
    const CONCURRENT_LIMIT: usize = 3;
    
    for chunk in symbols.chunks(BATCH_SIZE) {
        info!("🔄 Processing batch of {} stocks", chunk.len());
        
        // Process each batch with concurrency limit
        for concurrent_chunk in chunk.chunks(CONCURRENT_LIMIT) {
            let futures: Vec<_> = concurrent_chunk.iter().map(|symbol| {
                analyze_single_stock(symbol, calculator, storage)
            }).collect();
            
            let batch_results = join_all(futures).await;
            
            for result in batch_results {
                match result {
                    Ok(analysis_result) => {
                        results.successful += 1;
                        results.total_analyzed += 1;
                        
                        // Track trading signals
                        match analysis_result.trading_signal.as_str() {
                            "StrongBuy" => results.strong_buy += 1,
                            "WeakBuy" => results.weak_buy += 1,
                            "Hold" => results.hold += 1,
                            "WeakSell" => results.weak_sell += 1,
                            "StrongSell" => results.strong_sell += 1,
                            _ => {}
                        }
                        
                        info!("✅ Successfully analyzed {}", analysis_result.symbol);
                    }
                    Err(e) => {
                        results.failed += 1;
                        results.total_analyzed += 1;
                        error!("❌ Failed to analyze stock: {}", e);
                    }
                }
            }
        }
        
        // Add a small delay between batches to be respectful to APIs
        if chunk.len() == BATCH_SIZE {
            info!("⏸️  Waiting 2 seconds before next batch...");
            sleep(Duration::from_secs(2)).await;
        }
    }
    
    Ok(results)
}

/// Analyze a single stock and save to database
async fn analyze_single_stock(
    symbol: &str,
    calculator: &FundamentalsCalculator,
    storage: &FundamentalsStorage,
) -> Result<AnalysisResult> {
    let start_time = Instant::now();
    
    // Calculate fundamentals
    let fundamentals_result = calculator.calculate_fundamentals(symbol).await?;
    
    // Create API URLs metadata (for tracking data sources)
    let api_urls = FundamentalsApiUrls::default();
    
    // Store in database
    let stored_record = storage.store_fundamentals_result(
        symbol,
        &fundamentals_result,
        &api_urls,
        None, // GPT explanation
        None, // GPT trading suggestion
    ).await?;
    
    let analysis_time = start_time.elapsed();
    
    Ok(AnalysisResult {
        symbol: symbol.to_string(),
        fundamentals_score: fundamentals_result.fundamentals_score,
        trading_signal: fundamentals_result.trading_signal.to_string(),
        confidence_score: fundamentals_result.confidence_score,
        analysis_time_ms: analysis_time.as_millis() as u64,
        database_id: stored_record.id,
    })
}

/// Results of batch analysis
#[derive(Debug)]
struct BatchResults {
    total_analyzed: usize,
    successful: usize,
    failed: usize,
    strong_buy: usize,
    weak_buy: usize,
    hold: usize,
    weak_sell: usize,
    strong_sell: usize,
}

impl BatchResults {
    fn new() -> Self {
        Self {
            total_analyzed: 0,
            successful: 0,
            failed: 0,
            strong_buy: 0,
            weak_buy: 0,
            hold: 0,
            weak_sell: 0,
            strong_sell: 0,
        }
    }
}

/// Result of analyzing a single stock
#[derive(Debug)]
struct AnalysisResult {
    symbol: String,
    fundamentals_score: f64,
    trading_signal: String,
    confidence_score: f64,
    analysis_time_ms: u64,
    database_id: Option<i64>,
}

/// Display summary of batch analysis results
fn display_batch_summary(results: &BatchResults, total_time: std::time::Duration) {
    println!("\n{}", "=".repeat(60));
    println!("📊 BATCH FUNDAMENTALS ANALYSIS SUMMARY");
    println!("{}", "=".repeat(60));
    
    println!("⏱️  Total Time: {:.2} seconds", total_time.as_secs_f64());
    println!("📈 Total Stocks: {}", results.total_analyzed);
    println!("✅ Successful: {}", results.successful);
    println!("❌ Failed: {}", results.failed);
    
    if results.successful > 0 {
        let success_rate = (results.successful as f64 / results.total_analyzed as f64) * 100.0;
        println!("📊 Success Rate: {:.1}%", success_rate);
        
        let avg_time_per_stock = total_time.as_secs_f64() / results.successful as f64;
        println!("⚡ Average Time per Stock: {:.2} seconds", avg_time_per_stock);
    }
    
    println!("\n🎯 TRADING SIGNAL DISTRIBUTION:");
    println!("🟢 Strong Buy:  {}", results.strong_buy);
    println!("🟡 Weak Buy:    {}", results.weak_buy);
    println!("⚪ Hold:        {}", results.hold);
    println!("🟠 Weak Sell:   {}", results.weak_sell);
    println!("🔴 Strong Sell: {}", results.strong_sell);
    
    if results.successful > 0 {
        let buy_signals = results.strong_buy + results.weak_buy;
        let sell_signals = results.weak_sell + results.strong_sell;
        let buy_percentage = (buy_signals as f64 / results.successful as f64) * 100.0;
        let sell_percentage = (sell_signals as f64 / results.successful as f64) * 100.0;
        
        println!("\n📈 MARKET OUTLOOK:");
        println!("📈 Buy Signals: {:.1}%", buy_percentage);
        println!("📉 Sell Signals: {:.1}%", sell_percentage);
        
        if buy_percentage > sell_percentage {
            println!("🎯 Overall Market Sentiment: BULLISH");
        } else if sell_percentage > buy_percentage {
            println!("🎯 Overall Market Sentiment: BEARISH");
        } else {
            println!("🎯 Overall Market Sentiment: NEUTRAL");
        }
    }
    
    if results.failed > 0 {
        println!("\n⚠️  {} stocks failed analysis. Check logs for details.", results.failed);
    }
    
    println!("\n💾 All successful analyses have been saved to the fundamentals table.");
    println!("{}", "=".repeat(60));
}
