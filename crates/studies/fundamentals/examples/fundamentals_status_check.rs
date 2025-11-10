// Check the status of fundamentals analysis in the database
// This utility helps you see what stocks have been analyzed and their results

use buenotea_database::fundamentals_storage::FundamentalsStorage;
use buenotea_database::invite_list_storage::InviteListStorage;
use buenotea_error::Result;
use tracing::info;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("📊 Checking fundamentals analysis status");
    
    // Load environment variables
    dotenv::dotenv().ok();

    // Create storage handlers
    let invite_list_storage = InviteListStorage::from_env()?;
    let fundamentals_storage = FundamentalsStorage::from_env()?;
    
    // Test database connections
    fundamentals_storage.test_connection().await?;

    // Get all stocks from invite_list
    info!("📋 Fetching all stocks from invite_list table...");
    let all_stocks = invite_list_storage.get_all_stock_symbols().await?;
    info!("✅ Found {} stocks in invite_list", all_stocks.len());

    // Get all fundamentals records
    info!("📊 Fetching fundamentals analysis records...");
    let fundamentals_records = fundamentals_storage.get_all_latest_fundamentals().await?;
    info!("✅ Found {} fundamentals records", fundamentals_records.len());

    // Create a map of analyzed stocks
    let mut analyzed_stocks: HashMap<String, &sentiment_backend::database::fundamentals_models::FundamentalsRecord> = HashMap::new();
    for record in &fundamentals_records {
        analyzed_stocks.insert(record.symbol.clone(), record);
    }

    // Analyze status
    let mut analyzed_count = 0;
    let mut pending_count = 0;
    let mut trading_signals: HashMap<String, usize> = HashMap::new();

    println!("\n{}", "=".repeat(80));
    println!("📊 FUNDAMENTALS ANALYSIS STATUS REPORT");
    println!("{}", "=".repeat(80));

    println!("📈 Total Stocks in Invite List: {}", all_stocks.len());
    println!("📊 Total Fundamentals Records: {}", fundamentals_records.len());

    // Check each stock
    println!("\n📋 ANALYSIS STATUS BY STOCK:");
    println!("{:<8} {:<12} {:<15} {:<10} {:<20}", "Status", "Symbol", "Score", "Signal", "Last Updated");
    println!("{}", "-".repeat(80));

    for stock in &all_stocks {
        if let Some(record) = analyzed_stocks.get(stock) {
            analyzed_count += 1;
            
            // Count trading signals
            let signal = &record.trading_signal;
            *trading_signals.entry(signal.clone()).or_insert(0) += 1;
            
            // Format last updated time
            let last_updated = record.analysis_date.format("%Y-%m-%d");
            
            println!(
                "{:<8} {:<12} {:<15.3} {:<10} {:<20}",
                "✅ DONE",
                stock,
                record.fundamentals_score,
                signal,
                last_updated
            );
        } else {
            pending_count += 1;
            println!(
                "{:<8} {:<12} {:<15} {:<10} {:<20}",
                "⏳ PENDING",
                stock,
                "N/A",
                "N/A",
                "N/A"
            );
        }
    }

    // Summary statistics
    println!("\n📊 SUMMARY STATISTICS:");
    println!("{}", "-".repeat(40));
    println!("✅ Analyzed: {}", analyzed_count);
    println!("⏳ Pending: {}", pending_count);
    
    if all_stocks.len() > 0 {
        let completion_rate = (analyzed_count as f64 / all_stocks.len() as f64) * 100.0;
        println!("📈 Completion Rate: {:.1}%", completion_rate);
    }

    // Trading signal distribution
    if !trading_signals.is_empty() {
        println!("\n🎯 TRADING SIGNAL DISTRIBUTION:");
        println!("{}", "-".repeat(40));
        for (signal, count) in &trading_signals {
            let percentage = (*count as f64 / analyzed_count as f64) * 100.0;
            let emoji = match signal.as_str() {
                "StrongBuy" => "🟢",
                "WeakBuy" => "🟡",
                "Hold" => "⚪",
                "WeakSell" => "🟠",
                "StrongSell" => "🔴",
                _ => "❓",
            };
            println!("{} {}: {} ({:.1}%)", emoji, signal, count, percentage);
        }
    }

    // Get fundamentals statistics
    if !fundamentals_records.is_empty() {
        println!("\n📊 FUNDAMENTALS STATISTICS:");
        println!("{}", "-".repeat(40));
        
        let stats = fundamentals_storage.get_fundamentals_stats().await?;
        println!("📈 Average Score: {:.3}", stats["average_score"]);
        println!("📊 Data Freshness: {:.1}%", stats["data_freshness"]);
        println!("🔢 Total Records: {}", stats["total_records"]);
        println!("🏢 Unique Symbols: {}", stats["unique_symbols"]);
    }

    // Recommendations
    println!("\n💡 RECOMMENDATIONS:");
    println!("{}", "-".repeat(40));
    
    if pending_count > 0 {
        println!("🔄 Run batch fundamentals analysis to process {} pending stocks", pending_count);
        println!("   Command: ./run_fundamentals_batch_analysis.sh");
    }
    
    if analyzed_count > 0 {
        println!("📊 All analyzed stocks have been saved to the fundamentals table");
        println!("🔍 Check your Supabase dashboard to view detailed results");
    }
    
    if analyzed_count == 0 && pending_count > 0 {
        println!("⚠️  No stocks have been analyzed yet. Run the batch analysis first.");
    }

    println!("\n{}", "=".repeat(80));
    info!("📊 Status check completed successfully!");
    
    Ok(())
}
