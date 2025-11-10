// Database models for Fundamentals Analysis
// Stores comprehensive financial analysis in the database
// Note: These are database-specific types. Conversion from fundamentals crate types happens in the fundamentals crate.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Fundamentals database record (read from database)
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Fundamentals record creation request (for creating new records)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFundamentalsRecord {
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

/// Fundamentals record insert (for database inserts, without auto-generated fields)
#[derive(Debug, Clone, Serialize)]
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

impl From<CreateFundamentalsRecord> for FundamentalsInsert {
    fn from(record: CreateFundamentalsRecord) -> Self {
        Self {
            symbol: record.symbol,
            analysis_date: record.analysis_date,
            fundamentals_score: record.fundamentals_score,
            trading_signal: record.trading_signal,
            confidence_score: record.confidence_score,
            profitability_score: record.profitability_score,
            growth_score: record.growth_score,
            valuation_score: record.valuation_score,
            financial_strength_score: record.financial_strength_score,
            efficiency_score: record.efficiency_score,
            roe: record.roe,
            roa: record.roa,
            roic: record.roic,
            net_profit_margin: record.net_profit_margin,
            gross_profit_margin: record.gross_profit_margin,
            operating_profit_margin: record.operating_profit_margin,
            ebitda_margin: record.ebitda_margin,
            revenue_growth_yoy: record.revenue_growth_yoy,
            revenue_growth_qoq: record.revenue_growth_qoq,
            eps_growth_yoy: record.eps_growth_yoy,
            eps_growth_qoq: record.eps_growth_qoq,
            net_income_growth_yoy: record.net_income_growth_yoy,
            book_value_growth_yoy: record.book_value_growth_yoy,
            operating_cash_flow_growth_yoy: record.operating_cash_flow_growth_yoy,
            pe_ratio: record.pe_ratio,
            peg_ratio: record.peg_ratio,
            ps_ratio: record.ps_ratio,
            pb_ratio: record.pb_ratio,
            pcf_ratio: record.pcf_ratio,
            ev_ebitda: record.ev_ebitda,
            ev_sales: record.ev_sales,
            pfcf_ratio: record.pfcf_ratio,
            debt_to_equity: record.debt_to_equity,
            debt_to_assets: record.debt_to_assets,
            current_ratio: record.current_ratio,
            quick_ratio: record.quick_ratio,
            interest_coverage: record.interest_coverage,
            cash_to_debt: record.cash_to_debt,
            equity_multiplier: record.equity_multiplier,
            altman_z_score: record.altman_z_score,
            asset_turnover: record.asset_turnover,
            inventory_turnover: record.inventory_turnover,
            receivables_turnover: record.receivables_turnover,
            payables_turnover: record.payables_turnover,
            working_capital_turnover: record.working_capital_turnover,
            days_sales_outstanding: record.days_sales_outstanding,
            days_inventory_outstanding: record.days_inventory_outstanding,
            days_payables_outstanding: record.days_payables_outstanding,
            sector: record.sector,
            industry: record.industry,
            market_cap_category: record.market_cap_category,
            beta: record.beta,
            dividend_yield: record.dividend_yield,
            payout_ratio: record.payout_ratio,
            shares_outstanding: record.shares_outstanding,
            market_cap: record.market_cap,
            enterprise_value: record.enterprise_value,
            computation_time_ms: record.computation_time_ms,
            data_points_count: record.data_points_count,
            data_freshness: record.data_freshness,
            flags: record.flags,
            profitability_api_url: record.profitability_api_url,
            profitability_api_source: record.profitability_api_source,
            profitability_data_available: record.profitability_data_available,
            profitability_raw_data: record.profitability_raw_data,
            growth_api_url: record.growth_api_url,
            growth_api_source: record.growth_api_source,
            growth_data_available: record.growth_data_available,
            growth_raw_data: record.growth_raw_data,
            valuation_api_url: record.valuation_api_url,
            valuation_api_source: record.valuation_api_source,
            valuation_data_available: record.valuation_data_available,
            valuation_raw_data: record.valuation_raw_data,
            financial_strength_api_url: record.financial_strength_api_url,
            financial_strength_api_source: record.financial_strength_api_source,
            financial_strength_data_available: record.financial_strength_data_available,
            financial_strength_raw_data: record.financial_strength_raw_data,
            efficiency_api_url: record.efficiency_api_url,
            efficiency_api_source: record.efficiency_api_source,
            efficiency_data_available: record.efficiency_data_available,
            efficiency_raw_data: record.efficiency_raw_data,
            gpt_explanation: record.gpt_explanation,
            gpt_trading_suggestion: record.gpt_trading_suggestion,
        }
    }
}
