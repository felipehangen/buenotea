// Market Regime Analysis to Supabase
// Analyzes the overall market regime and saves it to the market_regime table

use sentiment_backend::regime::{MarketRegimeCalculator, MarketRegimeResult};
use sentiment_backend::database::{DatabaseClient, MarketRegimeRecord};
use sentiment_backend::error::Result;
use dotenv::dotenv;
use tokio;
use serde_json;
use std::env;
use tracing::error;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    println!("🌍 Starting Market Regime Analysis with Supabase storage...");
    println!("   Analyzing the overall market mood - \"What's the vibe of the whole club?\"\n");

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

    // Initialize market regime calculator
    let mut calculator = MarketRegimeCalculator::new();

    // Step 1: Run market regime analysis
    println!("\n🔍 Calculating market regime analysis...");
    let start_time = std::time::Instant::now();
    let market_regime_result = match calculator.calculate_market_regime().await {
        Ok(result) => {
            let computation_time = start_time.elapsed().as_millis() as i64;
            println!("✅ Market regime analysis completed successfully in {}ms", computation_time);
            result
        }
        Err(e) => {
            error!("❌ Market regime analysis failed: {}", e);
            return Err(e);
        }
    };

    // Display the results
    display_market_regime_results(&market_regime_result);

    // Step 2: Create database record
    println!("\n📝 Preparing database record...");
    let market_regime_record = MarketRegimeRecord::from_market_regime_result(&market_regime_result, None);

    // Step 3: Save to Supabase
    println!("\n💾 Saving to Supabase market_regime table...");
    let record_id = match save_market_regime_to_db(&db_client, &market_regime_record).await {
        Ok(id) => {
            println!("✅ Successfully saved to database with ID: {}", id);
            id
        }
        Err(e) => {
            error!("❌ Failed to save to database: {}", e);
            return Err(e);
        }
    };

    // Step 4: Display summary
    println!("\n📊 Market Regime Analysis Summary:");
    println!("   Market Regime: {}", market_regime_record.market_regime);
    println!("   Confidence: {:.1}%", market_regime_record.regime_confidence * 100.0);
    println!("   Risk Level: {}", market_regime_record.market_risk_level);
    println!("   SPY Price: ${:.2}", market_regime_record.spy_price.unwrap_or(0.0));
    println!("   VIX: {:.1}", market_regime_record.vix.unwrap_or(0.0));
    println!("   Market Breadth: {:.1}%", market_regime_record.market_breadth.unwrap_or(0.0) * 100.0);
    println!("   Database ID: {}", record_id);

    // Step 5: Save results to JSON file for reference
    save_results_to_json(&market_regime_result, &market_regime_record)?;
    
    println!("\n📁 Results saved to JSON files");
    println!("🎉 Market regime analysis successfully completed and stored in Supabase!");
    
    Ok(())
}

/// Display detailed market regime analysis results
fn display_market_regime_results(result: &MarketRegimeResult) {
    println!("\n🌍 Market Regime Analysis Results");
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

/// Save market regime record to database
async fn save_market_regime_to_db(
    db_client: &DatabaseClient,
    record: &MarketRegimeRecord,
) -> Result<i32> {
    let url = format!("{}/rest/v1/market_regime", db_client.config().supabase_url);
    
    let response = db_client
        .http_client()
        .post(&url)
        .header("apikey", &db_client.config().supabase_api_key)
        .header("Authorization", format!("Bearer {}", db_client.config().supabase_api_key))
        .header("Content-Type", "application/json")
        .header("Prefer", "return=representation")
        .json(record)
        .send()
        .await?;

    if response.status().is_success() {
        let response_text = response.text().await?;
        let records: Vec<serde_json::Value> = serde_json::from_str(&response_text)?;
        
        if let Some(first_record) = records.first() {
            let id = first_record.get("id")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| sentiment_backend::error::Error::ValidationError {
                    message: "No ID returned from database".to_string(),
                })?;
            Ok(id as i32)
        } else {
            Err(sentiment_backend::error::Error::ValidationError {
                message: "No records returned from database".to_string(),
            })
        }
    } else {
        let error_text = response.text().await?;
        Err(sentiment_backend::error::Error::DatabaseError(format!(
            "Failed to insert market regime record: {}",
            error_text
        )))
    }
}

/// Save the market regime analysis results to JSON files for reference
fn save_results_to_json(
    result: &MarketRegimeResult, 
    record: &MarketRegimeRecord,
) -> Result<()> {
    // Create a timestamped filename
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    
    // Save market regime results
    let result_filename = format!("market_regime_analysis_{}.json", timestamp);
    let result_json = serde_json::to_string_pretty(result)?;
    std::fs::write(&result_filename, result_json).map_err(|e| sentiment_backend::error::Error::ValidationError {
        message: format!("Failed to write result JSON file: {}", e)
    })?;
    
    // Save market regime record
    let record_filename = format!("market_regime_record_{}.json", timestamp);
    let record_json = serde_json::to_string_pretty(record)?;
    std::fs::write(&record_filename, record_json).map_err(|e| sentiment_backend::error::Error::ValidationError {
        message: format!("Failed to write record JSON file: {}", e)
    })?;
    
    println!("💾 Market regime results saved to: {}", result_filename);
    println!("💾 Market regime record saved to: {}", record_filename);
    
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
    fn test_market_regime_record_serialization() {
        // Test that MarketRegimeRecord can be serialized to JSON
        let record = create_mock_market_regime_record();
        let json = serde_json::to_string(&record).expect("Should serialize to JSON");
        assert!(!json.is_empty());
        
        // Test deserialization
        let deserialized: MarketRegimeRecord = serde_json::from_str(&json).expect("Should deserialize from JSON");
        assert_eq!(record.market_regime, deserialized.market_regime);
        assert_eq!(record.regime_confidence, deserialized.regime_confidence);
    }

    fn create_mock_market_regime_record() -> MarketRegimeRecord {
        use chrono::Utc;

        MarketRegimeRecord {
            id: None,
            analysis_date: Utc::now(),
            market_regime: "Bull".to_string(),
            regime_confidence: 0.85,
            spy_price: Some(450.0),
            spy_20d_change: Some(0.05),
            spy_50d_change: Some(0.08),
            vix: Some(18.5),
            market_breadth: Some(0.65),
            market_volatility: 2.5,
            volatility_percentile: 70.0,
            volatility_trend: "Stable".to_string(),
            short_term_trend: "Bullish".to_string(),
            medium_term_trend: "Bullish".to_string(),
            long_term_trend: "Bullish".to_string(),
            trend_strength: 80.0,
            trend_consistency: 90.0,
            advancing_stocks: Some(2500),
            declining_stocks: Some(1500),
            unchanged_stocks: Some(500),
            new_highs: Some(150),
            new_lows: Some(75),
            technology_performance: Some(0.05),
            healthcare_performance: Some(0.02),
            financial_performance: Some(-0.01),
            energy_performance: Some(0.08),
            consumer_performance: Some(0.03),
            leading_sector: Some("Energy".to_string()),
            lagging_sector: Some("Financial".to_string()),
            fear_greed_index: Some(65),
            put_call_ratio: Some(0.85),
            margin_debt_trend: Some("Increasing".to_string()),
            insider_sentiment: Some("Neutral".to_string()),
            market_risk_level: "Medium".to_string(),
            market_risk_score: 45.0,
            max_drawdown_risk: 15.0,
            correlation_risk: 40.0,
            liquidity_risk: 30.0,
            chatgpt_regime_analysis: None,
            chatgpt_market_outlook: None,
            chatgpt_risk_assessment: None,
            chatgpt_model_used: None,
            chatgpt_analysis_timestamp: None,
            data_sources_used: vec!["FMP".to_string()],
            analysis_period_days: 250,
            computation_time_ms: Some(1500),
            api_endpoints_used: vec!["https://financialmodelingprep.com/api/v3/historical-price-full/SPY".to_string()],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
