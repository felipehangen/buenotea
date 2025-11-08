// Regime Analysis to Supabase - Complete Implementation
// This script runs regime analysis with TTS scoring and saves results to Supabase

use sentiment_backend::regime::{RegimeCalculator, TTSResult};
use sentiment_backend::database::{DatabaseClient, RegimeStorage};
use sentiment_backend::database::regime_models::{RegimeRecord, ChatGPTAnalysis};
use sentiment_backend::ai::ChatGPTService;
use sentiment_backend::error::Result;
use dotenv::dotenv;
use tokio;
use serde_json;
use std::env;
use tracing::{error, warn};

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    println!("🚀 Starting Regime Analysis with TTS scoring and Supabase storage...\n");

    // Check for required environment variables
    let supabase_url = env::var("SUPABASE_URL")
        .map_err(|_| sentiment_backend::error::Error::ValidationError {
            message: "SUPABASE_URL environment variable not set".to_string(),
        })?;
    
    let supabase_api_key = env::var("SUPABASE_API_KEY")
        .map_err(|_| sentiment_backend::error::Error::ValidationError {
            message: "SUPABASE_API_KEY environment variable not set".to_string(),
        })?;

    println!("✅ Supabase configuration found");
    println!("   URL: {}", supabase_url);
    println!("   Key: {}...", &supabase_api_key[..20]);

    // Initialize database client
    println!("\n💾 Connecting to Supabase database...");
    let db_client = DatabaseClient::from_env()?;
    
    // Test database connection
    db_client.test_connection().await?;
    println!("✅ Database connection successful");

    // Initialize regime storage
    let regime_storage = RegimeStorage::new(db_client);

    // Initialize regime calculator
    let mut regime_calculator = RegimeCalculator::new();
    
    // Initialize ChatGPT service for AI analysis
    let chatgpt_service = ChatGPTService::default();

    // Get symbol from command line args or use default
    let symbol = env::args()
        .nth(1)
        .unwrap_or_else(|| "AAPL".to_string());

    println!("\n📊 Running regime analysis for symbol: {}", symbol);

    // Step 1: Run regime analysis with TTS scoring
    println!("🔍 Calculating TTS with regime analysis...");
    let start_time = std::time::Instant::now();
    let tts_result = match regime_calculator.calculate_tts_with_regime(&symbol).await {
        Ok(result) => {
            let computation_time = start_time.elapsed().as_millis() as i64;
            println!("✅ TTS analysis completed successfully in {}ms", computation_time);
            result
        }
        Err(e) => {
            error!("❌ TTS analysis failed: {}", e);
            return Err(e);
        }
    };

    // Display the results
    display_tts_results(&tts_result);

    // Step 2: Generate ChatGPT analysis
    println!("\n🤖 Generating ChatGPT analysis...");
    let chatgpt_analysis = match chatgpt_service.generate_regime_analysis(&tts_result).await {
        Ok(analysis) => {
            println!("✅ ChatGPT analysis completed");
            Some(analysis)
        }
        Err(e) => {
            warn!("⚠️  ChatGPT analysis failed: {}", e);
            warn!("   Continuing without AI analysis...");
            None
        }
    };

    if let Some(ref analysis) = chatgpt_analysis {
        display_chatgpt_analysis(analysis);
    }

    // Step 3: Create database record
    println!("\n📝 Preparing database record...");
    
    // Create API tracking information
    let computation_time = start_time.elapsed().as_millis() as i64;
    let api_tracking = sentiment_backend::regime::RegimeApiTracking {
        primary_api_source: "FMP".to_string(), // This would be populated from the calculator
        fallback_api_source: Some("Alpha Vantage".to_string()),
        api_endpoints_used: vec![
            format!("https://financialmodelingprep.com/api/v3/historical-price-full/{}", symbol),
            "https://financialmodelingprep.com/api/v3/historical-price-full/SPY".to_string(),
        ],
        price_data_points: 250, // Updated to match new data collection
        market_data_points: 250,
        analysis_period_days: 250,
        current_price: tts_result.technical_indicators.sma_20.unwrap_or(0.0),
        market_regime_confidence: tts_result.confidence_score,
    };

    // Create regime record from TTS result
    let regime_record = RegimeRecord::from_tts_result(&tts_result, &api_tracking, chatgpt_analysis.clone(), Some(computation_time));

    // Step 4: Save to Supabase
    println!("\n💾 Saving to Supabase regime table...");
    let record_id = match regime_storage.store_regime_analysis(&regime_record).await {
        Ok(id) => {
            println!("✅ Successfully saved to database with ID: {}", id);
            id
        }
        Err(e) => {
            error!("❌ Failed to save to database: {}", e);
            return Err(e);
        }
    };

    // Step 5: Display summary
    println!("\n📊 Analysis Summary:");
    println!("   Symbol: {}", regime_record.symbol);
    println!("   TTS Score: {:.3}", regime_record.tts_score);
    println!("   Market Regime: {}", regime_record.market_regime);
    println!("   Trading Signal: {}", regime_record.trading_signal);
    println!("   Confidence: {:.1}%", regime_record.confidence_score * 100.0);
    println!("   Risk Level: {}", regime_record.risk_level);
    println!("   Position Size: {:.1}%", regime_record.position_size * 100.0);
    println!("   Database ID: {}", record_id);
    println!("   ChatGPT Analysis: {}", if regime_record.chatgpt_trading_recommendation.is_some() { "✅ Included" } else { "❌ Not available" });

    // Step 6: Save results to JSON file for reference
    save_results_to_json(&tts_result, &regime_record, chatgpt_analysis.as_ref())?;
    
    println!("\n📁 Results saved to JSON files");
    println!("🎉 Regime analysis successfully completed and stored in Supabase!");
    
    Ok(())
}

/// Display detailed TTS analysis results
fn display_tts_results(result: &TTSResult) {
    println!("\n📈 TTS Analysis Results for {}", result.symbol);
    println!("═══════════════════════════════════════════════════════════════");
    
    // Main score and signal
    println!("🎯 Core Analysis:");
    println!("   TTS Score: {:.3} (Range: -1.0 to +1.0)", result.tts_score);
    println!("   Trading Signal: {} {}", result.trading_signal.emoji(), result.trading_signal.description());
    println!("   Position Size: {:.1}%", result.trading_signal.position_size() * 100.0);
    
    // Market regime
    println!("\n🌍 Market Regime:");
    println!("   Current Regime: {} {}", result.market_regime.emoji(), result.market_regime.description());
    println!("   Regime Multiplier: {:.2}x", result.market_regime.tts_multiplier());
    
    // Component scores
    println!("\n📊 Component Scores:");
    println!("   Momentum Score: {:.3} (30% weight)", result.components.momentum_score);
    println!("   Volatility Score: {:.3} (25% weight)", result.components.volatility_score);
    println!("   Volume Score: {:.3} (20% weight)", result.components.volume_score);
    println!("   Support/Resistance Score: {:.3} (15% weight)", result.components.support_resistance_score);
    println!("   Market Correlation Score: {:.3} (10% weight)", result.components.market_correlation_score);
    
    // Risk assessment
    println!("\n⚠️  Risk Assessment:");
    println!("   Risk Level: {:?}", result.risk_assessment.risk_level);
    println!("   Volatility Score: {:.1}%", result.risk_assessment.volatility_score);
    println!("   Max Drawdown Risk: {:.1}%", result.risk_assessment.max_drawdown_risk);
    if let Some(stop_loss) = result.risk_assessment.stop_loss {
        println!("   Stop Loss Recommendation: ${:.2}", stop_loss);
    }
    println!("   Position Size: {:.1}%", result.risk_assessment.position_size * 100.0);
    
    // Technical indicators summary
    println!("\n🔧 Key Technical Indicators:");
    if let Some(rsi) = result.technical_indicators.rsi_14 {
        println!("   RSI (14): {:.1}", rsi);
    }
    if let Some(sma_20) = result.technical_indicators.sma_20 {
        println!("   SMA (20): ${:.2}", sma_20);
    }
    if let Some(sma_50) = result.technical_indicators.sma_50 {
        println!("   SMA (50): ${:.2}", sma_50);
    }
    
    // Flags
    if !result.flags.is_empty() {
        println!("\n🚩 Warning Flags:");
        for flag in &result.flags {
            println!("   ⚠️  {}", flag);
        }
    }
    
    println!("═══════════════════════════════════════════════════════════════");
}

/// Display ChatGPT analysis results
fn display_chatgpt_analysis(analysis: &ChatGPTAnalysis) {
    println!("\n🤖 ChatGPT Analysis Results");
    println!("═══════════════════════════════════════════════════════════════");
    
    println!("📊 Regime Analysis:");
    println!("   {}", analysis.regime_analysis);
    
    println!("\n🎯 TTS Interpretation:");
    println!("   {}", analysis.tts_interpretation);
    
    println!("\n💡 Trading Recommendation:");
    println!("   {}", analysis.trading_recommendation);
    
    println!("\n📋 Analysis Metadata:");
    println!("   Model Used: {}", analysis.model_used);
    println!("   Timestamp: {}", analysis.analysis_timestamp.format("%Y-%m-%d %H:%M:%S UTC"));
    
    println!("═══════════════════════════════════════════════════════════════");
}

/// Save the TTS analysis results to JSON files for reference
fn save_results_to_json(
    result: &TTSResult, 
    regime_record: &RegimeRecord,
    chatgpt_analysis: Option<&ChatGPTAnalysis>
) -> Result<()> {
    // Create a timestamped filename
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    
    // Save TTS results
    let tts_filename = format!("{}_tts_regime_analysis_{}.json", result.symbol.to_lowercase(), timestamp);
    let tts_json = serde_json::to_string_pretty(result)?;
    std::fs::write(&tts_filename, tts_json).map_err(|e| sentiment_backend::error::Error::ValidationError {
        message: format!("Failed to write TTS JSON file: {}", e)
    })?;
    
    // Save regime record
    let regime_filename = format!("{}_regime_record_{}.json", result.symbol.to_lowercase(), timestamp);
    let regime_json = serde_json::to_string_pretty(regime_record)?;
    std::fs::write(&regime_filename, regime_json).map_err(|e| sentiment_backend::error::Error::ValidationError {
        message: format!("Failed to write regime record JSON file: {}", e)
    })?;
    
    // Save ChatGPT analysis if available
    if let Some(analysis) = chatgpt_analysis {
        let chatgpt_filename = format!("{}_chatgpt_analysis_{}.json", result.symbol.to_lowercase(), timestamp);
        let chatgpt_json = serde_json::to_string_pretty(analysis)?;
        std::fs::write(&chatgpt_filename, chatgpt_json).map_err(|e| sentiment_backend::error::Error::ValidationError {
            message: format!("Failed to write ChatGPT JSON file: {}", e)
        })?;
        println!("💾 ChatGPT analysis saved to: {}", chatgpt_filename);
    }
    
    println!("💾 TTS results saved to: {}", tts_filename);
    println!("💾 Regime record saved to: {}", regime_filename);
    
    Ok(())
}
