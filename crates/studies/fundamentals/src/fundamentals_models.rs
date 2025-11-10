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

/// Helper function to create fundamentals record from FundamentalsResult with API tracking
/// This function lives in the fundamentals crate to avoid circular dependencies
pub fn create_fundamentals_record_with_tracking(
    result: crate::models::FundamentalsResult,
    api_urls: FundamentalsApiUrls,
    gpt_explanation: Option<String>,
    gpt_trading_suggestion: Option<String>,
) -> buenotea_infrastructure::fundamentals_models::CreateFundamentalsRecord {
    buenotea_infrastructure::fundamentals_models::CreateFundamentalsRecord {
        symbol: result.symbol.clone(),
        analysis_date: result.timestamp,
        fundamentals_score: result.fundamentals_score,
        trading_signal: result.trading_signal.to_string(),
        confidence_score: result.confidence_score,
        profitability_score: result.components.profitability,
        growth_score: result.components.growth,
        valuation_score: result.components.valuation,
        financial_strength_score: result.components.financial_strength,
        efficiency_score: result.components.efficiency,
        roe: result.metrics.profitability.roe,
        roa: result.metrics.profitability.roa,
        roic: result.metrics.profitability.roic,
        net_profit_margin: result.metrics.profitability.net_profit_margin,
        gross_profit_margin: result.metrics.profitability.gross_profit_margin,
        operating_profit_margin: result.metrics.profitability.operating_profit_margin,
        ebitda_margin: result.metrics.profitability.ebitda_margin,
        revenue_growth_yoy: result.metrics.growth.revenue_growth_yoy,
        revenue_growth_qoq: result.metrics.growth.revenue_growth_qoq,
        eps_growth_yoy: result.metrics.growth.eps_growth_yoy,
        eps_growth_qoq: result.metrics.growth.eps_growth_qoq,
        net_income_growth_yoy: result.metrics.growth.net_income_growth_yoy,
        book_value_growth_yoy: result.metrics.growth.book_value_growth_yoy,
        operating_cash_flow_growth_yoy: result.metrics.growth.operating_cash_flow_growth_yoy,
        pe_ratio: result.metrics.valuation.pe_ratio,
        peg_ratio: result.metrics.valuation.peg_ratio,
        ps_ratio: result.metrics.valuation.ps_ratio,
        pb_ratio: result.metrics.valuation.pb_ratio,
        pcf_ratio: result.metrics.valuation.pcf_ratio,
        ev_ebitda: result.metrics.valuation.ev_ebitda,
        ev_sales: result.metrics.valuation.ev_sales,
        pfcf_ratio: result.metrics.valuation.pfcf_ratio,
        debt_to_equity: result.metrics.financial_strength.debt_to_equity,
        debt_to_assets: result.metrics.financial_strength.debt_to_assets,
        current_ratio: result.metrics.financial_strength.current_ratio,
        quick_ratio: result.metrics.financial_strength.quick_ratio,
        interest_coverage: result.metrics.financial_strength.interest_coverage,
        cash_to_debt: result.metrics.financial_strength.cash_to_debt,
        equity_multiplier: result.metrics.financial_strength.equity_multiplier,
        altman_z_score: result.metrics.financial_strength.altman_z_score,
        asset_turnover: result.metrics.efficiency.asset_turnover,
        inventory_turnover: result.metrics.efficiency.inventory_turnover,
        receivables_turnover: result.metrics.efficiency.receivables_turnover,
        payables_turnover: result.metrics.efficiency.payables_turnover,
        working_capital_turnover: result.metrics.efficiency.working_capital_turnover,
        days_sales_outstanding: result.metrics.efficiency.days_sales_outstanding,
        days_inventory_outstanding: result.metrics.efficiency.days_inventory_outstanding,
        days_payables_outstanding: result.metrics.efficiency.days_payables_outstanding,
        sector: result.meta.sector.clone(),
        industry: result.meta.industry.clone(),
        market_cap_category: result.meta.market_cap_category.clone(),
        beta: result.meta.beta,
        dividend_yield: result.meta.dividend_yield,
        payout_ratio: result.meta.payout_ratio,
        shares_outstanding: result.meta.shares_outstanding,
        market_cap: result.meta.market_cap,
        enterprise_value: result.meta.enterprise_value,
        computation_time_ms: Some(result.meta.computation_time_ms as i32),
        data_points_count: Some(result.meta.data_points_count as i32),
        data_freshness: Some(result.meta.data_freshness),
        flags: Some(result.flags.clone()),
        profitability_api_url: api_urls.profitability_api_url,
        profitability_api_source: api_urls.profitability_api_source,
        profitability_data_available: Some(api_urls.profitability_data_available),
        profitability_raw_data: api_urls.profitability_raw_data,
        growth_api_url: api_urls.growth_api_url,
        growth_api_source: api_urls.growth_api_source,
        growth_data_available: Some(api_urls.growth_data_available),
        growth_raw_data: api_urls.growth_raw_data,
        valuation_api_url: api_urls.valuation_api_url,
        valuation_api_source: api_urls.valuation_api_source,
        valuation_data_available: Some(api_urls.valuation_data_available),
        valuation_raw_data: api_urls.valuation_raw_data,
        financial_strength_api_url: api_urls.financial_strength_api_url,
        financial_strength_api_source: api_urls.financial_strength_api_source,
        financial_strength_data_available: Some(api_urls.financial_strength_data_available),
        financial_strength_raw_data: api_urls.financial_strength_raw_data,
        efficiency_api_url: api_urls.efficiency_api_url,
        efficiency_api_source: api_urls.efficiency_api_source,
        efficiency_data_available: Some(api_urls.efficiency_data_available),
        efficiency_raw_data: api_urls.efficiency_raw_data,
        gpt_explanation,
        gpt_trading_suggestion,
    }
}