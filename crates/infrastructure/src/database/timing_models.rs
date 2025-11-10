// Database models for timing (TTS) data storage

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Timing record for database storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingRecord {
    pub id: Option<i32>,
    pub symbol: String,
    pub analysis_date: DateTime<Utc>,
    
    // TTS Analysis Results
    pub tts_score: f64,
    pub trading_signal: String,
    pub confidence_score: f64,
    
    // Technical Indicators Scores (-1.0 to +1.0)
    pub rsi_score: f64,
    pub macd_score: f64,
    pub bollinger_score: f64,
    pub ma_score: f64,
    pub stochastic_score: f64,
    pub williams_score: f64,
    pub atr_score: f64,
    pub volume_score: f64,
    
    // Trend Analysis
    pub short_term_trend: String,
    pub medium_term_trend: String,
    pub long_term_trend: String,
    pub trend_strength: f64,
    pub trend_consistency: f64,
    
    // Support & Resistance
    pub support_level: f64,
    pub resistance_level: f64,
    pub support_distance: f64,
    pub resistance_distance: f64,
    pub support_strength: f64,
    pub resistance_strength: f64,
    
    // Volume Analysis
    pub current_volume: i64,
    pub avg_volume: i64,
    pub volume_ratio: f64,
    pub volume_trend: String,
    pub vp_relationship: String,
    
    // Risk Assessment
    pub volatility_score: f64,
    pub risk_level: String,
    pub max_drawdown_risk: f64,
    pub stop_loss: f64,
    pub risk_reward_ratio: f64,
    
    // API Source Tracking
    pub primary_api_source: String,
    pub fallback_api_source: Option<String>,
    pub api_endpoints_used: Vec<String>,
    pub raw_api_responses: Option<HashMap<String, serde_json::Value>>,
    
    // Price Data Used for Analysis
    pub price_data_points: i32,
    pub analysis_period_days: i32,
    pub current_price: f64,
    
    // AI Analysis
    pub chatgpt_explanation: Option<String>,
    pub trading_suggestion: Option<String>,
    
    // Metadata
    pub flags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// API endpoint information for tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEndpointInfo {
    pub url: String,
    pub method: String,
    pub response_size_bytes: Option<usize>,
    pub response_time_ms: Option<u64>,
    pub status_code: Option<u16>,
    pub success: bool,
}

/// Raw API response data for debugging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawApiResponse {
    pub endpoint: String,
    pub timestamp: DateTime<Utc>,
    pub request_url: String,
    pub response_data: serde_json::Value,
    pub response_headers: Option<HashMap<String, String>>,
    pub error_message: Option<String>,
}

/// Timing record creation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTimingRecord {
    pub symbol: String,
    pub analysis_date: DateTime<Utc>,
    pub tts_score: f64,
    pub trading_signal: String,
    pub confidence_score: f64,
    
    // Technical Indicators Scores
    pub rsi_score: f64,
    pub macd_score: f64,
    pub bollinger_score: f64,
    pub ma_score: f64,
    pub stochastic_score: f64,
    pub williams_score: f64,
    pub atr_score: f64,
    pub volume_score: f64,
    
    // Trend Analysis
    pub short_term_trend: String,
    pub medium_term_trend: String,
    pub long_term_trend: String,
    pub trend_strength: f64,
    pub trend_consistency: f64,
    
    // Support & Resistance
    pub support_level: f64,
    pub resistance_level: f64,
    pub support_distance: f64,
    pub resistance_distance: f64,
    pub support_strength: f64,
    pub resistance_strength: f64,
    
    // Volume Analysis
    pub current_volume: i64,
    pub avg_volume: i64,
    pub volume_ratio: f64,
    pub volume_trend: String,
    pub vp_relationship: String,
    
    // Risk Assessment
    pub volatility_score: f64,
    pub risk_level: String,
    pub max_drawdown_risk: f64,
    pub stop_loss: f64,
    pub risk_reward_ratio: f64,
    
    // API Source Tracking
    pub primary_api_source: String,
    pub fallback_api_source: Option<String>,
    pub api_endpoints_used: Vec<String>,
    pub raw_api_responses: Option<HashMap<String, serde_json::Value>>,
    
    // Price Data Used for Analysis
    pub price_data_points: i32,
    pub analysis_period_days: i32,
    pub current_price: f64,
    
    // AI Analysis
    pub chatgpt_explanation: Option<String>,
    pub trading_suggestion: Option<String>,
    
    // Metadata
    pub flags: Vec<String>,
}

/// Timing record insert (for database inserts, without auto-generated fields)
#[derive(Debug, Clone, Serialize)]
pub struct TimingInsert {
    pub symbol: String,
    pub analysis_date: DateTime<Utc>,
    pub tts_score: f64,
    pub trading_signal: String,
    pub confidence_score: f64,
    pub rsi_score: f64,
    pub macd_score: f64,
    pub bollinger_score: f64,
    pub ma_score: f64,
    pub stochastic_score: f64,
    pub williams_score: f64,
    pub atr_score: f64,
    pub volume_score: f64,
    pub short_term_trend: String,
    pub medium_term_trend: String,
    pub long_term_trend: String,
    pub trend_strength: f64,
    pub trend_consistency: f64,
    pub support_level: f64,
    pub resistance_level: f64,
    pub support_distance: f64,
    pub resistance_distance: f64,
    pub support_strength: f64,
    pub resistance_strength: f64,
    pub current_volume: i64,
    pub avg_volume: i64,
    pub volume_ratio: f64,
    pub volume_trend: String,
    pub vp_relationship: String,
    pub volatility_score: f64,
    pub risk_level: String,
    pub max_drawdown_risk: f64,
    pub stop_loss: f64,
    pub risk_reward_ratio: f64,
    pub primary_api_source: String,
    pub fallback_api_source: Option<String>,
    pub api_endpoints_used: serde_json::Value, // Stored as JSONB
    pub raw_api_responses: Option<serde_json::Value>, // Stored as JSONB
    pub price_data_points: i32,
    pub analysis_period_days: i32,
    pub current_price: f64,
    pub chatgpt_explanation: Option<String>,
    pub trading_suggestion: Option<String>,
    pub flags: serde_json::Value, // Stored as JSONB
}

impl From<CreateTimingRecord> for TimingInsert {
    fn from(record: CreateTimingRecord) -> Self {
        Self {
            symbol: record.symbol,
            analysis_date: record.analysis_date,
            tts_score: record.tts_score,
            trading_signal: record.trading_signal,
            confidence_score: record.confidence_score,
            rsi_score: record.rsi_score,
            macd_score: record.macd_score,
            bollinger_score: record.bollinger_score,
            ma_score: record.ma_score,
            stochastic_score: record.stochastic_score,
            williams_score: record.williams_score,
            atr_score: record.atr_score,
            volume_score: record.volume_score,
            short_term_trend: record.short_term_trend,
            medium_term_trend: record.medium_term_trend,
            long_term_trend: record.long_term_trend,
            trend_strength: record.trend_strength,
            trend_consistency: record.trend_consistency,
            support_level: record.support_level,
            resistance_level: record.resistance_level,
            support_distance: record.support_distance,
            resistance_distance: record.resistance_distance,
            support_strength: record.support_strength,
            resistance_strength: record.resistance_strength,
            current_volume: record.current_volume,
            avg_volume: record.avg_volume,
            volume_ratio: record.volume_ratio,
            volume_trend: record.volume_trend,
            vp_relationship: record.vp_relationship,
            volatility_score: record.volatility_score,
            risk_level: record.risk_level,
            max_drawdown_risk: record.max_drawdown_risk,
            stop_loss: record.stop_loss,
            risk_reward_ratio: record.risk_reward_ratio,
            primary_api_source: record.primary_api_source,
            fallback_api_source: record.fallback_api_source,
            api_endpoints_used: serde_json::json!(record.api_endpoints_used),
            raw_api_responses: record.raw_api_responses.map(|m| serde_json::json!(m)),
            price_data_points: record.price_data_points,
            analysis_period_days: record.analysis_period_days,
            current_price: record.current_price,
            chatgpt_explanation: record.chatgpt_explanation,
            trading_suggestion: record.trading_suggestion,
            flags: serde_json::json!(record.flags),
        }
    }
}

// Note: The From implementation and create_timing_record_with_tracking function
// are now defined in the buenotea-timing crate to avoid circular dependencies

/// Timing record update request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTimingRecord {
    pub chatgpt_explanation: Option<String>,
    pub trading_suggestion: Option<String>,
    pub raw_api_responses: Option<HashMap<String, serde_json::Value>>,
    pub flags: Option<Vec<String>>,
}

/// Query parameters for timing records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingQueryParams {
    pub symbol: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub min_tts_score: Option<f64>,
    pub max_tts_score: Option<f64>,
    pub trading_signal: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::timing::models::*;
    use crate::timing::indicators::*;

    #[test]
    fn test_timing_record_creation() {
        let create_record = CreateTimingRecord {
            symbol: "AAPL".to_string(),
            tts_score: 65.5,
            trading_signal: "Buy".to_string(),
            confidence_score: 0.85,
            rsi_score: 70.0,
            macd_score: 80.0,
            bollinger_score: 60.0,
            ma_score: 75.0,
            stochastic_score: 65.0,
            williams_score: 70.0,
            atr_score: 55.0,
            volume_score: 80.0,
            short_term_trend: "Bullish".to_string(),
            medium_term_trend: "Bullish".to_string(),
            long_term_trend: "Neutral".to_string(),
            trend_strength: 75.0,
            trend_consistency: 80.0,
            support_level: 150.0,
            resistance_level: 200.0,
            support_distance: 5.0,
            resistance_distance: 10.0,
            support_strength: 80.0,
            resistance_strength: 70.0,
            current_volume: 50000000,
            avg_volume: 45000000,
            volume_ratio: 1.11,
            volume_trend: "Increasing".to_string(),
            vp_relationship: "BullishDivergence".to_string(),
            volatility_score: 60.0,
            risk_level: "Medium".to_string(),
            max_drawdown_risk: 8.5,
            stop_loss: 145.0,
            risk_reward_ratio: 2.0,
            primary_api_source: "FMP".to_string(),
            fallback_api_source: Some("Alpha Vantage".to_string()),
            api_endpoints_used: vec!["https://financialmodelingprep.com/api/v3/historical-price-full/AAPL".to_string()],
            raw_api_responses: None,
            price_data_points: 50,
            analysis_period_days: 30,
            current_price: 175.0,
            chatgpt_explanation: Some("Technical indicators show bullish momentum with strong volume support.".to_string()),
            trading_suggestion: Some("Consider a long position with stop loss at $145.".to_string()),
            flags: vec!["High volume confirmation".to_string()],
        };

        assert_eq!(create_record.symbol, "AAPL");
        assert_eq!(create_record.tts_score, 65.5);
        assert_eq!(create_record.trading_signal, "Buy");
    }
}
