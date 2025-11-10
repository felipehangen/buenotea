// Example: Run fundamentals analysis on a single stock and save to Supabase
// This analyzes fundamental financial data and saves to fundamentals_history

use buenotea_fundamentals::{FundamentalsCalculator, FundamentalsStorage, FundamentalsApiUrls, create_fundamentals_record_with_tracking};
use buenotea_core::Result;
use buenotea_infrastructure::DatabaseClient;
use dotenv;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing for logging
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(tracing::Level::INFO)
            .finish()
    ).ok();

    println!("🚀 Starting Fundamentals Analysis (AAPL example)...\n");

    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Step 1: Run fundamentals analysis for AAPL
    println!("📈 Analyzing AAPL fundamentals...");
    let mut calculator = FundamentalsCalculator::new();
    let symbol = "AAPL";
    
    let result = calculator.calculate_fundamentals(symbol).await?;
    println!("✅ Fundamentals analysis completed!");
    println!("   Symbol: {}", result.symbol);
    println!("   Score: {:.2}", result.fundamentals_score);
    println!("   Signal: {}", result.trading_signal);
    println!("   Confidence: {:.1}%", result.confidence_score * 100.0);

    // Step 2: Prepare API URLs tracking (in a real scenario, you'd get this from the calculator)
    let api_urls = FundamentalsApiUrls::default(); // Using default for this example

    // Step 3: Create database record (no GPT analysis for now)
    let record_to_store = create_fundamentals_record_with_tracking(
        result.clone(),
        api_urls,
        None, // gpt_explanation
        None, // gpt_trading_suggestion
    );

    // Step 4: Save to Supabase
    println!("\n💾 Saving to Supabase fundamentals_history table...");
    let db_client = DatabaseClient::from_env()?;
    let fundamentals_storage = FundamentalsStorage::new(db_client);

    let record_id = match fundamentals_storage.store_fundamentals_record(&record_to_store).await {
        Ok(id) => {
            println!("✅ Successfully saved to database with ID: {}", id);
            id
        }
        Err(e) => {
            eprintln!("❌ Failed to save to database: {}", e);
            return Err(e);
        }
    };

    // Step 5: Display component scores
    println!("\n📊 Component Scores:");
    println!("   Profitability: {:.1}", result.components.profitability);
    println!("   Growth:        {:.1}", result.components.growth);
    println!("   Valuation:     {:.1}", result.components.valuation);
    println!("   Fin. Strength: {:.1}", result.components.financial_strength);
    println!("   Efficiency:    {:.1}", result.components.efficiency);

    // Step 6: Display key metrics
    println!("\n📈 Key Metrics:");
    if let Some(roe) = result.metrics.profitability.roe {
        println!("   ROE: {:.2}%", roe * 100.0);
    }
    if let Some(revenue_growth) = result.metrics.growth.revenue_growth_yoy {
        println!("   Revenue Growth YoY: {:.2}%", revenue_growth * 100.0);
    }
    if let Some(pe_ratio) = result.metrics.valuation.pe_ratio {
        println!("   P/E Ratio: {:.2}", pe_ratio);
    }

    // Step 7: Summary
    println!("\n✨ Summary:");
    println!("   Symbol: {}", result.symbol);
    println!("   Score: {:.2}", result.fundamentals_score);
    println!("   Signal: {}", result.trading_signal);
    println!("   Confidence: {:.1}%", result.confidence_score * 100.0);
    println!("   Database ID: {}", record_id);

    // Step 8: Display flags (if any)
    if !result.flags.is_empty() {
        println!("\n⚠️  Flags:");
        for flag in &result.flags {
            println!("   - {}", flag);
        }
    }

    println!("\n🎉 Fundamentals analysis successfully stored in Supabase!");

    Ok(())
}

