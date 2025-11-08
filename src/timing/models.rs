// Data models for Technical Trading Score (TTS) calculations

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Technical Trading Score result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TTSResult {
    /// Stock symbol analyzed
    pub symbol: String,
    /// Final TTS score between [-1.0, +1.0]
    /// +1.0 = Strong Buy, +0.5 = Buy, 0.0 = Neutral, -0.5 = Sell, -1.0 = Strong Sell
    pub tts_score: f64,
    /// Generated trading signal
    pub trading_signal: TTSSignal,
    /// Individual technical indicator scores (all between -1.0 and +1.0)
    pub indicators: TTSIndicators,
    /// Trend analysis
    pub trend_analysis: TrendAnalysis,
    /// Support and resistance levels
    pub support_resistance: SupportResistance,
    /// Volume analysis
    pub volume_analysis: VolumeAnalysis,
    /// Risk assessment
    pub risk_assessment: RiskAssessment,
    /// Timestamp of analysis
    pub timestamp: DateTime<Utc>,
    /// Confidence score (0.0 to 1.0)
    pub confidence_score: f64,
    /// Data quality flags
    pub flags: Vec<String>,
}

/// TTS trading signal
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TTSSignal {
    /// Strong buy signal (TTS >= +0.6)
    StrongBuy,
    /// Buy signal (TTS >= +0.2)
    Buy,
    /// Neutral signal (TTS between -0.2 and +0.2)
    Neutral,
    /// Sell signal (TTS <= -0.2)
    Sell,
    /// Strong sell signal (TTS <= -0.6)
    StrongSell,
}

impl TTSSignal {
    /// Convert a TTS score (-1.0 to +1.0) to a trading signal
    pub fn from_score(score: f64) -> Self {
        match score {
            s if s >= 0.6 => TTSSignal::StrongBuy,
            s if s >= 0.2 => TTSSignal::Buy,
            s if s >= -0.2 => TTSSignal::Neutral,
            s if s >= -0.6 => TTSSignal::Sell,
            _ => TTSSignal::StrongSell,
        }
    }

    /// Get position sizing recommendation (matches the +1 to -1 scale)
    pub fn position_size(&self) -> f64 {
        match self {
            TTSSignal::StrongBuy => 1.0,    // Maximum long position
            TTSSignal::Buy => 0.5,          // Moderate long position
            TTSSignal::Neutral => 0.0,      // No change
            TTSSignal::Sell => -0.5,        // Moderate short position
            TTSSignal::StrongSell => -1.0,  // Maximum short position
        }
    }

    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            TTSSignal::StrongBuy => "Strong Buy - Excellent technical setup",
            TTSSignal::Buy => "Buy - Positive technical indicators",
            TTSSignal::Neutral => "Neutral - Mixed technical signals",
            TTSSignal::Sell => "Sell - Negative technical indicators",
            TTSSignal::StrongSell => "Strong Sell - Poor technical setup",
        }
    }

    /// Get emoji representation
    pub fn emoji(&self) -> &'static str {
        match self {
            TTSSignal::StrongBuy => "🚀",
            TTSSignal::Buy => "📈",
            TTSSignal::Neutral => "➡️",
            TTSSignal::Sell => "📉",
            TTSSignal::StrongSell => "💥",
        }
    }
}

/// Technical indicators scores (all between -1.0 and +1.0)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TTSIndicators {
    /// RSI score (-1.0 to +1.0)
    pub rsi_score: f64,
    /// MACD score (-1.0 to +1.0)
    pub macd_score: f64,
    /// Bollinger Bands score (-1.0 to +1.0)
    pub bollinger_score: f64,
    /// Moving Averages score (-1.0 to +1.0)
    pub ma_score: f64,
    /// Stochastic score (-1.0 to +1.0)
    pub stochastic_score: f64,
    /// Williams %R score (-1.0 to +1.0)
    pub williams_score: f64,
    /// Average True Range score (-1.0 to +1.0)
    pub atr_score: f64,
    /// Volume score (-1.0 to +1.0)
    pub volume_score: f64,
}

/// Trend analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendAnalysis {
    /// Short-term trend (1-5 days)
    pub short_term: TrendDirection,
    /// Medium-term trend (1-4 weeks)
    pub medium_term: TrendDirection,
    /// Long-term trend (1-3 months)
    pub long_term: TrendDirection,
    /// Trend strength (0-100)
    pub strength: f64,
    /// Trend consistency score
    pub consistency: f64,
}

/// Trend direction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TrendDirection {
    StrongBullish,
    Bullish,
    Neutral,
    Bearish,
    StrongBearish,
}

impl TrendDirection {
    pub fn score(&self) -> f64 {
        match self {
            TrendDirection::StrongBullish => 1.0,   // +1.0 = Strong Bullish
            TrendDirection::Bullish => 0.5,         // +0.5 = Bullish
            TrendDirection::Neutral => 0.0,         // 0.0 = Neutral
            TrendDirection::Bearish => -0.5,        // -0.5 = Bearish
            TrendDirection::StrongBearish => -1.0,  // -1.0 = Strong Bearish
        }
    }
}

/// Support and resistance levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportResistance {
    /// Nearest support level
    pub support_level: f64,
    /// Nearest resistance level
    pub resistance_level: f64,
    /// Distance to support (%)
    pub support_distance: f64,
    /// Distance to resistance (%)
    pub resistance_distance: f64,
    /// Support strength (0-100)
    pub support_strength: f64,
    /// Resistance strength (0-100)
    pub resistance_strength: f64,
}

/// Volume analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeAnalysis {
    /// Current volume
    pub current_volume: u64,
    /// Average volume (20-day)
    pub avg_volume: u64,
    /// Volume ratio
    pub volume_ratio: f64,
    /// Volume trend
    pub volume_trend: VolumeTrend,
    /// Volume-price relationship
    pub vp_relationship: VolumePriceRelationship,
}

/// Volume trend
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VolumeTrend {
    Increasing,
    Stable,
    Decreasing,
}

/// Volume-price relationship
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VolumePriceRelationship {
    BullishDivergence,  // Price up, volume up
    BearishDivergence,  // Price up, volume down
    Neutral,
}

/// Risk assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    /// Volatility score (0-100, higher = more volatile)
    pub volatility_score: f64,
    /// Risk level
    pub risk_level: RiskLevel,
    /// Maximum drawdown risk
    pub max_drawdown_risk: f64,
    /// Stop loss recommendation
    pub stop_loss: f64,
    /// Risk-reward ratio
    pub risk_reward_ratio: f64,
}

/// Risk level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    VeryHigh,
}

impl RiskLevel {
    pub fn score(&self) -> f64 {
        match self {
            RiskLevel::Low => 25.0,
            RiskLevel::Medium => 50.0,
            RiskLevel::High => 75.0,
            RiskLevel::VeryHigh => 100.0,
        }
    }
}

/// Price data point for technical analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricePoint {
    pub date: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
}

impl PricePoint {
    /// Calculate true range
    pub fn true_range(&self, prev_close: f64) -> f64 {
        let high_low = self.high - self.low;
        let high_prev = (self.high - prev_close).abs();
        let low_prev = (self.low - prev_close).abs();
        high_low.max(high_prev).max(low_prev)
    }

    /// Calculate typical price
    pub fn typical_price(&self) -> f64 {
        (self.high + self.low + self.close) / 3.0
    }

    /// Calculate body size
    pub fn body_size(&self) -> f64 {
        (self.close - self.open).abs()
    }

    /// Calculate upper shadow
    pub fn upper_shadow(&self) -> f64 {
        self.high - self.open.max(self.close)
    }

    /// Calculate lower shadow
    pub fn lower_shadow(&self) -> f64 {
        self.open.min(self.close) - self.low
    }
}

/// Technical indicator values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorValues {
    pub rsi_14: Option<f64>,
    pub macd: Option<f64>,
    pub macd_signal: Option<f64>,
    pub macd_histogram: Option<f64>,
    pub bollinger_upper: Option<f64>,
    pub bollinger_middle: Option<f64>,
    pub bollinger_lower: Option<f64>,
    pub sma_20: Option<f64>,
    pub sma_50: Option<f64>,
    pub sma_200: Option<f64>,
    pub ema_12: Option<f64>,
    pub ema_26: Option<f64>,
    pub stochastic_k: Option<f64>,
    pub stochastic_d: Option<f64>,
    pub williams_r: Option<f64>,
    pub atr_14: Option<f64>,
}

/// API tracking information for TTS calculations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TTSApiTracking {
    pub primary_api_source: String,
    pub fallback_api_source: Option<String>,
    pub api_endpoints_used: Vec<String>,
    pub raw_api_responses: Option<std::collections::HashMap<String, serde_json::Value>>,
    pub price_data_points: i32,
    pub analysis_period_days: i32,
    pub current_price: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_tts_signal_position_sizing() {
        assert_eq!(TTSSignal::StrongBuy.position_size(), 0.15);
        assert_eq!(TTSSignal::Buy.position_size(), 0.10);
        assert_eq!(TTSSignal::Neutral.position_size(), 0.0);
        assert_eq!(TTSSignal::Sell.position_size(), -0.10);
        assert_eq!(TTSSignal::StrongSell.position_size(), -0.15);
    }

    #[test]
    fn test_trend_direction_scores() {
        assert_eq!(TrendDirection::StrongBullish.score(), 90.0);
        assert_eq!(TrendDirection::Bullish.score(), 70.0);
        assert_eq!(TrendDirection::Neutral.score(), 50.0);
        assert_eq!(TrendDirection::Bearish.score(), 30.0);
        assert_eq!(TrendDirection::StrongBearish.score(), 10.0);
    }

    #[test]
    fn test_risk_level_scores() {
        assert_eq!(RiskLevel::Low.score(), 25.0);
        assert_eq!(RiskLevel::Medium.score(), 50.0);
        assert_eq!(RiskLevel::High.score(), 75.0);
        assert_eq!(RiskLevel::VeryHigh.score(), 100.0);
    }

    #[test]
    fn test_price_point_calculations() {
        let price = PricePoint {
            date: Utc::now(),
            open: 100.0,
            high: 105.0,
            low: 98.0,
            close: 103.0,
            volume: 1000000,
        };

        assert_eq!(price.typical_price(), (105.0 + 98.0 + 103.0) / 3.0);
        assert_eq!(price.body_size(), 3.0);
        assert_eq!(price.upper_shadow(), 2.0);
        assert_eq!(price.lower_shadow(), 2.0);
        assert_eq!(price.true_range(99.0), 6.0);
    }
}
