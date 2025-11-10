use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a stock in the S&P 500 with safety analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteListRecord {
    pub id: Option<i64>,
    pub symbol: String,
    pub company_name: String,
    pub sector: Option<String>,
    pub industry: Option<String>,
    pub market_cap: Option<i64>,
    pub current_price: Option<f64>,
    
    // Safety analysis results
    pub is_safe_to_trade: bool,
    pub safety_score: Option<f64>, // Range: 0.00 to 1.00
    pub safety_reasoning: Option<String>,
    
    // Basic financial health checks
    pub has_recent_earnings: bool,
    pub has_positive_revenue: bool,
    pub has_stable_price: bool,
    pub has_sufficient_volume: bool,
    pub has_analyst_coverage: bool,
    
    // Risk assessment
    pub risk_level: String, // Low, Medium, High, VeryHigh
    pub volatility_rating: Option<String>, // Low, Medium, High
    pub liquidity_rating: Option<String>, // Low, Medium, High
    
    // Data quality and source tracking
    pub data_source: String,
    pub last_updated: DateTime<Utc>,
    pub data_freshness_score: Option<f64>, // Range: 0.00 to 1.00
    
    // Analysis metadata
    pub analysis_date: DateTime<Utc>,
    pub analysis_duration_ms: Option<i32>,
    pub warning_flags: Vec<String>,
    pub missing_data_components: Vec<String>,
    
    // Raw API response data (JSON)
    pub raw_company_data: Option<serde_json::Value>,
    pub raw_financial_data: Option<serde_json::Value>,
    pub raw_price_data: Option<serde_json::Value>,
    
    // Metadata
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Record for inserting new invite list data (without auto-generated fields)
#[derive(Debug, Clone, Serialize)]
pub struct InviteListInsert {
    pub symbol: String,
    pub company_name: String,
    pub sector: Option<String>,
    pub industry: Option<String>,
    pub market_cap: Option<i64>,
    pub current_price: Option<f64>,
    pub is_safe_to_trade: bool,
    pub safety_score: Option<f64>,
    pub safety_reasoning: Option<String>,
    pub has_recent_earnings: bool,
    pub has_positive_revenue: bool,
    pub has_stable_price: bool,
    pub has_sufficient_volume: bool,
    pub has_analyst_coverage: bool,
    pub risk_level: String,
    pub volatility_rating: Option<String>,
    pub liquidity_rating: Option<String>,
    pub data_source: String,
    pub last_updated: DateTime<Utc>,
    pub data_freshness_score: Option<f64>,
    pub analysis_date: DateTime<Utc>,
    pub analysis_duration_ms: Option<i32>,
    pub warning_flags: Vec<String>,
    pub missing_data_components: Vec<String>,
    pub raw_company_data: Option<serde_json::Value>,
    pub raw_financial_data: Option<serde_json::Value>,
    pub raw_price_data: Option<serde_json::Value>,
}

impl From<InviteListRecord> for InviteListInsert {
    fn from(record: InviteListRecord) -> Self {
        Self {
            symbol: record.symbol,
            company_name: record.company_name,
            sector: record.sector,
            industry: record.industry,
            market_cap: record.market_cap,
            current_price: record.current_price,
            is_safe_to_trade: record.is_safe_to_trade,
            safety_score: record.safety_score,
            safety_reasoning: record.safety_reasoning,
            has_recent_earnings: record.has_recent_earnings,
            has_positive_revenue: record.has_positive_revenue,
            has_stable_price: record.has_stable_price,
            has_sufficient_volume: record.has_sufficient_volume,
            has_analyst_coverage: record.has_analyst_coverage,
            risk_level: record.risk_level,
            volatility_rating: record.volatility_rating,
            liquidity_rating: record.liquidity_rating,
            data_source: record.data_source,
            last_updated: record.last_updated,
            data_freshness_score: record.data_freshness_score,
            analysis_date: record.analysis_date,
            analysis_duration_ms: record.analysis_duration_ms,
            warning_flags: record.warning_flags,
            missing_data_components: record.missing_data_components,
            raw_company_data: record.raw_company_data,
            raw_financial_data: record.raw_financial_data,
            raw_price_data: record.raw_price_data,
        }
    }
}

/// S&P 500 stock data from API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SP500Stock {
    pub symbol: String,
    pub name: String,
    pub sector: Option<String>,
    pub industry: Option<String>,
    pub market_cap: Option<i64>,
    pub current_price: Option<f64>,
}

/// Safety analysis result for a stock
#[derive(Debug, Clone)]
pub struct SafetyAnalysis {
    pub is_safe_to_trade: bool,
    pub safety_score: f64,
    pub safety_reasoning: String,
    pub risk_level: String,
    pub volatility_rating: String,
    pub liquidity_rating: String,
    pub has_recent_earnings: bool,
    pub has_positive_revenue: bool,
    pub has_stable_price: bool,
    pub has_sufficient_volume: bool,
    pub has_analyst_coverage: bool,
    pub warning_flags: Vec<String>,
    pub missing_data_components: Vec<String>,
}

/// API configuration for data sources
#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub fmp_api_key: String,
    pub alpha_vantage_api_key: Option<String>,
    pub finnhub_api_key: Option<String>,
}

impl ApiConfig {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            fmp_api_key: std::env::var("FMP_API_KEY")?,
            alpha_vantage_api_key: std::env::var("ALPHA_VANTAGE_API_KEY").ok(),
            finnhub_api_key: std::env::var("FINNHUB_API_KEY").ok(),
        })
    }
}

/// Database configuration for invite list storage
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub supabase_url: String,
    pub supabase_api_key: String,
    pub table_name: String,
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            supabase_url: std::env::var("SUPABASE_URL")?,
            supabase_api_key: std::env::var("SUPABASE_API_KEY")?,
            table_name: "invite_list".to_string(),
        })
    }
}
