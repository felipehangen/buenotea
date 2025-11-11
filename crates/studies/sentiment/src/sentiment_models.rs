// Helper models and functions for converting sentiment analysis to database records

use chrono::Utc;

/// Helper function to create sentiment record from QSSResult (now uses API tracking from result)
/// This function lives in the sentiment crate to avoid circular dependencies
pub fn create_sentiment_record_with_tracking(
    result: crate::models::QSSResult,
    gpt_explanation: String,
) -> buenotea_infrastructure::sentiment_models::CreateSentimentRecord {
    // Extract API tracking from result
    let api_tracking = &result.api_tracking;
    
    buenotea_infrastructure::sentiment_models::CreateSentimentRecord {
        symbol: result.symbol.clone(),
        analysis_date: result.timestamp,
        qss_score: result.qss_score,
        trading_signal: format!("{:?}", result.trading_signal),
        confidence_score: result.confidence_score,
        earnings_revisions_score: result.components.earnings_revisions,
        relative_strength_score: result.components.relative_strength,
        short_interest_score: result.components.short_interest,
        options_flow_score: result.components.options_flow,
        earnings_weight: 0.40,
        relative_strength_weight: 0.30,
        short_interest_weight: 0.20,
        options_flow_weight: 0.10,
        earnings_api_url: api_tracking.earnings_api_url.clone(),
        earnings_api_source: Some(api_tracking.earnings_api_source.clone()),
        earnings_data_available: api_tracking.earnings_api_url.is_some(),
        price_data_api_url: api_tracking.price_data_api_url.clone(),
        price_data_api_source: Some(api_tracking.price_data_api_source.clone()),
        price_data_available: api_tracking.price_data_api_url.is_some(),
        short_interest_api_url: api_tracking.short_interest_api_url.clone(),
        short_interest_api_source: Some(api_tracking.short_interest_api_source.clone()),
        short_interest_data_available: api_tracking.short_interest_api_url.is_some(),
        options_flow_api_url: api_tracking.options_flow_api_url.clone(),
        options_flow_api_source: Some(api_tracking.options_flow_api_source.clone()),
        options_flow_data_available: api_tracking.options_flow_api_url.is_some(),
        earnings_raw_data: api_tracking.earnings_raw_data.clone(),
        price_data_raw_data: api_tracking.price_data_raw_data.clone(),
        short_interest_raw_data: api_tracking.short_interest_raw_data.clone(),
        options_flow_raw_data: api_tracking.options_flow_raw_data.clone(),
        data_coverage_percentage: 75.0, // Could be calculated from available data
        computation_time_ms: result.meta.computation_time_ms as i32,
        data_points_count: result.meta.data_points_count as i32,
        trend_direction: result.meta.trend_direction,
        data_freshness_score: result.meta.data_freshness,
        warning_flags: result.flags.clone(),
        missing_data_components: vec![], // Could be calculated from flags
        gpt_explanation,
        gpt_explanation_timestamp: Some(Utc::now()),
        rsi_14: result.meta.rsi_14,
        rsi_source: result.meta.rsi_source,
        market_benchmark_return: result.meta.market_benchmark_return,
        sector_benchmark_return: result.meta.sector_benchmark_return,
        relative_to_market: result.meta.relative_to_market,
        relative_to_sector: result.meta.relative_to_sector,
        current_eps_estimate: result.meta.current_eps_estimate,
        previous_eps_estimate: result.meta.previous_eps_estimate,
        eps_change_percentage: result.meta.eps_change_percentage,
        current_revenue_estimate: result.meta.current_revenue_estimate,
        previous_revenue_estimate: result.meta.previous_revenue_estimate,
        revenue_change_percentage: result.meta.revenue_change_percentage,
        analyst_count: result.meta.analyst_count,
        current_price: result.meta.current_price,
        price_15d_ago: result.meta.price_15d_ago,
        price_30d_ago: result.meta.price_30d_ago,
        return_15d: result.meta.return_15d,
        return_30d: result.meta.return_30d,
        volume_ratio: result.meta.volume_ratio,
    }
}

