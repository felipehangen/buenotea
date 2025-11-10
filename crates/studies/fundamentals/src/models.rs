// Data models for fundamentals analysis

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Trading signal generated from fundamentals score
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TradingSignal {
    /// Strong buy signal (Fundamentals >= 0.6)
    StrongBuy,
    /// Weak buy signal (Fundamentals >= 0.2)
    WeakBuy,
    /// Hold signal (Fundamentals between -0.2 and 0.2)
    Hold,
    /// Weak sell signal (Fundamentals >= -0.6)
    WeakSell,
    /// Strong sell signal (Fundamentals < -0.6)
    StrongSell,
}

impl std::fmt::Display for TradingSignal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TradingSignal::StrongBuy => write!(f, "StrongBuy"),
            TradingSignal::WeakBuy => write!(f, "WeakBuy"),
            TradingSignal::Hold => write!(f, "Hold"),
            TradingSignal::WeakSell => write!(f, "WeakSell"),
            TradingSignal::StrongSell => write!(f, "StrongSell"),
        }
    }
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
            TradingSignal::StrongBuy => "Strong Buy - Excellent fundamentals",
            TradingSignal::WeakBuy => "Weak Buy - Good fundamentals",
            TradingSignal::Hold => "Hold - Average fundamentals",
            TradingSignal::WeakSell => "Weak Sell - Poor fundamentals",
            TradingSignal::StrongSell => "Strong Sell - Very poor fundamentals",
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

/// Complete fundamentals analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundamentalsResult {
    /// Stock symbol analyzed
    pub symbol: String,
    /// Final fundamentals score between [-1, +1]
    pub fundamentals_score: f64,
    /// Trading signal generated from score
    pub trading_signal: TradingSignal,
    /// Individual component scores
    pub components: FundamentalsComponents,
    /// Financial metrics used in analysis
    pub metrics: FinancialMetrics,
    /// Warning and context flags
    pub flags: Vec<String>,
    /// Confidence score (0.0 to 1.0)
    pub confidence_score: f64,
    /// Timestamp of analysis
    pub timestamp: DateTime<Utc>,
    /// Additional metadata
    pub meta: FundamentalsMeta,
}

/// Individual component scores that make up the fundamentals score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundamentalsComponents {
    /// Profitability score (weight: 0.25)
    pub profitability: f64,
    /// Growth score (weight: 0.25)
    pub growth: f64,
    /// Valuation score (weight: 0.25)
    pub valuation: f64,
    /// Financial strength score (weight: 0.15)
    pub financial_strength: f64,
    /// Efficiency score (weight: 0.10)
    pub efficiency: f64,
}

impl FundamentalsComponents {
    /// Calculate the weighted fundamentals score from components
    /// Formula: 0.25×Profitability + 0.25×Growth + 0.25×Valuation + 0.15×Financial_Strength + 0.10×Efficiency
    pub fn calculate_fundamentals_score(&self) -> f64 {
        let weighted_score = 
            0.25 * self.profitability +
            0.25 * self.growth +
            0.25 * self.valuation +
            0.15 * self.financial_strength +
            0.10 * self.efficiency;
        
        // Ensure score is within [-1, +1] range
        weighted_score.max(-1.0).min(1.0)
    }

    /// Count how many components have valid (non-zero) scores
    pub fn valid_components_count(&self) -> usize {
        let mut count = 0;
        if self.profitability != 0.0 { count += 1; }
        if self.growth != 0.0 { count += 1; }
        if self.valuation != 0.0 { count += 1; }
        if self.financial_strength != 0.0 { count += 1; }
        if self.efficiency != 0.0 { count += 1; }
        count
    }
}

/// Financial metrics used in fundamentals analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialMetrics {
    /// Profitability metrics
    pub profitability: ProfitabilityMetrics,
    /// Growth metrics
    pub growth: GrowthMetrics,
    /// Valuation metrics
    pub valuation: ValuationMetrics,
    /// Financial strength metrics
    pub financial_strength: FinancialStrengthMetrics,
    /// Efficiency metrics
    pub efficiency: EfficiencyMetrics,
}

/// Profitability metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitabilityMetrics {
    pub roe: Option<f64>,                    // Return on Equity
    pub roa: Option<f64>,                    // Return on Assets
    pub roic: Option<f64>,                   // Return on Invested Capital
    pub net_profit_margin: Option<f64>,      // Net Profit Margin
    pub gross_profit_margin: Option<f64>,    // Gross Profit Margin
    pub operating_profit_margin: Option<f64>, // Operating Profit Margin
    pub ebitda_margin: Option<f64>,          // EBITDA Margin
}

/// Growth metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthMetrics {
    pub revenue_growth_yoy: Option<f64>,           // Year-over-year revenue growth
    pub revenue_growth_qoq: Option<f64>,           // Quarter-over-quarter revenue growth
    pub eps_growth_yoy: Option<f64>,               // Year-over-year EPS growth
    pub eps_growth_qoq: Option<f64>,               // Quarter-over-quarter EPS growth
    pub net_income_growth_yoy: Option<f64>,        // Year-over-year net income growth
    pub book_value_growth_yoy: Option<f64>,        // Year-over-year book value growth
    pub operating_cash_flow_growth_yoy: Option<f64>, // Year-over-year operating cash flow growth
}

/// Valuation metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuationMetrics {
    pub pe_ratio: Option<f64>,           // Price-to-Earnings ratio
    pub peg_ratio: Option<f64>,          // Price-to-Earnings Growth ratio
    pub ps_ratio: Option<f64>,           // Price-to-Sales ratio
    pub pb_ratio: Option<f64>,           // Price-to-Book ratio
    pub pcf_ratio: Option<f64>,          // Price-to-Cash Flow ratio
    pub ev_ebitda: Option<f64>,          // Enterprise Value to EBITDA
    pub ev_sales: Option<f64>,           // Enterprise Value to Sales
    pub pfcf_ratio: Option<f64>,         // Price-to-Free Cash Flow ratio
}

/// Financial strength metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialStrengthMetrics {
    pub debt_to_equity: Option<f64>,        // Debt-to-Equity ratio
    pub debt_to_assets: Option<f64>,        // Debt-to-Assets ratio
    pub current_ratio: Option<f64>,         // Current ratio
    pub quick_ratio: Option<f64>,           // Quick ratio
    pub interest_coverage: Option<f64>,     // Interest coverage ratio
    pub cash_to_debt: Option<f64>,          // Cash-to-Debt ratio
    pub equity_multiplier: Option<f64>,     // Equity multiplier
    pub altman_z_score: Option<f64>,        // Altman Z-Score
}

/// Efficiency metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EfficiencyMetrics {
    pub asset_turnover: Option<f64>,           // Asset turnover ratio
    pub inventory_turnover: Option<f64>,       // Inventory turnover
    pub receivables_turnover: Option<f64>,     // Receivables turnover
    pub payables_turnover: Option<f64>,        // Payables turnover
    pub working_capital_turnover: Option<f64>, // Working capital turnover
    pub days_sales_outstanding: Option<f64>,   // Days Sales Outstanding
    pub days_inventory_outstanding: Option<f64>, // Days Inventory Outstanding
    pub days_payables_outstanding: Option<f64>, // Days Payables Outstanding
}

/// Additional metadata about the fundamentals calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundamentalsMeta {
    /// Company sector
    pub sector: Option<String>,
    /// Company industry
    pub industry: Option<String>,
    /// Market cap category
    pub market_cap_category: Option<String>,
    /// Beta coefficient
    pub beta: Option<f64>,
    /// Dividend yield
    pub dividend_yield: Option<f64>,
    /// Payout ratio
    pub payout_ratio: Option<f64>,
    /// Shares outstanding
    pub shares_outstanding: Option<i64>,
    /// Market capitalization
    pub market_cap: Option<i64>,
    /// Enterprise value
    pub enterprise_value: Option<i64>,
    /// Computation time in milliseconds
    pub computation_time_ms: u64,
    /// Number of data points collected
    pub data_points_count: u32,
    /// Data freshness score (0.0 to 1.0)
    pub data_freshness: f64,
    /// Trend direction based on score
    pub trend_direction: f64,
}

impl Default for FundamentalsMeta {
    fn default() -> Self {
        Self {
            sector: None,
            industry: None,
            market_cap_category: None,
            beta: None,
            dividend_yield: None,
            payout_ratio: None,
            shares_outstanding: None,
            market_cap: None,
            enterprise_value: None,
            computation_time_ms: 0,
            data_points_count: 0,
            data_freshness: 0.0,
            trend_direction: 0.0,
        }
    }
}