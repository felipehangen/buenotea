// Re-export timing models from infrastructure and add helper functions

pub use buenotea_infrastructure::timing_models::*;
use crate::models::{TTSResult, TTSApiTracking};
use buenotea_infrastructure::timing_models::CreateTimingRecord;

/// Helper function to create timing record from TTS result with API tracking
/// This function lives in the timing crate to avoid circular dependencies
pub fn create_timing_record_with_tracking(
    tts_result: TTSResult,
    api_tracking: TTSApiTracking,
) -> CreateTimingRecord {
    CreateTimingRecord {
        symbol: tts_result.symbol,
        tts_score: tts_result.tts_score,
        trading_signal: format!("{:?}", tts_result.trading_signal),
        confidence_score: tts_result.confidence_score,
        
        // Technical Indicators Scores
        rsi_score: tts_result.indicators.rsi_score,
        macd_score: tts_result.indicators.macd_score,
        bollinger_score: tts_result.indicators.bollinger_score,
        ma_score: tts_result.indicators.ma_score,
        stochastic_score: tts_result.indicators.stochastic_score,
        williams_score: tts_result.indicators.williams_score,
        atr_score: tts_result.indicators.atr_score,
        volume_score: tts_result.indicators.volume_score,
        
        // Trend Analysis
        short_term_trend: format!("{:?}", tts_result.trend_analysis.short_term),
        medium_term_trend: format!("{:?}", tts_result.trend_analysis.medium_term),
        long_term_trend: format!("{:?}", tts_result.trend_analysis.long_term),
        trend_strength: tts_result.trend_analysis.strength,
        trend_consistency: tts_result.trend_analysis.consistency,
        
        // Support & Resistance
        support_level: tts_result.support_resistance.support_level,
        resistance_level: tts_result.support_resistance.resistance_level,
        support_distance: tts_result.support_resistance.support_distance,
        resistance_distance: tts_result.support_resistance.resistance_distance,
        support_strength: tts_result.support_resistance.support_strength,
        resistance_strength: tts_result.support_resistance.resistance_strength,
        
        // Volume Analysis
        current_volume: tts_result.volume_analysis.current_volume as i64,
        avg_volume: tts_result.volume_analysis.avg_volume as i64,
        volume_ratio: tts_result.volume_analysis.volume_ratio,
        volume_trend: format!("{:?}", tts_result.volume_analysis.volume_trend),
        vp_relationship: format!("{:?}", tts_result.volume_analysis.vp_relationship),
        
        // Risk Assessment
        volatility_score: tts_result.risk_assessment.volatility_score,
        risk_level: format!("{:?}", tts_result.risk_assessment.risk_level),
        max_drawdown_risk: tts_result.risk_assessment.max_drawdown_risk,
        stop_loss: tts_result.risk_assessment.stop_loss,
        risk_reward_ratio: tts_result.risk_assessment.risk_reward_ratio,
        
        // API Source Tracking (filled from api_tracking)
        primary_api_source: api_tracking.primary_api_source,
        fallback_api_source: api_tracking.fallback_api_source,
        api_endpoints_used: api_tracking.api_endpoints_used,
        raw_api_responses: api_tracking.raw_api_responses,
        
        // Price Data Used for Analysis (filled from api_tracking)
        price_data_points: api_tracking.price_data_points,
        analysis_period_days: api_tracking.analysis_period_days,
        current_price: api_tracking.current_price,
        
        // AI Analysis (to be filled by AI service)
        chatgpt_explanation: None,
        trading_suggestion: None,
        
        // Metadata
        flags: tts_result.flags,
    }
}
