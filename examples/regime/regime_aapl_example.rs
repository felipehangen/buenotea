// Example: Calculate TTS (Time To Sell) score with market regime analysis for AAPL
// This example demonstrates the regime analysis module functionality

use sentiment_backend::regime::{RegimeCalculator, TTSResult};
use sentiment_backend::error::Result;
use tokio;
use serde_json;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    println!("🚀 Starting TTS (Time To Sell) analysis for AAPL with market regime detection...\n");

    // Create a new regime calculator
    let mut calculator = RegimeCalculator::new();

    // Calculate TTS score with market regime analysis for AAPL
    match calculator.calculate_tts_with_regime("AAPL").await {
        Ok(result) => {
            println!("✅ TTS Analysis completed successfully!\n");
            
            // Display the results
            display_tts_results(&result);
            
            // Save results to JSON file
            save_results_to_json(&result)?;
            
            println!("\n📊 Analysis Summary:");
            println!("   Symbol: {}", result.symbol);
            println!("   TTS Score: {:.3}", result.tts_score);
            println!("   Signal: {} {}", result.trading_signal.emoji(), result.trading_signal.description());
            println!("   Market Regime: {} {}", result.market_regime.emoji(), result.market_regime.description());
            println!("   Confidence: {:.1}%", result.confidence_score * 100.0);
            println!("   Risk Level: {:?}", result.risk_assessment.risk_level);
            
            if !result.flags.is_empty() {
                println!("\n⚠️  Flags:");
                for flag in &result.flags {
                    println!("   • {}", flag);
                }
            }
            
            println!("\n📁 Results saved to: aapl_tts_regime_analysis.json");
            
        }
        Err(e) => {
            eprintln!("❌ Error calculating TTS analysis: {:?}", e);
            return Err(e);
        }
    }

    Ok(())
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
    
    // Technical indicators
    println!("\n🔧 Technical Indicators:");
    if let Some(rsi) = result.technical_indicators.rsi_14 {
        println!("   RSI (14-day): {:.1}", rsi);
    }
    if let Some(macd) = result.technical_indicators.macd {
        println!("   MACD: {:.3}", macd);
    }
    if let Some(macd_signal) = result.technical_indicators.macd_signal {
        println!("   MACD Signal: {:.3}", macd_signal);
    }
    if let Some(sma_20) = result.technical_indicators.sma_20 {
        println!("   SMA (20-day): ${:.2}", sma_20);
    }
    if let Some(sma_50) = result.technical_indicators.sma_50 {
        println!("   SMA (50-day): ${:.2}", sma_50);
    }
    if let Some(atr) = result.technical_indicators.atr_14 {
        println!("   ATR (14-day): ${:.2}", atr);
    }
    
    // Market context
    println!("\n🏛️  Market Context:");
    if let Some(spy_price) = result.market_context.spy_price {
        println!("   SPY Price: ${:.2}", spy_price);
    }
    if let Some(spy_20d_change) = result.market_context.spy_20d_change {
        println!("   SPY 20-day Change: {:.2}%", spy_20d_change * 100.0);
    }
    if let Some(vix) = result.market_context.vix {
        println!("   VIX: {:.1}", vix);
    }
    
    // Risk assessment
    println!("\n⚠️  Risk Assessment:");
    println!("   Risk Level: {:?}", result.risk_assessment.risk_level);
    println!("   Volatility Score: {:.1}%", result.risk_assessment.volatility_score);
    println!("   Max Drawdown Risk: {:.1}%", result.risk_assessment.max_drawdown_risk);
    if let Some(stop_loss) = result.risk_assessment.stop_loss {
        println!("   Stop Loss Recommendation: ${:.2}", stop_loss);
    }
    println!("   Position Size: {:.1}%", result.risk_assessment.position_size * 100.0);
    
    // Analysis metadata
    println!("\n📋 Analysis Metadata:");
    println!("   Timestamp: {}", result.timestamp.format("%Y-%m-%d %H:%M:%S UTC"));
    println!("   Confidence Score: {:.1}%", result.confidence_score * 100.0);
    println!("   Valid Components: {}/5", result.components.valid_components_count());
    
    if !result.flags.is_empty() {
        println!("\n🚩 Warning Flags:");
        for (i, flag) in result.flags.iter().enumerate() {
            println!("   {}. {}", i + 1, flag);
        }
    }
    
    println!("═══════════════════════════════════════════════════════════════");
}

/// Save the TTS analysis results to a JSON file
fn save_results_to_json(result: &TTSResult) -> Result<()> {
    // Create a timestamped filename
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("aapl_tts_regime_analysis_{}.json", timestamp);
    
    // Serialize the result to pretty-printed JSON
    let json = serde_json::to_string_pretty(result)?;
    
    // Write to file
    std::fs::write(&filename, json).map_err(|e| sentiment_backend::error::Error::ValidationError {
        message: format!("Failed to write JSON file: {}", e)
    })?;
    
    println!("💾 Results saved to: {}", filename);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tts_result_serialization() {
        // Test that TTSResult can be serialized to JSON
        let result = create_mock_tts_result();
        let json = serde_json::to_string(&result).expect("Should serialize to JSON");
        assert!(!json.is_empty());
        
        // Test deserialization
        let deserialized: TTSResult = serde_json::from_str(&json).expect("Should deserialize from JSON");
        assert_eq!(result.symbol, deserialized.symbol);
        assert_eq!(result.tts_score, deserialized.tts_score);
    }

    fn create_mock_tts_result() -> TTSResult {
        use sentiment_backend::regime::*;
        use chrono::Utc;

        TTSResult {
            symbol: "AAPL".to_string(),
            tts_score: 0.3,
            trading_signal: TTSSignal::Hold,
            market_regime: MarketRegime::Bull,
            components: TTSComponents {
                momentum_score: 0.4,
                volatility_score: 0.2,
                volume_score: 0.1,
                support_resistance_score: 0.3,
                market_correlation_score: 0.5,
            },
            technical_indicators: TechnicalIndicators {
                rsi_14: Some(65.0),
                macd: Some(0.5),
                macd_signal: Some(0.3),
                macd_histogram: Some(0.2),
                bollinger_upper: Some(180.0),
                bollinger_middle: Some(175.0),
                bollinger_lower: Some(170.0),
                sma_20: Some(175.5),
                sma_50: Some(172.0),
                sma_200: Some(165.0),
                atr_14: Some(2.5),
                stochastic_k: Some(70.0),
                stochastic_d: Some(65.0),
                williams_r: Some(-30.0),
            },
            market_context: MarketContext {
                spy_price: Some(450.0),
                spy_20d_change: Some(0.05),
                spy_50d_change: Some(0.08),
                vix: Some(18.5),
                sector_relative_performance: Some(0.02),
                market_breadth: Some(0.6),
            },
            risk_assessment: RiskAssessment {
                risk_level: RiskLevel::Medium,
                volatility_score: 25.0,
                max_drawdown_risk: 50.0,
                stop_loss: Some(170.0),
                risk_reward_ratio: Some(2.0),
                position_size: 0.75,
            },
            timestamp: Utc::now(),
            confidence_score: 0.8,
            flags: vec!["RSI approaching overbought".to_string()],
        }
    }
}
