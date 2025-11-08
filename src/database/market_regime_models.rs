// Database models for Market Regime Analysis
// Stores overall market regime analysis in the database

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::regime::*;

/// Market regime database record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketRegimeRecord {
    pub analysis_date: DateTime<Utc>,
    
    // Market Regime Classification
    pub market_regime: String,
    pub regime_confidence: f64,
    
    // Market Context Data
    pub spy_price: Option<f64>,
    pub spy_20d_change: Option<f64>,
    pub spy_50d_change: Option<f64>,
    pub vix: Option<f64>,
    pub market_breadth: Option<f64>,
    
    // Market Volatility Analysis
    pub market_volatility: f64,
    pub volatility_percentile: f64,
    
    // Market Trend Analysis
    pub short_term_trend: String,
    pub medium_term_trend: String,
    pub long_term_trend: String,
    pub trend_strength: f64,
    pub trend_consistency: f64,
    
    // Market Breadth Analysis
    pub advancing_stocks: Option<i32>,
    pub declining_stocks: Option<i32>,
    pub unchanged_stocks: Option<i32>,
    pub new_highs: Option<i32>,
    pub new_lows: Option<i32>,
    
    // Sector Analysis
    pub technology_performance: Option<f64>,
    pub healthcare_performance: Option<f64>,
    pub financial_performance: Option<f64>,
    pub energy_performance: Option<f64>,
    pub consumer_performance: Option<f64>,
    
    // Market Sentiment Indicators
    pub fear_greed_index: Option<i32>,
    pub put_call_ratio: Option<f64>,
    pub margin_debt_trend: Option<String>,
    
    // Risk Assessment
    pub market_risk_level: String,
    pub market_risk_score: f64,
    pub max_drawdown_risk: f64,
    
    // AI Analysis (Optional)
    pub chatgpt_regime_analysis: Option<String>,
    pub chatgpt_market_outlook: Option<String>,
    pub chatgpt_risk_assessment: Option<String>,
    pub chatgpt_model_used: Option<String>,
    pub chatgpt_analysis_timestamp: Option<DateTime<Utc>>,
    
    // Analysis Metadata
    pub data_sources_used: Vec<String>,
    pub analysis_period_days: i32,
    pub computation_time_ms: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl MarketRegimeRecord {
    /// Create a new market regime record from analysis result
    pub fn from_market_regime_result(
        result: &MarketRegimeResult,
        chatgpt_analysis: Option<ChatGPTMarketAnalysis>,
    ) -> Self {
        Self {
            analysis_date: result.timestamp,
            
            // Market Regime Classification
            market_regime: result.market_regime.to_string(),
            regime_confidence: result.regime_confidence,
            
            // Market Context Data
            spy_price: result.market_context.spy_price,
            spy_20d_change: result.market_context.spy_20d_change,
            spy_50d_change: result.market_context.spy_50d_change,
            vix: result.market_context.vix,
            market_breadth: result.market_context.market_breadth,
            
            // Market Volatility Analysis
            market_volatility: result.volatility_analysis.market_volatility,
            volatility_percentile: result.volatility_analysis.volatility_percentile,
            
            // Market Trend Analysis
            short_term_trend: format!("{:?}", result.trend_analysis.short_term),
            medium_term_trend: format!("{:?}", result.trend_analysis.medium_term),
            long_term_trend: format!("{:?}", result.trend_analysis.long_term),
            trend_strength: result.trend_analysis.strength,
            trend_consistency: result.trend_analysis.consistency,
            
            // Market Breadth Analysis
            advancing_stocks: result.breadth_analysis.advancing_stocks,
            declining_stocks: result.breadth_analysis.declining_stocks,
            unchanged_stocks: result.breadth_analysis.unchanged_stocks,
            new_highs: result.breadth_analysis.new_highs,
            new_lows: result.breadth_analysis.new_lows,
            
            // Sector Analysis
            technology_performance: result.sector_analysis.technology_performance,
            healthcare_performance: result.sector_analysis.healthcare_performance,
            financial_performance: result.sector_analysis.financial_performance,
            energy_performance: result.sector_analysis.energy_performance,
            consumer_performance: result.sector_analysis.consumer_performance,
            
            // Market Sentiment Indicators
            fear_greed_index: result.sentiment_indicators.fear_greed_index,
            put_call_ratio: result.sentiment_indicators.put_call_ratio,
            margin_debt_trend: result.sentiment_indicators.margin_debt_trend.as_ref().map(|t| format!("{:?}", t)),
            
            // Risk Assessment
            market_risk_level: result.risk_assessment.risk_level.to_string(),
            market_risk_score: result.risk_assessment.risk_score,
            max_drawdown_risk: result.risk_assessment.max_drawdown_risk,
            
            // AI Analysis
            chatgpt_regime_analysis: chatgpt_analysis.as_ref().map(|a| a.regime_analysis.clone()),
            chatgpt_market_outlook: chatgpt_analysis.as_ref().map(|a| a.market_outlook.clone()),
            chatgpt_risk_assessment: chatgpt_analysis.as_ref().map(|a| a.risk_assessment.clone()),
            chatgpt_model_used: chatgpt_analysis.as_ref().map(|a| a.model_used.clone()),
            chatgpt_analysis_timestamp: chatgpt_analysis.as_ref().map(|a| a.analysis_timestamp),
            
            // Analysis Metadata
            data_sources_used: result.metadata.data_sources_used.clone(),
            analysis_period_days: result.metadata.analysis_period_days,
            computation_time_ms: result.metadata.computation_time_ms,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

/// ChatGPT analysis for market regime
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
    fn test_market_regime_record_creation() {
        let result = create_mock_market_regime_result();
        let record = MarketRegimeRecord::from_market_regime_result(&result, None);
        
        assert_eq!(record.market_regime, "Bull");
        assert_eq!(record.regime_confidence, 0.85);
        assert_eq!(record.spy_price, Some(450.0));
        assert_eq!(record.vix, Some(18.5));
    }

    fn create_mock_market_regime_result() -> MarketRegimeResult {
        MarketRegimeResult {
            market_regime: MarketRegime::Bull,
            regime_confidence: 0.85,
            market_context: MarketContext {
                spy_price: Some(450.0),
                spy_20d_change: Some(0.05),
                spy_50d_change: Some(0.08),
                vix: Some(18.5),
                market_breadth: Some(0.65),
            },
            volatility_analysis: VolatilityAnalysis {
                market_volatility: 2.5,
                volatility_percentile: 70.0,
            },
            trend_analysis: MarketTrendAnalysis {
                short_term: TrendDirection::Bullish,
                medium_term: TrendDirection::Bullish,
                long_term: TrendDirection::Bullish,
                strength: 80.0,
                consistency: 90.0,
            },
            breadth_analysis: MarketBreadthAnalysis {
                advancing_stocks: Some(2500),
                declining_stocks: Some(1500),
                unchanged_stocks: Some(500),
                new_highs: Some(150),
                new_lows: Some(75),
                breadth_ratio: Some(1.67),
            },
            sector_analysis: SectorAnalysis {
                technology_performance: Some(0.05),
                healthcare_performance: Some(0.02),
                financial_performance: Some(-0.01),
            energy_performance: Some(0.08),
            consumer_performance: Some(0.03),
            },
            sentiment_indicators: SentimentIndicators {
                fear_greed_index: Some(65),
                put_call_ratio: Some(0.85),
                margin_debt_trend: Some(MarginDebtTrend::Increasing),
                insider_sentiment: Some(InsiderSentiment::Neutral),
            },
            risk_assessment: MarketRiskAssessment {
                risk_level: RiskLevel::Medium,
                risk_score: 45.0,
                max_drawdown_risk: 15.0,
            },
            timestamp: Utc::now(),
            metadata: AnalysisMetadata {
                data_sources_used: vec!["FMP".to_string()],
                analysis_period_days: 250,
                computation_time_ms: Some(1500),
                api_endpoints_used: vec!["https://financialmodelingprep.com/api/v3/historical-price-full/SPY".to_string()],
                raw_api_responses: None,
            },
        }
    }
}
