// Database models for Market Regime Analysis
// Stores overall market regime analysis in the database
// Note: These are database-specific types. Conversion from regime crate types happens in the regime crate.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Market regime database record (read from database)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketRegimeRecord {
    pub id: Option<i32>,
    pub analysis_date: DateTime<Utc>,
    
    // Market Regime Classification
    pub market_regime: String,
    pub regime_confidence: f64,
    
    // Market Context Data
    pub spy_price: Option<f64>,
    pub spy_20d_change: Option<f64>,
    pub spy_50d_change: Option<f64>,
    pub vix: Option<f64>,
    pub market_breadth: Option<f64>,
    
    // Market Volatility Analysis
    pub market_volatility: f64,
    pub volatility_percentile: f64,
    
    // Market Trend Analysis
    pub short_term_trend: String,
    pub medium_term_trend: String,
    pub long_term_trend: String,
    pub trend_strength: f64,
    pub trend_consistency: f64,
    
    // Market Breadth Analysis
    pub advancing_stocks: Option<i32>,
    pub declining_stocks: Option<i32>,
    pub unchanged_stocks: Option<i32>,
    pub new_highs: Option<i32>,
    pub new_lows: Option<i32>,
    
    // Sector Analysis
    pub technology_performance: Option<f64>,
    pub healthcare_performance: Option<f64>,
    pub financial_performance: Option<f64>,
    pub energy_performance: Option<f64>,
    pub consumer_performance: Option<f64>,
    
    // Market Sentiment Indicators
    pub fear_greed_index: Option<i32>,
    pub put_call_ratio: Option<f64>,
    pub margin_debt_trend: Option<String>,
    
    // Risk Assessment
    pub market_risk_level: String,
    pub market_risk_score: f64,
    pub max_drawdown_risk: f64,
    
    // AI Analysis (Optional)
    pub chatgpt_regime_analysis: Option<String>,
    pub chatgpt_market_outlook: Option<String>,
    pub chatgpt_risk_assessment: Option<String>,
    pub chatgpt_model_used: Option<String>,
    pub chatgpt_analysis_timestamp: Option<DateTime<Utc>>,
    
    // Analysis Metadata
    pub data_sources_used: Vec<String>,
    pub analysis_period_days: i32,
    pub computation_time_ms: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Market regime record creation request (for creating new records)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMarketRegimeRecord {
    pub analysis_date: DateTime<Utc>,
    
    // Market Regime Classification
    pub market_regime: String,
    pub regime_confidence: f64,
    
    // Market Context Data
    pub spy_price: Option<f64>,
    pub spy_20d_change: Option<f64>,
    pub spy_50d_change: Option<f64>,
    pub vix: Option<f64>,
    pub market_breadth: Option<f64>,
    
    // Market Volatility Analysis
    pub market_volatility: f64,
    pub volatility_percentile: f64,
    
    // Market Trend Analysis
    pub short_term_trend: String,
    pub medium_term_trend: String,
    pub long_term_trend: String,
    pub trend_strength: f64,
    pub trend_consistency: f64,
    
    // Market Breadth Analysis
    pub advancing_stocks: Option<i32>,
    pub declining_stocks: Option<i32>,
    pub unchanged_stocks: Option<i32>,
    pub new_highs: Option<i32>,
    pub new_lows: Option<i32>,
    
    // Sector Analysis
    pub technology_performance: Option<f64>,
    pub healthcare_performance: Option<f64>,
    pub financial_performance: Option<f64>,
    pub energy_performance: Option<f64>,
    pub consumer_performance: Option<f64>,
    
    // Market Sentiment Indicators
    pub fear_greed_index: Option<i32>,
    pub put_call_ratio: Option<f64>,
    pub margin_debt_trend: Option<String>,
    
    // Risk Assessment
    pub market_risk_level: String,
    pub market_risk_score: f64,
    pub max_drawdown_risk: f64,
    
    // AI Analysis (Optional)
    pub chatgpt_regime_analysis: Option<String>,
    pub chatgpt_market_outlook: Option<String>,
    pub chatgpt_risk_assessment: Option<String>,
    pub chatgpt_model_used: Option<String>,
    pub chatgpt_analysis_timestamp: Option<DateTime<Utc>>,
    
    // Analysis Metadata
    pub data_sources_used: Vec<String>,
    pub analysis_period_days: i32,
    pub computation_time_ms: Option<i64>,
}

/// Market regime record insert (for database inserts, without auto-generated fields)
#[derive(Debug, Clone, Serialize)]
pub struct MarketRegimeInsert {
    pub analysis_date: DateTime<Utc>,
    pub market_regime: String,
    pub regime_confidence: f64,
    pub spy_price: Option<f64>,
    pub spy_20d_change: Option<f64>,
    pub spy_50d_change: Option<f64>,
    pub vix: Option<f64>,
    pub market_breadth: Option<f64>,
    pub market_volatility: f64,
    pub volatility_percentile: f64,
    pub short_term_trend: String,
    pub medium_term_trend: String,
    pub long_term_trend: String,
    pub trend_strength: f64,
    pub trend_consistency: f64,
    pub advancing_stocks: Option<i32>,
    pub declining_stocks: Option<i32>,
    pub unchanged_stocks: Option<i32>,
    pub new_highs: Option<i32>,
    pub new_lows: Option<i32>,
    pub technology_performance: Option<f64>,
    pub healthcare_performance: Option<f64>,
    pub financial_performance: Option<f64>,
    pub energy_performance: Option<f64>,
    pub consumer_performance: Option<f64>,
    pub fear_greed_index: Option<i32>,
    pub put_call_ratio: Option<f64>,
    pub margin_debt_trend: Option<String>,
    pub market_risk_level: String,
    pub market_risk_score: f64,
    pub max_drawdown_risk: f64,
    pub chatgpt_regime_analysis: Option<String>,
    pub chatgpt_market_outlook: Option<String>,
    pub chatgpt_risk_assessment: Option<String>,
    pub chatgpt_model_used: Option<String>,
    pub chatgpt_analysis_timestamp: Option<DateTime<Utc>>,
    pub data_sources_used: serde_json::Value, // Stored as TEXT[] in DB, serialized as JSON
    pub analysis_period_days: i32,
    pub computation_time_ms: Option<i64>,
}

impl From<CreateMarketRegimeRecord> for MarketRegimeInsert {
    fn from(record: CreateMarketRegimeRecord) -> Self {
        Self {
            analysis_date: record.analysis_date,
            market_regime: record.market_regime,
            regime_confidence: record.regime_confidence,
            spy_price: record.spy_price,
            spy_20d_change: record.spy_20d_change,
            spy_50d_change: record.spy_50d_change,
            vix: record.vix,
            market_breadth: record.market_breadth,
            market_volatility: record.market_volatility,
            volatility_percentile: record.volatility_percentile,
            short_term_trend: record.short_term_trend,
            medium_term_trend: record.medium_term_trend,
            long_term_trend: record.long_term_trend,
            trend_strength: record.trend_strength,
            trend_consistency: record.trend_consistency,
            advancing_stocks: record.advancing_stocks,
            declining_stocks: record.declining_stocks,
            unchanged_stocks: record.unchanged_stocks,
            new_highs: record.new_highs,
            new_lows: record.new_lows,
            technology_performance: record.technology_performance,
            healthcare_performance: record.healthcare_performance,
            financial_performance: record.financial_performance,
            energy_performance: record.energy_performance,
            consumer_performance: record.consumer_performance,
            fear_greed_index: record.fear_greed_index,
            put_call_ratio: record.put_call_ratio,
            margin_debt_trend: record.margin_debt_trend,
            market_risk_level: record.market_risk_level,
            market_risk_score: record.market_risk_score,
            max_drawdown_risk: record.max_drawdown_risk,
            chatgpt_regime_analysis: record.chatgpt_regime_analysis,
            chatgpt_market_outlook: record.chatgpt_market_outlook,
            chatgpt_risk_assessment: record.chatgpt_risk_assessment,
            chatgpt_model_used: record.chatgpt_model_used,
            chatgpt_analysis_timestamp: record.chatgpt_analysis_timestamp,
            data_sources_used: serde_json::json!(record.data_sources_used),
            analysis_period_days: record.analysis_period_days,
            computation_time_ms: record.computation_time_ms,
        }
    }
}

// Note: The From implementation and create_market_regime_record_with_tracking function
// are defined in the buenotea-regime crate to avoid circular dependencies
