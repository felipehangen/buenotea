// Example: Run market regime analysis and save to Supabase
// This analyzes overall market conditions and saves to market_regime_history

use buenotea_regime::{MarketRegimeCalculator, MarketRegimeStorage, create_market_regime_record_with_tracking};
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

    println!("🚀 Starting Market Regime Analysis...\n");

    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Step 1: Calculate market regime
    println!("📈 Analyzing overall market conditions...");
    let mut calculator = MarketRegimeCalculator::new();
    
    let result = match calculator.calculate_market_regime().await {
        Ok(result) => {
            println!("✅ Market regime analysis completed!\n");
            result
        }
        Err(e) => {
            eprintln!("❌ Failed to calculate market regime: {}", e);
            return Err(e);
        }
    };

    // Step 2: Display results
    println!("📊 Market Regime Analysis Results:");
    println!("   Market Regime: {} {}", result.market_regime, result.market_regime.emoji());
    println!("   Confidence: {:.1}%", result.regime_confidence * 100.0);
    println!("   Risk Level: {:?}", result.risk_assessment.risk_level);
    println!("   Risk Score: {:.1}", result.risk_assessment.risk_score);
    
    if let Some(spy_price) = result.market_context.spy_price {
        println!("   SPY Price: ${:.2}", spy_price);
    }
    if let Some(vix) = result.market_context.vix {
        println!("   VIX: {:.2}", vix);
    }
    println!("   Trend Strength: {:.1}%", result.trend_analysis.strength);
    println!("   Trend Consistency: {:.1}%", result.trend_analysis.consistency);
    println!("   Market Volatility: {:.2}%", result.volatility_analysis.market_volatility);
    println!();

    // Step 3: Save to Supabase
    println!("💾 Saving to Supabase market_regime_history table...");
    let storage = MarketRegimeStorage::from_env()?;
    
    // Create database record (no ChatGPT analysis for now)
    let record = create_market_regime_record_with_tracking(result, None);
    
    match storage.store_market_regime_record(&record).await {
        Ok(id) => {
            println!("✅ Successfully saved to database with ID: {}", id);
            println!("\n📊 Summary:");
            println!("   Market Regime: {}", record.market_regime);
            println!("   Confidence: {:.1}%", record.regime_confidence * 100.0);
            println!("   Risk Level: {}", record.market_risk_level);
            println!("   Analysis Date: {}", record.analysis_date);
            println!("   Database ID: {}", id);
        }
        Err(e) => {
            eprintln!("❌ Failed to save to database: {}", e);
            return Err(e);
        }
    }

    println!("\n🎉 Market regime analysis successfully stored in Supabase!");
    Ok(())
}

