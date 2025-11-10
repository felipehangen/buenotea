// Market Regime Analysis Example
// Analyzes the overall market regime - "What's the vibe of the whole club?"

use buenotea_regime::{MarketRegimeCalculator, MarketRegimeResult};
use buenotea_error::Result;
use tokio;
use serde_json;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    println!("🌍 Starting Market Regime Analysis...");
    println!("   Analyzing the overall market mood - \"What's the vibe of the whole club?\"\n");

    // Create a new market regime calculator
    let mut calculator = MarketRegimeCalculator::new();

    // Calculate overall market regime
    match calculator.calculate_market_regime().await {
        Ok(result) => {
            println!("✅ Market Regime Analysis completed successfully!\n");
            
            // Display the results
            display_market_regime_results(&result);
            
            // Save results to JSON file
            save_results_to_json(&result)?;
            
            println!("\n📊 Market Regime Summary:");
            println!("   Market Regime: {} {}", result.market_regime.emoji(), result.market_regime.description());
            println!("   Confidence: {:.1}%", result.regime_confidence * 100.0);
            println!("   Risk Level: {}", result.risk_assessment.risk_level);
            println!("   SPY Price: ${:.2}", result.market_context.spy_price.unwrap_or(0.0));
            println!("   VIX: {:.1}", result.market_context.vix.unwrap_or(0.0));
            println!("   Market Breadth: {:.1}%", result.market_context.market_breadth.unwrap_or(0.0) * 100.0);
            
            println!("\n📁 Results saved to: market_regime_analysis.json");
            
        }
        Err(e) => {
            eprintln!("❌ Error calculating market regime analysis: {:?}", e);
            return Err(e);
        }
    }

    Ok(())
}

/// Display detailed market regime analysis results
fn display_market_regime_results(result: &MarketRegimeResult) {
    println!("🌍 Market Regime Analysis Results");
    println!("═══════════════════════════════════════════════════════════════");
    
    // Main regime classification
    println!("🎯 Market Regime Classification:");
    println!("   Current Regime: {} {}", result.market_regime.emoji(), result.market_regime.description());
    println!("   Confidence: {:.1}%", result.regime_confidence * 100.0);
    println!("   Stock Analysis Multiplier: {:.2}x", result.market_regime.stock_analysis_multiplier());
    
    // Market context
    println!("\n📊 Market Context:");
    if let Some(spy_price) = result.market_context.spy_price {
        println!("   SPY Price: ${:.2}", spy_price);
    }
    if let Some(spy_20d_change) = result.market_context.spy_20d_change {
        println!("   SPY 20-day Change: {:.2}%", spy_20d_change * 100.0);
    }
    if let Some(spy_50d_change) = result.market_context.spy_50d_change {
        println!("   SPY 50-day Change: {:.2}%", spy_50d_change * 100.0);
    }
    if let Some(vix) = result.market_context.vix {
        println!("   VIX: {:.1}", vix);
    }
    if let Some(market_breadth) = result.market_context.market_breadth {
        println!("   Market Breadth: {:.1}%", market_breadth * 100.0);
    }
    
    // Volatility analysis
    println!("\n⚡ Volatility Analysis:");
    println!("   Market Volatility: {:.2}%", result.volatility_analysis.market_volatility);
    println!("   Volatility Percentile: {:.1}%", result.volatility_analysis.volatility_percentile);
    println!("   Volatility Trend: {:?}", result.volatility_analysis.volatility_trend);
    
    // Trend analysis
    println!("\n📈 Trend Analysis:");
    println!("   Short-term Trend: {:?}", result.trend_analysis.short_term);
    println!("   Medium-term Trend: {:?}", result.trend_analysis.medium_term);
    println!("   Long-term Trend: {:?}", result.trend_analysis.long_term);
    println!("   Trend Strength: {:.1}%", result.trend_analysis.strength);
    println!("   Trend Consistency: {:.1}%", result.trend_analysis.consistency);
    
    // Market breadth
    println!("\n📊 Market Breadth:");
    if let Some(advancing) = result.breadth_analysis.advancing_stocks {
        println!("   Advancing Stocks: {}", advancing);
    }
    if let Some(declining) = result.breadth_analysis.declining_stocks {
        println!("   Declining Stocks: {}", declining);
    }
    if let Some(ratio) = result.breadth_analysis.breadth_ratio {
        println!("   Breadth Ratio: {:.2}", ratio);
    }
    if let Some(new_highs) = result.breadth_analysis.new_highs {
        println!("   New Highs: {}", new_highs);
    }
    if let Some(new_lows) = result.breadth_analysis.new_lows {
        println!("   New Lows: {}", new_lows);
    }
    
    // Sector analysis
    println!("\n🏭 Sector Analysis:");
    if let Some(tech) = result.sector_analysis.technology_performance {
        println!("   Technology: {:.2}%", tech * 100.0);
    }
    if let Some(healthcare) = result.sector_analysis.healthcare_performance {
        println!("   Healthcare: {:.2}%", healthcare * 100.0);
    }
    if let Some(financial) = result.sector_analysis.financial_performance {
        println!("   Financial: {:.2}%", financial * 100.0);
    }
    if let Some(energy) = result.sector_analysis.energy_performance {
        println!("   Energy: {:.2}%", energy * 100.0);
    }
    if let Some(consumer) = result.sector_analysis.consumer_performance {
        println!("   Consumer: {:.2}%", consumer * 100.0);
    }
    if let Some(leading) = &result.sector_analysis.leading_sector {
        println!("   Leading Sector: {}", leading);
    }
    if let Some(lagging) = &result.sector_analysis.lagging_sector {
        println!("   Lagging Sector: {}", lagging);
    }
    
    // Sentiment indicators
    println!("\n😊 Sentiment Indicators:");
    if let Some(fear_greed) = result.sentiment_indicators.fear_greed_index {
        println!("   Fear & Greed Index: {} ({})", fear_greed, interpret_fear_greed(fear_greed));
    }
    if let Some(put_call) = result.sentiment_indicators.put_call_ratio {
        println!("   Put/Call Ratio: {:.2}", put_call);
    }
    if let Some(margin_trend) = &result.sentiment_indicators.margin_debt_trend {
        println!("   Margin Debt Trend: {:?}", margin_trend);
    }
    if let Some(insider) = &result.sentiment_indicators.insider_sentiment {
        println!("   Insider Sentiment: {:?}", insider);
    }
    
    // Risk assessment
    println!("\n⚠️  Risk Assessment:");
    println!("   Risk Level: {}", result.risk_assessment.risk_level);
    println!("   Risk Score: {:.1}%", result.risk_assessment.risk_score);
    println!("   Max Drawdown Risk: {:.1}%", result.risk_assessment.max_drawdown_risk);
    println!("   Correlation Risk: {:.1}%", result.risk_assessment.correlation_risk);
    println!("   Liquidity Risk: {:.1}%", result.risk_assessment.liquidity_risk);
    
    // Analysis metadata
    println!("\n📋 Analysis Metadata:");
    println!("   Timestamp: {}", result.timestamp.format("%Y-%m-%d %H:%M:%S UTC"));
    println!("   Data Sources: {:?}", result.metadata.data_sources_used);
    println!("   Analysis Period: {} days", result.metadata.analysis_period_days);
    if let Some(computation_time) = result.metadata.computation_time_ms {
        println!("   Computation Time: {}ms", computation_time);
    }
    
    println!("═══════════════════════════════════════════════════════════════");
}

/// Interpret Fear & Greed Index value
fn interpret_fear_greed(index: i32) -> &'static str {
    match index {
        0..=24 => "Extreme Fear",
        25..=49 => "Fear",
        50..=74 => "Greed",
        75..=100 => "Extreme Greed",
        _ => "Unknown",
    }
}

/// Save the market regime analysis results to a JSON file
fn save_results_to_json(result: &MarketRegimeResult) -> Result<()> {
    // Create a timestamped filename
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("market_regime_analysis_{}.json", timestamp);
    
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
    fn test_fear_greed_interpretation() {
        assert_eq!(interpret_fear_greed(10), "Extreme Fear");
        assert_eq!(interpret_fear_greed(30), "Fear");
        assert_eq!(interpret_fear_greed(60), "Greed");
        assert_eq!(interpret_fear_greed(90), "Extreme Greed");
    }

    #[test]
    fn test_market_regime_result_serialization() {
        // Test that MarketRegimeResult can be serialized to JSON
        let result = create_mock_market_regime_result();
        let json = serde_json::to_string(&result).expect("Should serialize to JSON");
        assert!(!json.is_empty());
        
        // Test deserialization
        let deserialized: MarketRegimeResult = serde_json::from_str(&json).expect("Should deserialize from JSON");
        assert_eq!(result.market_regime, deserialized.market_regime);
        assert_eq!(result.regime_confidence, deserialized.regime_confidence);
    }

    fn create_mock_market_regime_result() -> MarketRegimeResult {
        use buenotea_regime::*;
        use chrono::Utc;

        MarketRegimeResult {
            market_regime: MarketRegime::Bull,
            regime_confidence: 0.85,
            market_context: MarketContext {
                spy_price: Some(450.0),
                spy_20d_change: Some(0.05),
                spy_50d_change: Some(0.08),
                vix: Some(18.5),
                market_breadth: Some(0.65),
            },
            volatility_analysis: VolatilityAnalysis {
                market_volatility: 2.5,
                volatility_percentile: 70.0,
                volatility_trend: VolatilityTrend::Stable,
            },
            trend_analysis: MarketTrendAnalysis {
                short_term: TrendDirection::Bullish,
                medium_term: TrendDirection::Bullish,
                long_term: TrendDirection::Bullish,
                strength: 80.0,
                consistency: 90.0,
            },
            breadth_analysis: MarketBreadthAnalysis {
                advancing_stocks: Some(2500),
                declining_stocks: Some(1500),
                unchanged_stocks: Some(500),
                new_highs: Some(150),
                new_lows: Some(75),
                breadth_ratio: Some(1.67),
            },
            sector_analysis: SectorAnalysis {
                technology_performance: Some(0.05),
                healthcare_performance: Some(0.02),
                financial_performance: Some(-0.01),
                energy_performance: Some(0.08),
                consumer_performance: Some(0.03),
                leading_sector: Some("Energy".to_string()),
                lagging_sector: Some("Financial".to_string()),
            },
            sentiment_indicators: SentimentIndicators {
                fear_greed_index: Some(65),
                put_call_ratio: Some(0.85),
                margin_debt_trend: Some(MarginDebtTrend::Increasing),
                insider_sentiment: Some(InsiderSentiment::Neutral),
            },
            risk_assessment: MarketRiskAssessment {
                risk_level: RiskLevel::Medium,
                risk_score: 45.0,
                max_drawdown_risk: 15.0,
                correlation_risk: 40.0,
                liquidity_risk: 30.0,
            },
            timestamp: Utc::now(),
            metadata: AnalysisMetadata {
                data_sources_used: vec!["FMP".to_string()],
                analysis_period_days: 250,
                computation_time_ms: Some(1500),
                api_endpoints_used: vec!["https://financialmodelingprep.com/api/v3/historical-price-full/SPY".to_string()],
                raw_api_responses: None,
            },
        }
    }
}







