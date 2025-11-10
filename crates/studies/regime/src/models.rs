// Data models for Market Regime Analysis
// This represents the overall market regime - "What's the vibe of the whole club?"

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Market regime classification - the overall market mood
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MarketRegime {
    /// Bull market - strong upward trend across the market
    Bull,
    /// Bear market - strong downward trend across the market
    Bear,
    /// Sideways/consolidation market - no clear direction
    Sideways,
    /// High volatility regime - choppy, uncertain conditions
    Volatile,
    /// Low volatility regime - calm, stable conditions
    Stable,
    /// Transition between regimes - changing conditions
    Transition,
}

impl MarketRegime {
    /// Get the regime's impact on individual stock analysis (multiplier)
    pub fn stock_analysis_multiplier(&self) -> f64 {
        match self {
            MarketRegime::Bull => 1.2,        // Bull markets favor holding
            MarketRegime::Bear => 0.8,        // Bear markets favor selling
            MarketRegime::Sideways => 1.0,    // Neutral impact
            MarketRegime::Volatile => 0.9,    // Volatility slightly favors caution
            MarketRegime::Stable => 1.1,      // Stability favors holding
            MarketRegime::Transition => 0.95, // Transition periods are uncertain
        }
    }
}

impl std::fmt::Display for MarketRegime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MarketRegime::Bull => write!(f, "Bull"),
            MarketRegime::Bear => write!(f, "Bear"),
            MarketRegime::Sideways => write!(f, "Sideways"),
            MarketRegime::Volatile => write!(f, "Volatile"),
            MarketRegime::Stable => write!(f, "Stable"),
            MarketRegime::Transition => write!(f, "Transition"),
        }
    }
}

impl MarketRegime {
    /// Get a human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            MarketRegime::Bull => "Bull Market - Strong upward trend across the market, favorable for holding",
            MarketRegime::Bear => "Bear Market - Strong downward trend across the market, consider selling",
            MarketRegime::Sideways => "Sideways Market - Consolidation phase, mixed signals",
            MarketRegime::Volatile => "Volatile Market - High volatility, increased risk and uncertainty",
            MarketRegime::Stable => "Stable Market - Low volatility, steady conditions",
            MarketRegime::Transition => "Transition Market - Changing conditions, high uncertainty",
        }
    }

    /// Get emoji representation
    pub fn emoji(&self) -> &'static str {
        match self {
            MarketRegime::Bull => "🐂",
            MarketRegime::Bear => "🐻",
            MarketRegime::Sideways => "➡️",
            MarketRegime::Volatile => "⚡",
            MarketRegime::Stable => "🛡️",
            MarketRegime::Transition => "🔄",
        }
    }
}

/// Market regime analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketRegimeResult {
    /// Market regime classification
    pub market_regime: MarketRegime,
    /// Confidence in the regime classification (0.0 to 1.0)
    pub regime_confidence: f64,
    /// Market context data
    pub market_context: MarketContext,
    /// Market volatility analysis
    pub volatility_analysis: VolatilityAnalysis,
    /// Market trend analysis
    pub trend_analysis: MarketTrendAnalysis,
    /// Market breadth analysis
    pub breadth_analysis: MarketBreadthAnalysis,
    /// Sector analysis
    pub sector_analysis: SectorAnalysis,
    /// Market sentiment indicators
    pub sentiment_indicators: SentimentIndicators,
    /// Risk assessment
    pub risk_assessment: MarketRiskAssessment,
    /// Analysis timestamp
    pub timestamp: DateTime<Utc>,
    /// Analysis metadata
    pub metadata: AnalysisMetadata,
}

/// Market context information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketContext {
    /// S&P 500 current price
    pub spy_price: Option<f64>,
    /// S&P 500 20-day change
    pub spy_20d_change: Option<f64>,
    /// S&P 500 50-day change
    pub spy_50d_change: Option<f64>,
    /// VIX (Volatility Index)
    pub vix: Option<f64>,
    /// Market breadth (advancing vs declining stocks)
    pub market_breadth: Option<f64>,
}

/// Market volatility analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolatilityAnalysis {
    /// Overall market volatility
    pub market_volatility: f64,
    /// Volatility percentile (0-100)
    pub volatility_percentile: f64,
    /// Volatility trend
    pub volatility_trend: VolatilityTrend,
}

/// Volatility trend
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VolatilityTrend {
    Increasing,
    Decreasing,
    Stable,
}

/// Market trend analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketTrendAnalysis {
    /// Short-term trend (1-5 days)
    pub short_term: TrendDirection,
    /// Medium-term trend (1-4 weeks)
    pub medium_term: TrendDirection,
    /// Long-term trend (1-3 months)
    pub long_term: TrendDirection,
    /// Trend strength (0-100)
    pub strength: f64,
    /// Trend consistency (0-100)
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
            TrendDirection::StrongBullish => 1.0,
            TrendDirection::Bullish => 0.5,
            TrendDirection::Neutral => 0.0,
            TrendDirection::Bearish => -0.5,
            TrendDirection::StrongBearish => -1.0,
        }
    }
}

/// Market breadth analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketBreadthAnalysis {
    /// Number of advancing stocks
    pub advancing_stocks: Option<i32>,
    /// Number of declining stocks
    pub declining_stocks: Option<i32>,
    /// Number of unchanged stocks
    pub unchanged_stocks: Option<i32>,
    /// Number of new highs
    pub new_highs: Option<i32>,
    /// Number of new lows
    pub new_lows: Option<i32>,
    /// Breadth ratio (advancing / declining)
    pub breadth_ratio: Option<f64>,
}

/// Sector analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectorAnalysis {
    /// Technology sector performance vs market
    pub technology_performance: Option<f64>,
    /// Healthcare sector performance vs market
    pub healthcare_performance: Option<f64>,
    /// Financial sector performance vs market
    pub financial_performance: Option<f64>,
    /// Energy sector performance vs market
    pub energy_performance: Option<f64>,
    /// Consumer sector performance vs market
    pub consumer_performance: Option<f64>,
    /// Leading sector
    pub leading_sector: Option<String>,
    /// Lagging sector
    pub lagging_sector: Option<String>,
}

/// Market sentiment indicators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentIndicators {
    /// Fear & Greed Index (0-100)
    pub fear_greed_index: Option<i32>,
    /// Put/Call ratio
    pub put_call_ratio: Option<f64>,
    /// Margin debt trend
    pub margin_debt_trend: Option<MarginDebtTrend>,
    /// Insider trading sentiment
    pub insider_sentiment: Option<InsiderSentiment>,
}

/// Margin debt trend
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MarginDebtTrend {
    Increasing,
    Decreasing,
    Stable,
}

/// Insider trading sentiment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InsiderSentiment {
    Bullish, // More buying than selling
    Bearish, // More selling than buying
    Neutral,
}

/// Market risk assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketRiskAssessment {
    /// Risk level classification
    pub risk_level: RiskLevel,
    /// Market risk score (0-100)
    pub risk_score: f64,
    /// Maximum drawdown risk
    pub max_drawdown_risk: f64,
    /// Correlation risk
    pub correlation_risk: f64,
    /// Liquidity risk
    pub liquidity_risk: f64,
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

impl std::fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiskLevel::Low => write!(f, "Low"),
            RiskLevel::Medium => write!(f, "Medium"),
            RiskLevel::High => write!(f, "High"),
            RiskLevel::VeryHigh => write!(f, "VeryHigh"),
        }
    }
}

/// Analysis metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisMetadata {
    /// Data sources used
    pub data_sources_used: Vec<String>,
    /// Analysis period in days
    pub analysis_period_days: i32,
    /// Computation time in milliseconds
    pub computation_time_ms: Option<i64>,
    /// API endpoints used
    pub api_endpoints_used: Vec<String>,
    /// Raw API responses (for debugging)
    pub raw_api_responses: Option<std::collections::HashMap<String, serde_json::Value>>,
}

/// Market regime detection parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegimeDetectionParams {
    /// Lookback period for trend analysis (days)
    pub trend_lookback: usize,
    /// Volatility threshold for regime classification
    pub volatility_threshold: f64,
    /// Trend strength threshold
    pub trend_threshold: f64,
    /// Minimum data points required
    pub min_data_points: usize,
}

impl Default for RegimeDetectionParams {
    fn default() -> Self {
        Self {
            trend_lookback: 50,
            volatility_threshold: 0.02, // 2% daily volatility
            trend_threshold: 0.05,      // 5% trend strength
            min_data_points: 30,
        }
    }
}

/// AI analysis for market regime
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatGPTMarketAnalysis {
    /// AI analysis of the market regime
    pub regime_analysis: String,
    /// AI market outlook
    pub market_outlook: String,
    /// AI risk assessment
    pub risk_assessment: String,
    /// Model used for analysis
    pub model_used: String,
    /// Analysis timestamp
    pub analysis_timestamp: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_market_regime_multipliers() {
        assert_eq!(MarketRegime::Bull.stock_analysis_multiplier(), 1.2);
        assert_eq!(MarketRegime::Bear.stock_analysis_multiplier(), 0.8);
        assert_eq!(MarketRegime::Sideways.stock_analysis_multiplier(), 1.0);
        assert_eq!(MarketRegime::Volatile.stock_analysis_multiplier(), 0.9);
        assert_eq!(MarketRegime::Stable.stock_analysis_multiplier(), 1.1);
        assert_eq!(MarketRegime::Transition.stock_analysis_multiplier(), 0.95);
    }

    #[test]
    fn test_trend_direction_scores() {
        assert_eq!(TrendDirection::StrongBullish.score(), 1.0);
        assert_eq!(TrendDirection::Bullish.score(), 0.5);
        assert_eq!(TrendDirection::Neutral.score(), 0.0);
        assert_eq!(TrendDirection::Bearish.score(), -0.5);
        assert_eq!(TrendDirection::StrongBearish.score(), -1.0);
    }

    #[test]
    fn test_risk_level_scores() {
        assert_eq!(RiskLevel::Low.score(), 25.0);
        assert_eq!(RiskLevel::Medium.score(), 50.0);
        assert_eq!(RiskLevel::High.score(), 75.0);
        assert_eq!(RiskLevel::VeryHigh.score(), 100.0);
    }

    #[test]
    fn test_market_regime_descriptions() {
        assert!(MarketRegime::Bull.description().contains("Bull Market"));
        assert!(MarketRegime::Bear.description().contains("Bear Market"));
        assert!(MarketRegime::Volatile.description().contains("Volatile Market"));
    }

    #[test]
    fn test_market_regime_emojis() {
        assert_eq!(MarketRegime::Bull.emoji(), "🐂");
        assert_eq!(MarketRegime::Bear.emoji(), "🐻");
        assert_eq!(MarketRegime::Volatile.emoji(), "⚡");
    }
}