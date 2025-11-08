// Example: Fetch S&P 500 stocks, analyze safety, and save to Supabase

use sentiment_backend::invite_list::{InviteListCalculator, SP500Fetcher, SafetyAnalysis};
use sentiment_backend::database::{DatabaseClient, InviteListStorage};
use sentiment_backend::invite_list::models::{ApiConfig, SP500Stock, InviteListRecord};
use sentiment_backend::error::Result;
use tokio;
use std::collections::HashMap;
use dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    println!("🚀 Starting S&P 500 safety analysis and saving to Supabase...\n");

    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Load API configuration from environment variables
    let api_config = ApiConfig::from_env()
        .map_err(|e| {
            eprintln!("❌ Failed to load API configuration: {}", e);
            eprintln!("Please ensure the following environment variables are set in .env file:");
            eprintln!("  - FMP_API_KEY");
            eprintln!("  - SUPABASE_URL");
            eprintln!("  - SUPABASE_API_KEY");
            e
        })?;

    println!("✅ API configuration loaded successfully!\n");

    // Initialize components
    let fetcher = SP500Fetcher::new(api_config.clone());
    let calculator = InviteListCalculator::new(api_config.clone());

    // Step 1: Fetch S&P 500 stock list
    println!("📈 Fetching S&P 500 stock list...");
    let sp500_stocks = fetcher.fetch_sp500_list().await?;
    println!("✅ Fetched {} S&P 500 stocks\n", sp500_stocks.len());

    // Step 2: Analyze all S&P 500 stocks
    let stocks_to_analyze = &sp500_stocks;
    println!("🔍 Analyzing safety for ALL {} stocks...", stocks_to_analyze.len());
    println!("⏳ This will take several minutes...\n");

    let mut safe_stocks = Vec::new();
    let mut all_records = Vec::new();

    for (index, stock) in stocks_to_analyze.iter().enumerate() {
        // Progress indicator every 50 stocks
        if index % 50 == 0 || index < 10 {
            println!("  [{}/{}] Analyzing {} ({})...", 
                     index + 1, stocks_to_analyze.len(), stock.symbol, stock.name);
        }

        // Fetch detailed data for the stock
        match fetcher.fetch_complete_stock_data(&stock.symbol).await {
            Ok((company_data, financial_data, price_data)) => {
                // Analyze safety
                let safety_analysis = calculator.analyze_stock_safety(
                    stock, 
                    &company_data, 
                    &financial_data, 
                    &price_data
                );

                // Create database record
                let record = calculator.create_invite_list_record(
                    stock,
                    &safety_analysis,
                    &company_data,
                    &financial_data,
                    &price_data,
                );

                all_records.push(record.clone());

                if safety_analysis.is_safe_to_trade {
                    safe_stocks.push(record.clone());
                }

                // Show progress for first 10 and every 50th
                if index % 50 == 0 || index < 10 {
                    let status = if safety_analysis.is_safe_to_trade { "✅ SAFE" } else { "❌ NOT SAFE" };
                    println!("    {} - Score: {:.2}, Risk: {}", 
                             status, safety_analysis.safety_score, safety_analysis.risk_level);
                }
            }
            Err(e) => {
                println!("    ⚠️  Failed to analyze {}: {}", stock.symbol, e);
            }
        }

        // Progress indicator every 50 stocks
        if index % 50 == 0 {
            println!("  📊 Progress: {}/{} stocks analyzed ({}%)", 
                     index + 1, stocks_to_analyze.len(), 
                     ((index + 1) * 100) / stocks_to_analyze.len());
        }

        // Small delay to respect rate limits
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    println!("\n📊 Analysis Summary:");
    println!("  Total analyzed: {}", all_records.len());
    println!("  Safe to trade: {}", safe_stocks.len());
    println!("  Safety rate: {:.1}%", 
             (safe_stocks.len() as f64 / all_records.len() as f64) * 100.0);

    // Step 3: Save to database in smart batches to avoid timeout
    println!("\n💾 Saving {} records to Supabase in smart batches...", all_records.len());
    
    // Connect to database
    let db_client = DatabaseClient::from_env()?;
    let storage = InviteListStorage::new(db_client);

    // Use smaller batch sizes to avoid timeouts
    const BATCH_SIZE: usize = 25; // Optimal batch size
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
                        println!("    🔄 Retrying batch {} in 3 seconds...", batch_num + 1);
                        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                    } else {
                        total_failed += chunk.len();
                        println!("    💥 Batch {} failed after {} attempts", batch_num + 1, max_retries);
                    }
                }
            }
        }

        // Delay between batches to avoid overwhelming Supabase
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
    
    println!("\n🎉 Smart Batch Save Complete!");
    println!("  ✅ Successfully saved: {} records", total_saved);
    println!("  ❌ Failed to save: {} records", total_failed);
    println!("  📊 Success rate: {:.1}%", 
             (total_saved as f64 / all_records.len() as f64) * 100.0);
    
    // Display safe stocks
    if !safe_stocks.is_empty() {
        println!("\n🎯 SAFE STOCKS TO TRADE:");
        println!("{:<8} {:<30} {:<12} {:<8} {:<12}", 
                 "Symbol", "Company", "Safety Score", "Risk", "Sector");
        println!("{}", "-".repeat(80));
        
        for record in &safe_stocks {
            println!("{:<8} {:<30} {:<12.2} {:<8} {:<12}", 
                     record.symbol,
                     record.company_name.chars().take(28).collect::<String>(),
                     record.safety_score.unwrap_or(0.0),
                     record.risk_level,
                     record.sector.as_deref().unwrap_or("N/A"));
        }
    }

    // Step 4: Display detailed analysis for a few stocks
    if !safe_stocks.is_empty() {
        println!("\n🔍 Detailed Analysis for Top Safe Stocks:");
        for (i, record) in safe_stocks.iter().take(3).enumerate() {
            println!("\n{}. {} ({})", i + 1, record.symbol, record.company_name);
            println!("   Safety Score: {:.2}", record.safety_score.unwrap_or(0.0));
            println!("   Risk Level: {}", record.risk_level);
            println!("   Volatility: {}", record.volatility_rating.as_deref().unwrap_or("N/A"));
            println!("   Liquidity: {}", record.liquidity_rating.as_deref().unwrap_or("N/A"));
            println!("   Sector: {}", record.sector.as_deref().unwrap_or("N/A"));
            
            if let Some(reasoning) = &record.safety_reasoning {
                println!("   Reasoning: {}", reasoning.lines().next().unwrap_or("N/A"));
            }
        }
    }

    println!("\n🎉 S&P 500 safety analysis completed successfully!");
    println!("💡 You can now query the 'invite_list' table in Supabase to see all results.");
    println!("💡 Use the 'safe_stocks' view to see only stocks that are safe to trade.");

    Ok(())
}

/// Display analysis results in a formatted way
fn display_analysis_results(records: &[InviteListRecord]) {
    println!("\n📊 Analysis Results:");
    println!("{:<8} {:<30} {:<12} {:<8} {:<12} {:<8}", 
             "Symbol", "Company", "Safety Score", "Risk", "Sector", "Safe?");
    println!("{}", "-".repeat(90));
    
    for record in records {
        let safe_indicator = if record.is_safe_to_trade { "✅" } else { "❌" };
        println!("{:<8} {:<30} {:<12.2} {:<8} {:<12} {:<8}", 
                 record.symbol,
                 record.company_name.chars().take(28).collect::<String>(),
                 record.safety_score.unwrap_or(0.0),
                 record.risk_level,
                 record.sector.as_deref().unwrap_or("N/A"),
                 safe_indicator);
    }
}

/// Display sector analysis
fn display_sector_analysis(records: &[InviteListRecord]) {
    use std::collections::HashMap;
    
    let mut sector_stats: HashMap<String, (usize, usize)> = HashMap::new();
    
    for record in records {
        if let Some(sector) = &record.sector {
            let entry = sector_stats.entry(sector.clone()).or_insert((0, 0));
            entry.0 += 1;
            if record.is_safe_to_trade {
                entry.1 += 1;
            }
        }
    }
    
    println!("\n📈 Sector Analysis:");
    println!("{:<20} {:<8} {:<8} {:<8}", "Sector", "Total", "Safe", "Rate");
    println!("{}", "-".repeat(50));
    
    let mut sectors: Vec<_> = sector_stats.iter().collect();
    sectors.sort_by(|a, b| b.1.1.cmp(&a.1.1)); // Sort by safe count
    
    for (sector, (total, safe)) in sectors {
        let rate = (*safe as f64 / *total as f64) * 100.0;
        println!("{:<20} {:<8} {:<8} {:<8.1}%", sector, total, safe, rate);
    }
}
