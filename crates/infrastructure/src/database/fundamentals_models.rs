// Database models for fundamentals analysis storage

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Database record for fundamentals analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundamentalsRecord {
    pub id: Option<i64>,
    pub symbol: String,
    pub analysis_date: DateTime<Utc>,
    
    // Core scoring data
    pub fundamentals_score: f64,
    pub trading_signal: String,
    pub confidence_score: f64,
    
    // Component scores
    pub profitability_score: f64,
    pub growth_score: f64,
    pub valuation_score: f64,
    pub financial_strength_score: f64,
    pub efficiency_score: f64,
    
    // Profitability metrics
    pub roe: Option<f64>,
    pub roa: Option<f64>,
    pub roic: Option<f64>,
    pub net_profit_margin: Option<f64>,
    pub gross_profit_margin: Option<f64>,
    pub operating_profit_margin: Option<f64>,
    pub ebitda_margin: Option<f64>,
    
    // Growth metrics
    pub revenue_growth_yoy: Option<f64>,
    pub revenue_growth_qoq: Option<f64>,
    pub eps_growth_yoy: Option<f64>,
    pub eps_growth_qoq: Option<f64>,
    pub net_income_growth_yoy: Option<f64>,
    pub book_value_growth_yoy: Option<f64>,
    pub operating_cash_flow_growth_yoy: Option<f64>,
    
    // Valuation metrics
    pub pe_ratio: Option<f64>,
    pub peg_ratio: Option<f64>,
    pub ps_ratio: Option<f64>,
    pub pb_ratio: Option<f64>,
    pub pcf_ratio: Option<f64>,
    pub ev_ebitda: Option<f64>,
    pub ev_sales: Option<f64>,
    pub pfcf_ratio: Option<f64>,
    
    // Financial strength metrics
    pub debt_to_equity: Option<f64>,
    pub debt_to_assets: Option<f64>,
    pub current_ratio: Option<f64>,
    pub quick_ratio: Option<f64>,
    pub interest_coverage: Option<f64>,
    pub cash_to_debt: Option<f64>,
    pub equity_multiplier: Option<f64>,
    pub altman_z_score: Option<f64>,
    
    // Efficiency metrics
    pub asset_turnover: Option<f64>,
    pub inventory_turnover: Option<f64>,
    pub receivables_turnover: Option<f64>,
    pub payables_turnover: Option<f64>,
    pub working_capital_turnover: Option<f64>,
    pub days_sales_outstanding: Option<f64>,
    pub days_inventory_outstanding: Option<f64>,
    pub days_payables_outstanding: Option<f64>,
    
    // Company metadata
    pub sector: Option<String>,
    pub industry: Option<String>,
    pub market_cap_category: Option<String>,
    pub beta: Option<f64>,
    pub dividend_yield: Option<f64>,
    pub payout_ratio: Option<f64>,
    pub shares_outstanding: Option<i64>,
    pub market_cap: Option<i64>,
    pub enterprise_value: Option<i64>,
    
    // Analysis metadata
    pub computation_time_ms: Option<i32>,
    pub data_points_count: Option<i32>,
    pub data_freshness: Option<f64>,
    pub flags: Option<Vec<String>>,
    
    // API tracking
    pub profitability_api_url: Option<String>,
    pub profitability_api_source: Option<String>,
    pub profitability_data_available: Option<bool>,
    pub profitability_raw_data: Option<serde_json::Value>,
    
    pub growth_api_url: Option<String>,
    pub growth_api_source: Option<String>,
    pub growth_data_available: Option<bool>,
    pub growth_raw_data: Option<serde_json::Value>,
    
    pub valuation_api_url: Option<String>,
    pub valuation_api_source: Option<String>,
    pub valuation_data_available: Option<bool>,
    pub valuation_raw_data: Option<serde_json::Value>,
    
    pub financial_strength_api_url: Option<String>,
    pub financial_strength_api_source: Option<String>,
    pub financial_strength_data_available: Option<bool>,
    pub financial_strength_raw_data: Option<serde_json::Value>,
    
    pub efficiency_api_url: Option<String>,
    pub efficiency_api_source: Option<String>,
    pub efficiency_data_available: Option<bool>,
    pub efficiency_raw_data: Option<serde_json::Value>,
    
    // AI Analysis
    pub gpt_explanation: Option<String>,
    pub gpt_trading_suggestion: Option<String>,
    
    // Timestamps
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Insert record for fundamentals analysis (omits auto-generated fields)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundamentalsInsert {
    pub symbol: String,
    pub analysis_date: DateTime<Utc>,
    
    // Core scoring data
    pub fundamentals_score: f64,
    pub trading_signal: String,
    pub confidence_score: f64,
    
    // Component scores
    pub profitability_score: f64,
    pub growth_score: f64,
    pub valuation_score: f64,
    pub financial_strength_score: f64,
    pub efficiency_score: f64,
    
    // Profitability metrics
    pub roe: Option<f64>,
    pub roa: Option<f64>,
    pub roic: Option<f64>,
    pub net_profit_margin: Option<f64>,
    pub gross_profit_margin: Option<f64>,
    pub operating_profit_margin: Option<f64>,
    pub ebitda_margin: Option<f64>,
    
    // Growth metrics
    pub revenue_growth_yoy: Option<f64>,
    pub revenue_growth_qoq: Option<f64>,
    pub eps_growth_yoy: Option<f64>,
    pub eps_growth_qoq: Option<f64>,
    pub net_income_growth_yoy: Option<f64>,
    pub book_value_growth_yoy: Option<f64>,
    pub operating_cash_flow_growth_yoy: Option<f64>,
    
    // Valuation metrics
    pub pe_ratio: Option<f64>,
    pub peg_ratio: Option<f64>,
    pub ps_ratio: Option<f64>,
    pub pb_ratio: Option<f64>,
    pub pcf_ratio: Option<f64>,
    pub ev_ebitda: Option<f64>,
    pub ev_sales: Option<f64>,
    pub pfcf_ratio: Option<f64>,
    
    // Financial strength metrics
    pub debt_to_equity: Option<f64>,
    pub debt_to_assets: Option<f64>,
    pub current_ratio: Option<f64>,
    pub quick_ratio: Option<f64>,
    pub interest_coverage: Option<f64>,
    pub cash_to_debt: Option<f64>,
    pub equity_multiplier: Option<f64>,
    pub altman_z_score: Option<f64>,
    
    // Efficiency metrics
    pub asset_turnover: Option<f64>,
    pub inventory_turnover: Option<f64>,
    pub receivables_turnover: Option<f64>,
    pub payables_turnover: Option<f64>,
    pub working_capital_turnover: Option<f64>,
    pub days_sales_outstanding: Option<f64>,
    pub days_inventory_outstanding: Option<f64>,
    pub days_payables_outstanding: Option<f64>,
    
    // Company metadata
    pub sector: Option<String>,
    pub industry: Option<String>,
    pub market_cap_category: Option<String>,
    pub beta: Option<f64>,
    pub dividend_yield: Option<f64>,
    pub payout_ratio: Option<f64>,
    pub shares_outstanding: Option<i64>,
    pub market_cap: Option<i64>,
    pub enterprise_value: Option<i64>,
    
    // Analysis metadata
    pub computation_time_ms: Option<i32>,
    pub data_points_count: Option<i32>,
    pub data_freshness: Option<f64>,
    pub flags: Option<Vec<String>>,
    
    // API tracking
    pub profitability_api_url: Option<String>,
    pub profitability_api_source: Option<String>,
    pub profitability_data_available: Option<bool>,
    pub profitability_raw_data: Option<serde_json::Value>,
    
    pub growth_api_url: Option<String>,
    pub growth_api_source: Option<String>,
    pub growth_data_available: Option<bool>,
    pub growth_raw_data: Option<serde_json::Value>,
    
    pub valuation_api_url: Option<String>,
    pub valuation_api_source: Option<String>,
    pub valuation_data_available: Option<bool>,
    pub valuation_raw_data: Option<serde_json::Value>,
    
    pub financial_strength_api_url: Option<String>,
    pub financial_strength_api_source: Option<String>,
    pub financial_strength_data_available: Option<bool>,
    pub financial_strength_raw_data: Option<serde_json::Value>,
    
    pub efficiency_api_url: Option<String>,
    pub efficiency_api_source: Option<String>,
    pub efficiency_data_available: Option<bool>,
    pub efficiency_raw_data: Option<serde_json::Value>,
    
    // AI Analysis
    pub gpt_explanation: Option<String>,
    pub gpt_trading_suggestion: Option<String>,
}

/// API URLs and metadata for fundamentals analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundamentalsApiUrls {
    pub profitability_api_url: Option<String>,
    pub profitability_api_source: Option<String>,
    pub profitability_data_available: bool,
    pub profitability_raw_data: Option<serde_json::Value>,
    
    pub growth_api_url: Option<String>,
    pub growth_api_source: Option<String>,
    pub growth_data_available: bool,
    pub growth_raw_data: Option<serde_json::Value>,
    
    pub valuation_api_url: Option<String>,
    pub valuation_api_source: Option<String>,
    pub valuation_data_available: bool,
    pub valuation_raw_data: Option<serde_json::Value>,
    
    pub financial_strength_api_url: Option<String>,
    pub financial_strength_api_source: Option<String>,
    pub financial_strength_data_available: bool,
    pub financial_strength_raw_data: Option<serde_json::Value>,
    
    pub efficiency_api_url: Option<String>,
    pub efficiency_api_source: Option<String>,
    pub efficiency_data_available: bool,
    pub efficiency_raw_data: Option<serde_json::Value>,
}

impl Default for FundamentalsApiUrls {
    fn default() -> Self {
        Self {
            profitability_api_url: None,
            profitability_api_source: None,
            profitability_data_available: false,
            profitability_raw_data: None,
            growth_api_url: None,
            growth_api_source: None,
            growth_data_available: false,
            growth_raw_data: None,
            valuation_api_url: None,
            valuation_api_source: None,
            valuation_data_available: false,
            valuation_raw_data: None,
            financial_strength_api_url: None,
            financial_strength_api_source: None,
            financial_strength_data_available: false,
            financial_strength_raw_data: None,
            efficiency_api_url: None,
            efficiency_api_source: None,
            efficiency_data_available: false,
            efficiency_raw_data: None,
        }
    }
}