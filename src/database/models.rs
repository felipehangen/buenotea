use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentRecord {
    pub id: Option<i64>,
    pub symbol: String,
    pub analysis_date: DateTime<Utc>,
    
    // Final sentiment score and signal
    pub qss_score: f64,
    pub trading_signal: String,
    pub confidence_score: f64,
    
    // Component scores (weighted components of QSS)
    pub earnings_revisions_score: f64,
    pub relative_strength_score: f64,
    pub short_interest_score: f64,
    pub options_flow_score: f64,
    
    // Component weights (for reference)
    pub earnings_weight: f64,
    pub relative_strength_weight: f64,
    pub short_interest_weight: f64,
    pub options_flow_weight: f64,
    
    // API endpoint information
    pub earnings_api_url: Option<String>,
    pub earnings_api_source: Option<String>,
    pub earnings_data_available: bool,
    
    pub price_data_api_url: Option<String>,
    pub price_data_api_source: Option<String>,
    pub price_data_available: bool,
    
    pub short_interest_api_url: Option<String>,
    pub short_interest_api_source: Option<String>,
    pub short_interest_data_available: bool,
    
    pub options_flow_api_url: Option<String>,
    pub options_flow_api_source: Option<String>,
    pub options_flow_data_available: bool,
    
    // Raw API response data (JSON)
    pub earnings_raw_data: Option<serde_json::Value>,
    pub price_data_raw_data: Option<serde_json::Value>,
    pub short_interest_raw_data: Option<serde_json::Value>,
    pub options_flow_raw_data: Option<serde_json::Value>,
    
    // Data quality metrics
    pub data_coverage_percentage: f64,
    pub computation_time_ms: i32,
    pub data_points_count: i32,
    pub trend_direction: f64,
    pub data_freshness_score: f64,
    
    // Warning flags and context
    pub warning_flags: Vec<String>,
    pub missing_data_components: Vec<String>,
    
    // GPT-generated explanation
    pub gpt_explanation: String,
    pub gpt_explanation_timestamp: Option<DateTime<Utc>>,
    
    // Technical indicators
    pub rsi_14: Option<f64>,
    pub rsi_source: Option<String>,
    
    // Market context
    pub market_benchmark_return: Option<f64>,
    pub sector_benchmark_return: Option<f64>,
    pub relative_to_market: Option<f64>,
    pub relative_to_sector: Option<f64>,
    
    // Earnings data
    pub current_eps_estimate: Option<f64>,
    pub previous_eps_estimate: Option<f64>,
    pub eps_change_percentage: Option<f64>,
    pub current_revenue_estimate: Option<i64>,
    pub previous_revenue_estimate: Option<i64>,
    pub revenue_change_percentage: Option<f64>,
    pub analyst_count: Option<i32>,
    
    // Price data
    pub current_price: Option<f64>,
    pub price_15d_ago: Option<f64>,
    pub price_30d_ago: Option<f64>,
    pub return_15d: Option<f64>,
    pub return_30d: Option<f64>,
    pub volume_ratio: Option<f64>,
    
    // Metadata
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl SentimentRecord {
    pub fn from_qss_result(
        symbol: String,
        qss_result: &crate::sentiment::models::QSSResult,
        api_urls: &ApiUrls,
        gpt_explanation: String,
    ) -> Self {
        Self {
            id: None,
            symbol,
            analysis_date: qss_result.timestamp,
            qss_score: qss_result.qss_score,
            trading_signal: format!("{:?}", qss_result.trading_signal),
            confidence_score: qss_result.confidence_score,
            earnings_revisions_score: qss_result.components.earnings_revisions,
            relative_strength_score: qss_result.components.relative_strength,
            short_interest_score: qss_result.components.short_interest,
            options_flow_score: qss_result.components.options_flow,
            earnings_weight: 0.40,
            relative_strength_weight: 0.30,
            short_interest_weight: 0.20,
            options_flow_weight: 0.10,
            earnings_api_url: api_urls.earnings_api_url.clone(),
            earnings_api_source: api_urls.earnings_api_source.clone(),
            earnings_data_available: api_urls.earnings_data_available,
            price_data_api_url: api_urls.price_data_api_url.clone(),
            price_data_api_source: api_urls.price_data_api_source.clone(),
            price_data_available: api_urls.price_data_available,
            short_interest_api_url: api_urls.short_interest_api_url.clone(),
            short_interest_api_source: api_urls.short_interest_api_source.clone(),
            short_interest_data_available: api_urls.short_interest_data_available,
            options_flow_api_url: api_urls.options_flow_api_url.clone(),
            options_flow_api_source: api_urls.options_flow_api_source.clone(),
            options_flow_data_available: api_urls.options_flow_data_available,
            earnings_raw_data: api_urls.earnings_raw_data.clone(),
            price_data_raw_data: api_urls.price_data_raw_data.clone(),
            short_interest_raw_data: api_urls.short_interest_raw_data.clone(),
            options_flow_raw_data: api_urls.options_flow_raw_data.clone(),
            data_coverage_percentage: 75.0, // Default value - could be calculated
            computation_time_ms: qss_result.meta.computation_time_ms as i32,
            data_points_count: qss_result.meta.data_points_count as i32,
            trend_direction: qss_result.meta.trend_direction,
            data_freshness_score: qss_result.meta.data_freshness,
            warning_flags: qss_result.flags.clone(),
            missing_data_components: vec![], // Default empty - could be calculated from flags
            gpt_explanation,
            gpt_explanation_timestamp: Some(Utc::now()),
            rsi_14: qss_result.meta.rsi_14,
            rsi_source: qss_result.meta.rsi_source.clone(),
            market_benchmark_return: qss_result.meta.market_benchmark_return,
            sector_benchmark_return: qss_result.meta.sector_benchmark_return,
            relative_to_market: qss_result.meta.relative_to_market,
            relative_to_sector: qss_result.meta.relative_to_sector,
            current_eps_estimate: qss_result.meta.current_eps_estimate,
            previous_eps_estimate: qss_result.meta.previous_eps_estimate,
            eps_change_percentage: qss_result.meta.eps_change_percentage,
            current_revenue_estimate: qss_result.meta.current_revenue_estimate,
            previous_revenue_estimate: qss_result.meta.previous_revenue_estimate,
            revenue_change_percentage: qss_result.meta.revenue_change_percentage,
            analyst_count: qss_result.meta.analyst_count,
            current_price: qss_result.meta.current_price,
            price_15d_ago: qss_result.meta.price_15d_ago,
            price_30d_ago: qss_result.meta.price_30d_ago,
            return_15d: qss_result.meta.return_15d,
            return_30d: qss_result.meta.return_30d,
            volume_ratio: qss_result.meta.volume_ratio,
            created_at: None,
            updated_at: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApiUrls {
    pub earnings_api_url: Option<String>,
    pub earnings_api_source: Option<String>,
    pub earnings_data_available: bool,
    pub earnings_raw_data: Option<serde_json::Value>,
    
    pub price_data_api_url: Option<String>,
    pub price_data_api_source: Option<String>,
    pub price_data_available: bool,
    pub price_data_raw_data: Option<serde_json::Value>,
    
    pub short_interest_api_url: Option<String>,
    pub short_interest_api_source: Option<String>,
    pub short_interest_data_available: bool,
    pub short_interest_raw_data: Option<serde_json::Value>,
    
    pub options_flow_api_url: Option<String>,
    pub options_flow_api_source: Option<String>,
    pub options_flow_data_available: bool,
    pub options_flow_raw_data: Option<serde_json::Value>,
}

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
            table_name: "sentiment".to_string(),
        })
    }
}

/// Record for inserting new sentiment data (without auto-generated fields)
#[derive(Debug, Clone, Serialize)]
pub struct SentimentInsert {
    pub symbol: String,
    pub analysis_date: DateTime<Utc>,
    pub qss_score: f64,
    pub trading_signal: String,
    pub confidence_score: f64,
    pub earnings_revisions_score: f64,
    pub relative_strength_score: f64,
    pub short_interest_score: f64,
    pub options_flow_score: f64,
    pub earnings_weight: f64,
    pub relative_strength_weight: f64,
    pub short_interest_weight: f64,
    pub options_flow_weight: f64,
    pub earnings_api_url: Option<String>,
    pub earnings_api_source: Option<String>,
    pub earnings_data_available: bool,
    pub price_data_api_url: Option<String>,
    pub price_data_api_source: Option<String>,
    pub price_data_available: bool,
    pub short_interest_api_url: Option<String>,
    pub short_interest_api_source: Option<String>,
    pub short_interest_data_available: bool,
    pub options_flow_api_url: Option<String>,
    pub options_flow_api_source: Option<String>,
    pub options_flow_data_available: bool,
    pub earnings_raw_data: Option<serde_json::Value>,
    pub price_data_raw_data: Option<serde_json::Value>,
    pub short_interest_raw_data: Option<serde_json::Value>,
    pub options_flow_raw_data: Option<serde_json::Value>,
    pub data_coverage_percentage: f64,
    pub computation_time_ms: i32,
    pub data_points_count: i32,
    pub trend_direction: f64,
    pub data_freshness_score: f64,
    pub warning_flags: Vec<String>,
    pub missing_data_components: Vec<String>,
    pub gpt_explanation: String,
    pub gpt_explanation_timestamp: Option<DateTime<Utc>>,
    pub rsi_14: Option<f64>,
    pub rsi_source: Option<String>,
    pub market_benchmark_return: Option<f64>,
    pub sector_benchmark_return: Option<f64>,
    pub relative_to_market: Option<f64>,
    pub relative_to_sector: Option<f64>,
    pub current_eps_estimate: Option<f64>,
    pub previous_eps_estimate: Option<f64>,
    pub eps_change_percentage: Option<f64>,
    pub current_revenue_estimate: Option<i64>,
    pub previous_revenue_estimate: Option<i64>,
    pub revenue_change_percentage: Option<f64>,
    pub analyst_count: Option<i32>,
    pub current_price: Option<f64>,
    pub price_15d_ago: Option<f64>,
    pub price_30d_ago: Option<f64>,
    pub return_15d: Option<f64>,
    pub return_30d: Option<f64>,
    pub volume_ratio: Option<f64>,
}

impl From<SentimentRecord> for SentimentInsert {
    fn from(record: SentimentRecord) -> Self {
        Self {
            symbol: record.symbol,
            analysis_date: record.analysis_date,
            qss_score: record.qss_score,
            trading_signal: record.trading_signal,
            confidence_score: record.confidence_score,
            earnings_revisions_score: record.earnings_revisions_score,
            relative_strength_score: record.relative_strength_score,
            short_interest_score: record.short_interest_score,
            options_flow_score: record.options_flow_score,
            earnings_weight: record.earnings_weight,
            relative_strength_weight: record.relative_strength_weight,
            short_interest_weight: record.short_interest_weight,
            options_flow_weight: record.options_flow_weight,
            earnings_api_url: record.earnings_api_url,
            earnings_api_source: record.earnings_api_source,
            earnings_data_available: record.earnings_data_available,
            price_data_api_url: record.price_data_api_url,
            price_data_api_source: record.price_data_api_source,
            price_data_available: record.price_data_available,
            short_interest_api_url: record.short_interest_api_url,
            short_interest_api_source: record.short_interest_api_source,
            short_interest_data_available: record.short_interest_data_available,
            options_flow_api_url: record.options_flow_api_url,
            options_flow_api_source: record.options_flow_api_source,
            options_flow_data_available: record.options_flow_data_available,
            earnings_raw_data: record.earnings_raw_data,
            price_data_raw_data: record.price_data_raw_data,
            short_interest_raw_data: record.short_interest_raw_data,
            options_flow_raw_data: record.options_flow_raw_data,
            data_coverage_percentage: record.data_coverage_percentage,
            computation_time_ms: record.computation_time_ms,
            data_points_count: record.data_points_count,
            trend_direction: record.trend_direction,
            data_freshness_score: record.data_freshness_score,
            warning_flags: record.warning_flags,
            missing_data_components: record.missing_data_components,
            gpt_explanation: record.gpt_explanation,
            gpt_explanation_timestamp: record.gpt_explanation_timestamp,
            rsi_14: record.rsi_14,
            rsi_source: record.rsi_source,
            market_benchmark_return: record.market_benchmark_return,
            sector_benchmark_return: record.sector_benchmark_return,
            relative_to_market: record.relative_to_market,
            relative_to_sector: record.relative_to_sector,
            current_eps_estimate: record.current_eps_estimate,
            previous_eps_estimate: record.previous_eps_estimate,
            eps_change_percentage: record.eps_change_percentage,
            current_revenue_estimate: record.current_revenue_estimate,
            previous_revenue_estimate: record.previous_revenue_estimate,
            revenue_change_percentage: record.revenue_change_percentage,
            analyst_count: record.analyst_count,
            current_price: record.current_price,
            price_15d_ago: record.price_15d_ago,
            price_30d_ago: record.price_30d_ago,
            return_15d: record.return_15d,
            return_30d: record.return_30d,
            volume_ratio: record.volume_ratio,
        }
    }
}
