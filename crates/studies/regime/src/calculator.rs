// Market Regime Analysis Calculator
// Analyzes the overall market regime - "What's the vibe of the whole club?"

use buenotea_core::Result;
use super::models::*;
use chrono::{DateTime, Utc};
use std::time::Instant;
use tracing::info;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Main Market Regime Calculator that analyzes overall market conditions
pub struct MarketRegimeCalculator {
    client: Client,
    raw_api_responses: HashMap<String, serde_json::Value>,
    api_endpoints_used: Vec<String>,
    primary_api_source: String,
    fallback_api_source: Option<String>,
}

impl MarketRegimeCalculator {
    /// Create a new Market Regime Calculator
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            raw_api_responses: HashMap::new(),
            api_endpoints_used: Vec::new(),
            primary_api_source: String::new(),
            fallback_api_source: None,
        }
    }

    /// Calculate overall market regime analysis
    pub async fn calculate_market_regime(&mut self) -> Result<MarketRegimeResult> {
        let start_time = Instant::now();
        info!("Starting market regime analysis...");

        // Reset tracking for new calculation
        self.raw_api_responses.clear();
        self.api_endpoints_used.clear();
        self.primary_api_source.clear();
        self.fallback_api_source = None;

        // Step 1: Collect market context data (SPY, VIX, etc.)
        let market_context = self.collect_market_context().await?;

        // Step 2: Analyze market volatility
        let volatility_analysis = self.analyze_market_volatility(&market_context).await?;

        // Step 3: Analyze market trends
        let trend_analysis = self.analyze_market_trends(&market_context).await?;

        // Step 4: Analyze market breadth
        let breadth_analysis = self.analyze_market_breadth().await?;

        // Step 5: Analyze sector performance
        let sector_analysis = self.analyze_sector_performance().await?;

        // Step 6: Analyze market sentiment
        let sentiment_indicators = self.analyze_market_sentiment().await?;

        // Step 7: Assess market risk
        let risk_assessment = self.assess_market_risk(&market_context, &volatility_analysis).await?;

        // Step 8: Detect market regime
        let market_regime = self.detect_market_regime(
            &market_context,
            &volatility_analysis,
            &trend_analysis,
            &breadth_analysis,
        )?;

        // Step 9: Calculate regime confidence
        let regime_confidence = self.calculate_regime_confidence(
            &market_context,
            &volatility_analysis,
            &trend_analysis,
            &breadth_analysis,
        );

        // Step 10: Create analysis metadata
        let computation_time = start_time.elapsed().as_millis() as i64;
        let metadata = AnalysisMetadata {
            data_sources_used: vec![self.primary_api_source.clone()],
            analysis_period_days: 250,
            computation_time_ms: Some(computation_time),
            api_endpoints_used: self.api_endpoints_used.clone(),
            raw_api_responses: if self.raw_api_responses.is_empty() { None } else { Some(self.raw_api_responses.clone()) },
        };

        let result = MarketRegimeResult {
            market_regime,
            regime_confidence,
            market_context,
            volatility_analysis,
            trend_analysis,
            breadth_analysis,
            sector_analysis,
            sentiment_indicators,
            risk_assessment,
            timestamp: Utc::now(),
            metadata,
        };

        info!("Market regime analysis completed in {}ms", computation_time);
        Ok(result)
    }

    /// Collect market context data (SPY, VIX, etc.)
    async fn collect_market_context(&mut self) -> Result<MarketContext> {
        let mut market_context = MarketContext {
            spy_price: None,
            spy_20d_change: None,
            spy_50d_change: None,
            vix: None,
            market_breadth: None,
        };

        // Fetch SPY data for market context
        if let Ok(spy_data) = self.fetch_price_data_fmp("SPY").await {
            if let (Some(current), Some(price_20d_ago), Some(price_50d_ago)) = (
                spy_data.last().map(|p| p.close),
                spy_data.get(spy_data.len().saturating_sub(20)).map(|p| p.close),
                spy_data.get(spy_data.len().saturating_sub(50)).map(|p| p.close),
            ) {
                market_context.spy_price = Some(current);
                market_context.spy_20d_change = Some((current - price_20d_ago) / price_20d_ago);
                market_context.spy_50d_change = Some((current - price_50d_ago) / price_50d_ago);
            }
        }

        // Fetch VIX data (simplified - using a mock value for now)
        // In a real implementation, you would fetch VIX data from a financial API
        market_context.vix = Some(20.0); // Mock VIX value
        
        // Calculate market breadth (simplified)
        // In a real implementation, you would analyze advancing vs declining stocks
        market_context.market_breadth = Some(0.65); // Mock market breadth (65% advancing)

        Ok(market_context)
    }

    /// Analyze market volatility
    async fn analyze_market_volatility(&self, market_context: &MarketContext) -> Result<VolatilityAnalysis> {
        // Calculate volatility based on SPY data
        let market_volatility = if let Some(spy_change) = market_context.spy_20d_change {
            spy_change.abs() * 100.0 // Convert to percentage
        } else {
            2.0 // Default volatility
        };

        // Calculate volatility percentile (simplified)
        let volatility_percentile = if market_volatility > 3.0 {
            90.0 // High volatility
        } else if market_volatility > 2.0 {
            70.0 // Medium-high volatility
        } else if market_volatility > 1.0 {
            50.0 // Medium volatility
        } else {
            30.0 // Low volatility
        };

        // Determine volatility trend (simplified)
        let volatility_trend = if market_volatility > 2.5 {
            VolatilityTrend::Increasing
        } else if market_volatility < 1.0 {
            VolatilityTrend::Decreasing
        } else {
            VolatilityTrend::Stable
        };

        Ok(VolatilityAnalysis {
            market_volatility,
            volatility_percentile,
            volatility_trend,
        })
    }

    /// Analyze market trends
    async fn analyze_market_trends(&self, market_context: &MarketContext) -> Result<MarketTrendAnalysis> {
        // Analyze trends based on SPY performance
        let short_term = if let Some(spy_20d_change) = market_context.spy_20d_change {
            if spy_20d_change > 0.05 {
                TrendDirection::StrongBullish
            } else if spy_20d_change > 0.02 {
                TrendDirection::Bullish
            } else if spy_20d_change < -0.05 {
                TrendDirection::StrongBearish
            } else if spy_20d_change < -0.02 {
                TrendDirection::Bearish
            } else {
                TrendDirection::Neutral
            }
        } else {
            TrendDirection::Neutral
        };

        let medium_term = if let Some(spy_50d_change) = market_context.spy_50d_change {
            if spy_50d_change > 0.10 {
                TrendDirection::StrongBullish
            } else if spy_50d_change > 0.05 {
                TrendDirection::Bullish
            } else if spy_50d_change < -0.10 {
                TrendDirection::StrongBearish
            } else if spy_50d_change < -0.05 {
                TrendDirection::Bearish
            } else {
                TrendDirection::Neutral
            }
        } else {
            TrendDirection::Neutral
        };

        // Long-term trend (simplified)
        let long_term = medium_term.clone();

        // Calculate trend strength and consistency
        let strength = (short_term.score() + medium_term.score() + long_term.score()).abs() * 50.0;
        let consistency = if short_term == medium_term && medium_term == long_term {
            90.0 // High consistency
        } else if short_term == medium_term || medium_term == long_term {
            70.0 // Medium consistency
        } else {
            50.0 // Low consistency
        };

        Ok(MarketTrendAnalysis {
            short_term,
            medium_term,
            long_term,
            strength,
            consistency,
        })
    }

    /// Analyze market breadth
    async fn analyze_market_breadth(&self) -> Result<MarketBreadthAnalysis> {
        // Mock market breadth analysis
        // In a real implementation, you would fetch data from market breadth APIs
        Ok(MarketBreadthAnalysis {
            advancing_stocks: Some(2500),
            declining_stocks: Some(1500),
            unchanged_stocks: Some(500),
            new_highs: Some(150),
            new_lows: Some(75),
            breadth_ratio: Some(2500.0 / 1500.0),
        })
    }

    /// Analyze sector performance
    async fn analyze_sector_performance(&self) -> Result<SectorAnalysis> {
        // Mock sector analysis
        // In a real implementation, you would fetch sector ETF data
        Ok(SectorAnalysis {
            technology_performance: Some(0.05),
            healthcare_performance: Some(0.02),
            financial_performance: Some(-0.01),
            energy_performance: Some(0.08),
            consumer_performance: Some(0.03),
            leading_sector: Some("Energy".to_string()),
            lagging_sector: Some("Financial".to_string()),
        })
    }

    /// Analyze market sentiment
    async fn analyze_market_sentiment(&self) -> Result<SentimentIndicators> {
        // Mock sentiment analysis
        // In a real implementation, you would fetch sentiment data from various sources
        Ok(SentimentIndicators {
            fear_greed_index: Some(65), // Slightly greedy
            put_call_ratio: Some(0.85),
            margin_debt_trend: Some(MarginDebtTrend::Increasing),
            insider_sentiment: Some(InsiderSentiment::Neutral),
        })
    }

    /// Assess market risk
    async fn assess_market_risk(
        &self,
        market_context: &MarketContext,
        volatility_analysis: &VolatilityAnalysis,
    ) -> Result<MarketRiskAssessment> {
        // Calculate risk score based on volatility and VIX
        let mut risk_score = volatility_analysis.market_volatility * 20.0; // Scale volatility
        
        if let Some(vix) = market_context.vix {
            risk_score += vix * 2.0; // Add VIX component
        }

        // Determine risk level
        let risk_level = if risk_score > 80.0 {
            RiskLevel::VeryHigh
        } else if risk_score > 60.0 {
            RiskLevel::High
        } else if risk_score > 40.0 {
            RiskLevel::Medium
        } else {
            RiskLevel::Low
        };

        // Calculate other risk metrics
        let max_drawdown_risk = risk_score * 1.5; // Estimate max drawdown
        let correlation_risk = if volatility_analysis.market_volatility > 2.0 { 80.0 } else { 40.0 };
        let liquidity_risk = 30.0; // Assume low liquidity risk for major markets

        Ok(MarketRiskAssessment {
            risk_level,
            risk_score: risk_score.min(100.0),
            max_drawdown_risk,
            correlation_risk,
            liquidity_risk,
        })
    }

    /// Detect market regime based on all analysis components
    fn detect_market_regime(
        &self,
        market_context: &MarketContext,
        volatility_analysis: &VolatilityAnalysis,
        trend_analysis: &MarketTrendAnalysis,
        breadth_analysis: &MarketBreadthAnalysis,
    ) -> Result<MarketRegime> {
        // Regime detection logic based on multiple factors
        
        // High volatility regime
        if volatility_analysis.market_volatility > 3.0 || market_context.vix.unwrap_or(20.0) > 30.0 {
            return Ok(MarketRegime::Volatile);
        }

        // Low volatility regime
        if volatility_analysis.market_volatility < 1.0 && market_context.vix.unwrap_or(20.0) < 15.0 {
            return Ok(MarketRegime::Stable);
        }

        // Bull market conditions
        if trend_analysis.short_term == TrendDirection::StrongBullish
            && trend_analysis.medium_term == TrendDirection::Bullish
            && breadth_analysis.breadth_ratio.unwrap_or(1.0) > 1.5
        {
            return Ok(MarketRegime::Bull);
        }

        // Bear market conditions
        if trend_analysis.short_term == TrendDirection::StrongBearish
            && trend_analysis.medium_term == TrendDirection::Bearish
            && breadth_analysis.breadth_ratio.unwrap_or(1.0) < 0.7
        {
            return Ok(MarketRegime::Bear);
        }

        // Sideways market
        if trend_analysis.short_term == TrendDirection::Neutral
            && trend_analysis.medium_term == TrendDirection::Neutral
            && volatility_analysis.market_volatility < 2.0
        {
            return Ok(MarketRegime::Sideways);
        }

        // Transition period (mixed signals)
        Ok(MarketRegime::Transition)
    }

    /// Calculate confidence in regime classification
    fn calculate_regime_confidence(
        &self,
        market_context: &MarketContext,
        volatility_analysis: &VolatilityAnalysis,
        trend_analysis: &MarketTrendAnalysis,
        breadth_analysis: &MarketBreadthAnalysis,
    ) -> f64 {
        let mut confidence: f64 = 0.5; // Base confidence

        // More data points = higher confidence
        if market_context.spy_price.is_some() { confidence += 0.1; }
        if market_context.vix.is_some() { confidence += 0.1; }
        if market_context.market_breadth.is_some() { confidence += 0.1; }

        // Trend consistency = higher confidence
        if trend_analysis.consistency > 80.0 { confidence += 0.1; }
        else if trend_analysis.consistency < 50.0 { confidence -= 0.1; }

        // Breadth confirmation = higher confidence
        if let Some(ratio) = breadth_analysis.breadth_ratio {
            if ratio > 1.2 || ratio < 0.8 { confidence += 0.1; } // Clear breadth signal
        }

        // Volatility clarity = higher confidence
        if volatility_analysis.market_volatility > 3.0 || volatility_analysis.market_volatility < 1.0 {
            confidence += 0.1; // Clear volatility signal
        }

        confidence.max(0.0).min(1.0)
    }

    /// Fetch price data from FMP (Financial Modeling Prep)
    async fn fetch_price_data_fmp(&mut self, symbol: &str) -> Result<Vec<PriceData>> {
        let api_key = std::env::var("FMP_API_KEY")
            .map_err(|_| buenotea_core::Error::MissingApiKey("FMP".to_string()))?;
        
        let url = format!(
            "https://financialmodelingprep.com/api/v3/historical-price-full/{}?apikey={}&limit=250",
            symbol, api_key
        );
        
        self.api_endpoints_used.push(url.clone());
        
        let response = self.client.get(&url).send().await?;
        let json: Value = response.json().await?;
        
        // Store raw response for debugging
        self.raw_api_responses.insert("fmp_price_data".to_string(), json.clone());
        
        let mut price_data = Vec::new();
        
        if let Some(historical_data) = json.get("historical") {
            if let Some(array) = historical_data.as_array() {
                for item in array.iter().take(250) { // Limit to 250 days
                    if let (Some(date_str), Some(open), Some(high), Some(low), Some(close), Some(volume)) = (
                        item.get("date").and_then(|v| v.as_str()),
                        item.get("open").and_then(|v| v.as_f64()),
                        item.get("high").and_then(|v| v.as_f64()),
                        item.get("low").and_then(|v| v.as_f64()),
                        item.get("close").and_then(|v| v.as_f64()),
                        item.get("volume").and_then(|v| v.as_u64()),
                    ) {
                        if let Ok(date) = chrono::DateTime::parse_from_rfc3339(&format!("{}T00:00:00Z", date_str)) {
                            price_data.push(PriceData {
                                date: date.with_timezone(&Utc),
                                open,
                                high,
                                low,
                                close,
                                volume,
                            });
                        }
                    }
                }
            }
        }
        
        price_data.reverse(); // FMP returns newest first, we want oldest first
        Ok(price_data)
    }
}

/// Price data point for market analysis
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

impl Default for MarketRegimeCalculator {
    fn default() -> Self {
        Self::new()
    }
}