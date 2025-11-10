// Simple example: Analyze a few S&P 500 stocks for safety

use buenotea_invite_list::{InviteListCalculator, SP500Fetcher};
use buenotea_invite_list::models::{ApiConfig, SP500Stock};
use buenotea_core::Result;
use tokio;
use dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing for logging
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(tracing::Level::INFO)
            .finish()
    ).ok();

    println!("🚀 Starting simple S&P 500 safety analysis...\n");

    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Load API configuration from environment variables
    let api_config = ApiConfig::from_env()
        .map_err(|e| {
            eprintln!("❌ Failed to load API configuration: {}", e);
            eprintln!("Please ensure the following environment variables are set in .env file:");
            eprintln!("  - FMP_API_KEY");
            e
        })?;

    println!("✅ API configuration loaded successfully!\n");

    // Initialize components
    let fetcher = SP500Fetcher::new(api_config.clone());
    let calculator = InviteListCalculator::new(api_config.clone());

    // Test with a few well-known stocks
    let test_symbols = vec!["AAPL", "MSFT", "GOOGL", "AMZN", "TSLA"];
    
    println!("🔍 Analyzing safety for test stocks: {:?}", test_symbols);

    for symbol in test_symbols {
        println!("\n📊 Analyzing {}...", symbol);
        
        // Create a mock stock for testing
        let mock_stock = SP500Stock {
            symbol: symbol.to_string(),
            name: format!("{} Inc.", symbol),
            sector: Some("Technology".to_string()),
            industry: Some("Software".to_string()),
            market_cap: Some(1_000_000_000_000),
            current_price: Some(150.0),
        };

        // Fetch detailed data for the stock
        match fetcher.fetch_complete_stock_data(symbol).await {
            Ok((company_data, financial_data, price_data)) => {
                println!("  ✅ Data fetched successfully");
                
                // Analyze safety
                let safety_analysis = calculator.analyze_stock_safety(
                    &mock_stock, 
                    &company_data, 
                    &financial_data, 
                    &price_data
                );

                // Display results
                println!("  📈 Safety Analysis Results:");
                println!("    Safe to trade: {}", if safety_analysis.is_safe_to_trade { "✅ YES" } else { "❌ NO" });
                println!("    Safety Score: {:.2}/1.00", safety_analysis.safety_score);
                println!("    Risk Level: {}", safety_analysis.risk_level);
                println!("    Volatility: {}", safety_analysis.volatility_rating);
                println!("    Liquidity: {}", safety_analysis.liquidity_rating);
                
                println!("    📋 Health Checks:");
                println!("      Recent Earnings: {}", if safety_analysis.has_recent_earnings { "✅" } else { "❌" });
                println!("      Positive Revenue: {}", if safety_analysis.has_positive_revenue { "✅" } else { "❌" });
                println!("      Stable Price: {}", if safety_analysis.has_stable_price { "✅" } else { "❌" });
                println!("      Sufficient Volume: {}", if safety_analysis.has_sufficient_volume { "✅" } else { "❌" });
                println!("      Analyst Coverage: {}", if safety_analysis.has_analyst_coverage { "✅" } else { "❌" });
                
                if !safety_analysis.warning_flags.is_empty() {
                    println!("    ⚠️  Warnings: {:?}", safety_analysis.warning_flags);
                }
                
                if !safety_analysis.missing_data_components.is_empty() {
                    println!("    📝 Missing Data: {:?}", safety_analysis.missing_data_components);
                }
                
                println!("    💭 Reasoning:");
                for line in safety_analysis.safety_reasoning.lines() {
                    println!("      {}", line);
                }
            }
            Err(e) => {
                println!("  ❌ Failed to fetch data: {}", e);
            }
        }

        // Small delay to respect rate limits
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    }

    println!("\n🎉 Analysis completed!");
    println!("💡 This example demonstrates the safety analysis process.");
    println!("💡 Run 'invite_list_to_supabase' to save results to database.");

    Ok(())
}
