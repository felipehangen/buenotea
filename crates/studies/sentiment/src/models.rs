// Data models for sentiment analysis and QSS calculations

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Trading signal generated from QSS score
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TradingSignal {
    /// Strong buy signal (QSS >= 0.6)
    StrongBuy,
    /// Weak buy signal (QSS >= 0.2)
    WeakBuy,
    /// Hold signal (QSS between -0.2 and 0.2)
    Hold,
    /// Weak sell signal (QSS >= -0.6)
    WeakSell,
    /// Strong sell signal (QSS < -0.6)
    StrongSell,
}

impl TradingSignal {
    /// Get the position sizing recommendation for this signal
    pub fn position_size(&self) -> f64 {
        match self {
            TradingSignal::StrongBuy => 0.10,  // 10% of portfolio
            TradingSignal::WeakBuy => 0.05,    // 5% of portfolio
            TradingSignal::Hold => 0.0,        // No change
            TradingSignal::WeakSell => -0.05,  // Reduce by 5%
            TradingSignal::StrongSell => -0.10, // Reduce by 10%
        }
    }

    /// Get a human-readable description of the signal
    pub fn description(&self) -> &'static str {
        match self {
            TradingSignal::StrongBuy => "Strong Buy - High confidence bullish signal",
            TradingSignal::WeakBuy => "Weak Buy - Moderate bullish conditions",
            TradingSignal::Hold => "Hold - Mixed signals, maintain current position",
            TradingSignal::WeakSell => "Weak Sell - Moderate bearish conditions",
            TradingSignal::StrongSell => "Strong Sell - High confidence bearish signal",
        }
    }

    /// Get the emoji representation of the signal
    pub fn emoji(&self) -> &'static str {
        match self {
            TradingSignal::StrongBuy => "🟢",
            TradingSignal::WeakBuy => "🟡",
            TradingSignal::Hold => "⚪",
            TradingSignal::WeakSell => "🟠",
            TradingSignal::StrongSell => "🔴",
        }
    }
}

/// Complete QSS analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QSSResult {
    /// Stock symbol analyzed
    pub symbol: String,
    /// Final QSS score between [-1, +1]
    pub qss_score: f64,
    /// Generated trading signal
    pub trading_signal: TradingSignal,
    /// Individual component scores
    pub components: QSSComponents,
    /// Warning and context flags
    pub flags: Vec<String>,
    /// Confidence score (0.0 to 1.0)
    pub confidence_score: f64,
    /// Timestamp of analysis
    pub timestamp: DateTime<Utc>,
    /// Additional metadata
    pub meta: QSSMeta,
}

/// Individual component scores that make up the QSS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QSSComponents {
    /// Earnings revisions score (40% weight)
    pub earnings_revisions: f64,
    /// Relative strength score (30% weight)
    pub relative_strength: f64,
    /// Short interest score (20% weight)
    pub short_interest: f64,
    /// Options flow score (10% weight)
    pub options_flow: f64,
}

impl QSSComponents {
    /// Calculate the weighted QSS score from components
    pub fn calculate_qss(&self) -> f64 {
        0.40 * self.earnings_revisions +
        0.30 * self.relative_strength +
        0.20 * self.short_interest +
        0.10 * self.options_flow
    }

    /// Get the number of components with valid data
    pub fn valid_components_count(&self) -> usize {
        let mut count = 0;
        if self.earnings_revisions != 0.0 { count += 1; }
        if self.relative_strength != 0.0 { count += 1; }
        if self.short_interest != 0.0 { count += 1; }
        if self.options_flow != 0.0 { count += 1; }
        count
    }
}

/// Additional metadata about the QSS calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QSSMeta {
    /// Computation time in milliseconds
    pub computation_time_ms: u64,
    /// Number of data points used
    pub data_points_count: usize,
    /// Trend direction (-1 to 1)
    pub trend_direction: f64,
    /// Data freshness score (0 to 1)
    pub data_freshness: f64,
    // Additional detailed data
    pub rsi_14: Option<f64>,
    pub rsi_source: Option<String>,
    pub current_price: Option<f64>,
    pub price_15d_ago: Option<f64>,
    pub price_30d_ago: Option<f64>,
    pub return_15d: Option<f64>,
    pub return_30d: Option<f64>,
    pub current_eps_estimate: Option<f64>,
    pub previous_eps_estimate: Option<f64>,
    pub eps_change_percentage: Option<f64>,
    pub current_revenue_estimate: Option<i64>,
    pub previous_revenue_estimate: Option<i64>,
    pub revenue_change_percentage: Option<f64>,
    pub analyst_count: Option<i32>,
    pub market_benchmark_return: Option<f64>,
    pub sector_benchmark_return: Option<f64>,
    pub relative_to_market: Option<f64>,
    pub relative_to_sector: Option<f64>,
    pub volume_ratio: Option<f64>,
}

/// Historical price data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceData {
    /// Trading date
    pub date: DateTime<Utc>,
    /// Opening price
    pub open: f64,
    /// High price
    pub high: f64,
    /// Low price
    pub low: f64,
    /// Closing price
    pub close: f64,
    /// Trading volume
    pub volume: u64,
}

impl PriceData {
    /// Calculate the return from this price to another
    pub fn return_to(&self, other: &PriceData) -> f64 {
        (other.close - self.close) / self.close
    }

    /// Calculate the true range for ATR calculation
    pub fn true_range(&self, previous_close: f64) -> f64 {
        let high_low = self.high - self.low;
        let high_prev = (self.high - previous_close).abs();
        let low_prev = (self.low - previous_close).abs();
        high_low.max(high_prev).max(low_prev)
    }
}

/// Analyst estimate data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstimateData {
    /// Estimate date
    pub date: DateTime<Utc>,
    /// Earnings per share estimate
    pub eps: f64,
    /// Revenue estimate
    pub revenue: f64,
    /// Analyst recommendation (1=Strong Buy, 0=Hold, -1=Strong Sell)
    pub recommendation: f64,
    /// Number of analysts providing estimates
    pub analyst_count: u32,
}

impl EstimateData {
    /// Calculate the change from another estimate
    pub fn change_from(&self, other: &EstimateData) -> EstimateChange {
        EstimateChange {
            eps_change: (self.eps - other.eps) / other.eps,
            revenue_change: (self.revenue - other.revenue) / other.revenue,
            recommendation_change: self.recommendation - other.recommendation,
        }
    }
}

/// Changes in analyst estimates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstimateChange {
    /// EPS change percentage
    pub eps_change: f64,
    /// Revenue change percentage
    pub revenue_change: f64,
    /// Recommendation change
    pub recommendation_change: f64,
}

/// Short interest data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortInterestData {
    /// Trading date
    pub date: DateTime<Utc>,
    /// Short selling volume
    pub short_volume: u64,
    /// Total trading volume
    pub total_volume: u64,
    /// Short interest ratio
    pub short_ratio: f64,
}

impl ShortInterestData {
    /// Create from volume data
    pub fn from_volumes(date: DateTime<Utc>, short_volume: u64, total_volume: u64) -> Self {
        let short_ratio = if total_volume > 0 {
            short_volume as f64 / total_volume as f64
        } else {
            0.0
        };

        Self {
            date,
            short_volume,
            total_volume,
            short_ratio,
        }
    }
}

/// Options flow data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsFlowData {
    /// Trading date
    pub date: DateTime<Utc>,
    /// Call option volume
    pub call_volume: u64,
    /// Put option volume
    pub put_volume: u64,
    /// Call premium volume
    pub call_premium: f64,
    /// Put premium volume
    pub put_premium: f64,
    /// Days to expiration
    pub dte: u32,
}

impl OptionsFlowData {
    /// Calculate the put/call ratio
    pub fn put_call_ratio(&self) -> f64 {
        if self.call_volume > 0 {
            self.put_volume as f64 / self.call_volume as f64
        } else {
            0.0
        }
    }

    /// Calculate the premium skew
    pub fn premium_skew(&self) -> f64 {
        let total_premium = self.call_premium + self.put_premium;
        if total_premium > 0.0 {
            self.call_premium / total_premium
        } else {
            0.5 // Neutral if no premium data
        }
    }
}

/// Technical indicator values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalIndicators {
    /// 14-day Relative Strength Index
    pub rsi_14: f64,
    /// 14-day Average True Range
    pub atr_14: f64,
    /// Volume ratio (recent vs average)
    pub volume_ratio: f64,
    /// Moving average convergence divergence
    pub macd: f64,
    /// MACD signal line
    pub macd_signal: f64,
}

/// Performance comparison data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceComparison {
    /// 1-month performance vs SPY
    pub vs_spy_1m: f64,
    /// 3-month performance vs SPY
    pub vs_spy_3m: f64,
    /// 1-month performance vs sector
    pub vs_sector_1m: f64,
    /// 3-month performance vs sector
    pub vs_sector_3m: f64,
}

/// Data quality flags
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataQualityFlag {
    /// Earnings announcement within 2 days
    EarningsWindow,
    /// Insufficient analyst estimates
    NoEstimates,
    /// High dispersion in estimates
    LowConfidence,
    /// Less than minimum required data
    InsufficientData,
    /// No short interest data available
    NoShortData,
    /// No options flow data available
    NoOptionsData,
    /// Data is stale or outdated
    StaleData,
    /// API rate limit exceeded
    RateLimited,
    /// Network connectivity issues
    NetworkIssues,
    /// Options flow contradicts price trend
    FlowConflict,
}

impl DataQualityFlag {
    /// Get a human-readable description of the flag
    pub fn description(&self) -> &'static str {
        match self {
            DataQualityFlag::EarningsWindow => "Earnings announcement within 2 days",
            DataQualityFlag::NoEstimates => "Insufficient analyst estimates available",
            DataQualityFlag::LowConfidence => "High dispersion in analyst estimates",
            DataQualityFlag::InsufficientData => "Less than minimum required data points",
            DataQualityFlag::NoShortData => "No short interest data available",
            DataQualityFlag::NoOptionsData => "No options flow data available",
            DataQualityFlag::StaleData => "Data is stale or outdated",
            DataQualityFlag::RateLimited => "API rate limit exceeded",
            DataQualityFlag::NetworkIssues => "Network connectivity issues",
            DataQualityFlag::FlowConflict => "Options flow contradicts price trend",
        }
    }

    /// Get the severity level of the flag
    pub fn severity(&self) -> Severity {
        match self {
            DataQualityFlag::EarningsWindow => Severity::Info,
            DataQualityFlag::NoEstimates => Severity::Warning,
            DataQualityFlag::LowConfidence => Severity::Warning,
            DataQualityFlag::InsufficientData => Severity::Error,
            DataQualityFlag::NoShortData => Severity::Warning,
            DataQualityFlag::NoOptionsData => Severity::Warning,
            DataQualityFlag::StaleData => Severity::Warning,
            DataQualityFlag::RateLimited => Severity::Error,
            DataQualityFlag::NetworkIssues => Severity::Error,
            DataQualityFlag::FlowConflict => Severity::Warning,
        }
    }
}

/// Severity levels for data quality flags
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Severity {
    /// Informational only
    Info,
    /// Warning - may affect accuracy
    Warning,
    /// Error - critical issue
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_trading_signal_position_size() {
        assert_eq!(TradingSignal::StrongBuy.position_size(), 0.10);
        assert_eq!(TradingSignal::WeakBuy.position_size(), 0.05);
        assert_eq!(TradingSignal::Hold.position_size(), 0.0);
        assert_eq!(TradingSignal::WeakSell.position_size(), -0.05);
        assert_eq!(TradingSignal::StrongSell.position_size(), -0.10);
    }

    #[test]
    fn test_qss_components_calculation() {
        let components = QSSComponents {
            earnings_revisions: 0.5,
            relative_strength: 0.3,
            short_interest: -0.2,
            options_flow: 0.1,
        };

        let expected = 0.40 * 0.5 + 0.30 * 0.3 + 0.20 * (-0.2) + 0.10 * 0.1;
        assert_eq!(components.calculate_qss(), expected);
    }

    #[test]
    fn test_estimate_change_calculation() {
        let old = EstimateData {
            date: Utc::now(),
            eps: 1.0,
            revenue: 100.0,
            recommendation: 0.5,
            analyst_count: 5,
        };

        let new = EstimateData {
            date: Utc::now(),
            eps: 1.1,
            revenue: 105.0,
            recommendation: 0.7,
            analyst_count: 6,
        };

        let change = new.change_from(&old);
        assert!((change.eps_change - 0.1).abs() < 0.001);
        assert!((change.revenue_change - 0.05).abs() < 0.001);
        assert!((change.recommendation_change - 0.2).abs() < 0.001);
    }

    #[test]
    fn test_short_interest_ratio_calculation() {
        let data = ShortInterestData::from_volumes(Utc::now(), 1000, 10000);
        assert_eq!(data.short_ratio, 0.1);

        let data_zero = ShortInterestData::from_volumes(Utc::now(), 1000, 0);
        assert_eq!(data_zero.short_ratio, 0.0);
    }

    #[test]
    fn test_options_premium_skew() {
        let data = OptionsFlowData {
            date: Utc::now(),
            call_volume: 1000,
            put_volume: 800,
            call_premium: 1000.0,
            put_premium: 600.0,
            dte: 30,
        };

        assert_eq!(data.put_call_ratio(), 0.8);
        assert_eq!(data.premium_skew(), 1000.0 / 1600.0);
    }
}
