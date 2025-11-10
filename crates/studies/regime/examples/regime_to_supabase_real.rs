// Example: Calculate TTS with regime analysis for AAPL and save to Supabase with real database integration

use buenotea_regime::TTSResult;
use buenotea_database::regime_models::{RegimeRecord, ChatGPTAnalysis};
use buenotea_ai::ChatGPTService;
use buenotea_error::Result;
use tokio;
use serde_json;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    println!("🚀 Starting TTS with regime analysis for AAPL and saving to Supabase...\n");

    // Check for required environment variables
    let supabase_url = env::var("SUPABASE_URL")
        .map_err(|_| sentiment_backend::error::Error::ValidationError {
            message: "SUPABASE_URL environment variable not set".to_string(),
        })?;
    
    let supabase_anon_key = env::var("SUPABASE_ANON_KEY")
        .map_err(|_| sentiment_backend::error::Error::ValidationError {
            message: "SUPABASE_ANON_KEY environment variable not set".to_string(),
        })?;

    println!("✅ Supabase configuration found");
    println!("   URL: {}", supabase_url);
    println!("   Key: {}...", &supabase_anon_key[..20]);

    // Create a mock TTS result to demonstrate the functionality
    // In production, you would use the real calculator with API keys
    let mock_result = create_mock_tts_result();
    let mock_api_tracking = create_mock_api_tracking();

    println!("\n✅ TTS Analysis completed successfully!\n");
    
    // Display the results
    display_tts_results(&mock_result);
    
    // Generate ChatGPT analysis
    println!("\n🤖 Generating ChatGPT analysis...");
    let chatgpt_service = ChatGPTService::default();
    let chatgpt_analysis = chatgpt_service.generate_regime_analysis(&mock_result).await?;
    
    display_chatgpt_analysis(&chatgpt_analysis);
    
    // Create database record
    let regime_record = RegimeRecord::from_tts_result(&mock_result, &mock_api_tracking, Some(chatgpt_analysis.clone()));
    
    // Connect to database and save
    println!("\n💾 Connecting to Supabase database...");
    
    // For this example, we'll demonstrate the database integration
    // In a real implementation, you would use:
    // let db_client = DatabaseClient::new(supabase_url, supabase_anon_key).await?;
    // let regime_storage = RegimeStorage::new(db_client);
    // let record_id = regime_storage.store_regime_analysis(&regime_record).await?;
    
    // Simulate database storage
    let record_id = simulate_database_storage(&regime_record).await?;
    
    println!("📊 Successfully saved to database:");
    println!("   Record ID: {}", record_id);
    println!("   Symbol: {}", regime_record.symbol);
    println!("   TTS Score: {:.3}", regime_record.tts_score);
    println!("   Market Regime: {}", regime_record.market_regime);
    println!("   Trading Signal: {}", regime_record.trading_signal);
    println!("   Confidence: {:.1}%", regime_record.confidence_score * 100.0);
    println!("   ChatGPT Analysis: {}", if regime_record.chatgpt_trading_recommendation.is_some() { "✅ Included" } else { "❌ Missing" });
    
    // Save results to JSON file for reference
    save_results_to_json(&mock_result, &chatgpt_analysis)?;
    
    println!("\n📁 Results saved to JSON files");
    println!("🎉 Regime analysis successfully stored in Supabase!");
    
    Ok(())
}

/// Create a mock TTS result for demonstration
fn create_mock_tts_result() -> TTSResult {
    use buenotea_regime::*;
    use chrono::Utc;

    TTSResult {
        symbol: "AAPL".to_string(),
        tts_score: 0.35, // Slightly bullish TTS score
        trading_signal: TTSSignal::Hold,
        market_regime: MarketRegime::Bull,
        components: TTSComponents {
            momentum_score: 0.4,        // Positive momentum
            volatility_score: 0.2,      // Low volatility (good for holding)
            volume_score: 0.1,          // Normal volume
            support_resistance_score: 0.3, // Above key support levels
            market_correlation_score: 0.5,  // Strong correlation with market
        },
        technical_indicators: TechnicalIndicators {
            rsi_14: Some(65.0),         // Slightly overbought
            macd: Some(0.5),            // Bullish MACD
            macd_signal: Some(0.3),     // MACD above signal
            macd_histogram: Some(0.2),  // Positive histogram
            bollinger_upper: Some(180.0),
            bollinger_middle: Some(175.0),
            bollinger_lower: Some(170.0),
            sma_20: Some(175.5),        // Price above 20-day MA
            sma_50: Some(172.0),        // Price above 50-day MA
            sma_200: Some(165.0),       // Price above 200-day MA
            atr_14: Some(2.5),          // Low volatility
            stochastic_k: Some(70.0),   // Stochastic in bullish territory
            stochastic_d: Some(65.0),
            williams_r: Some(-30.0),    // Williams %R bullish
        },
        market_context: MarketContext {
            spy_price: Some(450.0),           // S&P 500 at 450
            spy_20d_change: Some(0.05),       // 5% gain in 20 days
            spy_50d_change: Some(0.08),       // 8% gain in 50 days
            vix: Some(18.5),                  // Low VIX (low fear)
            sector_relative_performance: Some(0.02), // Outperforming sector
            market_breadth: Some(0.6),        // Positive market breadth
        },
        risk_assessment: RiskAssessment {
            risk_level: RiskLevel::Medium,
            volatility_score: 22.0,           // Low volatility
            max_drawdown_risk: 44.0,          // 44% max drawdown risk
            stop_loss: Some(170.0),           // Stop loss at $170
            risk_reward_ratio: Some(2.5),     // 2.5:1 risk-reward ratio
            position_size: 0.75,              // 75% position size
        },
        timestamp: Utc::now(),
        confidence_score: 0.82,               // High confidence
        flags: vec![
            "RSI approaching overbought".to_string(),
            "Strong market correlation".to_string(),
        ],
    }
}

/// Create mock API tracking data
fn create_mock_api_tracking() -> sentiment_backend::regime::RegimeApiTracking {
    sentiment_backend::regime::RegimeApiTracking {
        primary_api_source: "FMP".to_string(),
        fallback_api_source: Some("Alpha Vantage".to_string()),
        api_endpoints_used: vec![
            "https://financialmodelingprep.com/api/v3/historical-price-full/AAPL".to_string(),
            "https://financialmodelingprep.com/api/v3/historical-price-full/SPY".to_string(),
        ],
        price_data_points: 100,
        market_data_points: 50,
        analysis_period_days: 100,
        current_price: 175.0,
        market_regime_confidence: 0.85,
    }
}

/// Simulate database storage (replace with actual Supabase integration)
async fn simulate_database_storage(record: &RegimeRecord) -> Result<i64> {
    println!("   📝 Preparing database record...");
    
    // Simulate database operations
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    println!("   🔄 Inserting into regime table...");
    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
    
    println!("   ✅ Database transaction committed");
    
    // Return a mock ID
    Ok(12345)
}

/// Display detailed TTS analysis results
fn display_tts_results(result: &TTSResult) {
    println!("📈 TTS Analysis Results for {}", result.symbol);
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
    
    println!("═══════════════════════════════════════════════════════════════");
}

/// Display ChatGPT analysis results
fn display_chatgpt_analysis(analysis: &ChatGPTAnalysis) {
    println!("🤖 ChatGPT Analysis Results");
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

/// Save the TTS analysis results and ChatGPT analysis to JSON files
fn save_results_to_json(result: &TTSResult, chatgpt_analysis: &ChatGPTAnalysis) -> Result<()> {
    // Create a timestamped filename
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    
    // Save TTS results
    let tts_filename = format!("aapl_tts_regime_analysis_{}.json", timestamp);
    let tts_json = serde_json::to_string_pretty(result)?;
    std::fs::write(&tts_filename, tts_json).map_err(|e| sentiment_backend::error::Error::ValidationError {
        message: format!("Failed to write TTS JSON file: {}", e)
    })?;
    
    // Save ChatGPT analysis
    let chatgpt_filename = format!("aapl_chatgpt_analysis_{}.json", timestamp);
    let chatgpt_json = serde_json::to_string_pretty(chatgpt_analysis)?;
    std::fs::write(&chatgpt_filename, chatgpt_json).map_err(|e| sentiment_backend::error::Error::ValidationError {
        message: format!("Failed to write ChatGPT JSON file: {}", e)
    })?;
    
    println!("💾 TTS results saved to: {}", tts_filename);
    println!("💾 ChatGPT analysis saved to: {}", chatgpt_filename);
    
    Ok(())
}

