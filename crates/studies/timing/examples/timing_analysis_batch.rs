// Batch timing analysis for all stocks in the invite_list table
// This script fetches all stocks from the invite_list table, runs timing analysis on each,
// and saves the results to the timing table.

use buenotea_database::{InviteListStorage, TimingStorage};
use buenotea_timing::calculator::TTSCalculator;
use buenotea_database::timing_models::create_timing_record_with_tracking;
use tracing::{info, error, warn};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("Starting batch timing analysis for all stocks in invite_list table");

    // Initialize storage clients
    let invite_list_storage = InviteListStorage::from_env()?;
    let timing_storage = TimingStorage::from_env()?;

    // Fetch stock symbols from invite_list table (faster query)
    info!("Fetching stock symbols from invite_list table...");
    let start_fetch = Instant::now();
    let stock_symbols = invite_list_storage.get_all_stock_symbols().await?;
    let fetch_duration = start_fetch.elapsed();
    
    info!("Fetched {} stock symbols in {:.2} seconds", stock_symbols.len(), fetch_duration.as_secs_f64());

    if stock_symbols.is_empty() {
        warn!("No stocks found in invite_list table. Exiting.");
        return Ok(());
    }

    // Initialize TTS calculator
    let mut tts_calculator = TTSCalculator::new();

    // Statistics tracking
    let mut successful_analyses = 0;
    let mut failed_analyses = 0;
    let total_start_time = Instant::now();

    info!("Starting timing analysis for {} stocks...", stock_symbols.len());

    // Process each stock
    for (index, symbol) in stock_symbols.iter().enumerate() {
        let stock_start_time = Instant::now();
        
        info!("Processing stock {}/{}: {}", 
              index + 1, stock_symbols.len(), symbol);

        // Run timing analysis with API tracking
        match tts_calculator.calculate_tts_with_tracking(symbol).await {
            Ok((tts_result, api_tracking)) => {
                // Create timing record with API tracking data
                let timing_record = create_timing_record_with_tracking(tts_result, api_tracking);
                
                // Save to database
                match timing_storage.save_timing_record(timing_record).await {
                    Ok(saved_record) => {
                        let stock_duration = stock_start_time.elapsed();
                        info!("✓ Successfully analyzed and saved timing data for {} in {:.2}s (TTS Score: {:.2}, Signal: {})", 
                              symbol, stock_duration.as_secs_f64(), 
                              saved_record.tts_score, saved_record.trading_signal);
                        successful_analyses += 1;
                    }
                    Err(e) => {
                        error!("✗ Failed to save timing data for {}: {}", symbol, e);
                        failed_analyses += 1;
                    }
                }
            }
            Err(e) => {
                error!("✗ Failed to analyze timing for {}: {}", symbol, e);
                failed_analyses += 1;
            }
        }

        // Add a small delay to avoid overwhelming APIs
        if index < stock_symbols.len() - 1 {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }

    let total_duration = total_start_time.elapsed();

    // Print summary
    info!("{}", "=".repeat(60));
    info!("BATCH TIMING ANALYSIS COMPLETE");
    info!("{}", "=".repeat(60));
    info!("Total stocks processed: {}", stock_symbols.len());
    info!("Successful analyses: {}", successful_analyses);
    info!("Failed analyses: {}", failed_analyses);
    info!("Success rate: {:.1}%", (successful_analyses as f64 / stock_symbols.len() as f64) * 100.0);
    info!("Total processing time: {:.2} seconds", total_duration.as_secs_f64());
    info!("Average time per stock: {:.2} seconds", total_duration.as_secs_f64() / stock_symbols.len() as f64);
    
    if successful_analyses > 0 {
        info!("Timing analysis data has been saved to the timing table for {} stocks.", successful_analyses);
    }

    if failed_analyses > 0 {
        warn!("{} stocks failed timing analysis. Check the logs above for details.", failed_analyses);
    }

    Ok(())
}
