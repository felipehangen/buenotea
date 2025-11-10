// Technical Trading Score (TTS) calculator
// Combines multiple technical indicators to generate trading signals

use buenotea_core::Result;
use super::models::*;
use super::indicators::*;
use chrono::Utc;
use std::time::Instant;
use tracing::{info, warn};
use reqwest::Client;
use serde_json::Value;

/// Main TTS calculator that combines technical indicators
pub struct TTSCalculator {
    client: Client,
    raw_api_responses: std::collections::HashMap<String, serde_json::Value>,
    api_endpoints_used: Vec<String>,
    primary_api_source: String,
    fallback_api_source: Option<String>,
}

impl TTSCalculator {
    /// Create a new TTS calculator
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            raw_api_responses: std::collections::HashMap::new(),
            api_endpoints_used: Vec::new(),
            primary_api_source: String::new(),
            fallback_api_source: None,
        }
    }

    /// Calculate TTS score for a given stock symbol with API tracking
    pub async fn calculate_tts_with_tracking(&mut self, symbol: &str) -> Result<(TTSResult, TTSApiTracking)> {
        let start_time = Instant::now();
        info!("Starting TTS calculation for {}", symbol);

        // Reset tracking for new calculation
        self.raw_api_responses.clear();
        self.api_endpoints_used.clear();
        self.primary_api_source.clear();
        self.fallback_api_source = None;

        // Collect price data from multiple sources
        let price_data = self.collect_price_data(symbol).await?;
        
        if price_data.is_empty() {
            return Err(buenotea_core::Error::ValidationError { 
                message: format!("No price data available for {}", symbol) 
            });
        }

        // Calculate technical indicators
        let indicators = calculate_all_indicators(&price_data)?;
        
        // Calculate individual indicator scores
        let indicator_scores = self.calculate_indicator_scores(&price_data, &indicators)?;
        
        // Calculate trend analysis
        let trend_analysis = self.calculate_trend_analysis(&price_data)?;
        
        // Calculate support and resistance
        let support_resistance = self.calculate_support_resistance(&price_data)?;
        
        // Calculate volume analysis
        let volume_analysis = self.calculate_volume_analysis(&price_data)?;
        
        // Calculate risk assessment
        let risk_assessment = self.calculate_risk_assessment(&price_data, &indicators)?;
        
        // Calculate final TTS score
        let tts_score = self.calculate_final_tts_score(&indicator_scores, &trend_analysis)?;
        
        // Generate trading signal
        let trading_signal = self.generate_trading_signal(tts_score);
        
        // Calculate confidence score
        let confidence_score = self.calculate_confidence_score(&price_data, &indicators);
        
        // Generate flags
        let flags = self.generate_flags(&price_data, &indicators);
        
        let computation_time = start_time.elapsed().as_millis() as u64;

        let result = TTSResult {
            symbol: symbol.to_string(),
            tts_score,
            trading_signal,
            indicators: indicator_scores,
            trend_analysis,
            support_resistance,
            volume_analysis,
            risk_assessment,
            timestamp: Utc::now(),
            confidence_score,
            flags,
        };

        info!("TTS calculation completed for {} in {}ms", symbol, computation_time);
        
        // Create API tracking information
        let api_tracking = TTSApiTracking {
            primary_api_source: self.primary_api_source.clone(),
            fallback_api_source: self.fallback_api_source.clone(),
            api_endpoints_used: self.api_endpoints_used.clone(),
            raw_api_responses: if self.raw_api_responses.is_empty() { None } else { Some(self.raw_api_responses.clone()) },
            price_data_points: price_data.len() as i32,
            analysis_period_days: 30, // Approximate
            current_price: price_data.last().map(|p| p.close).unwrap_or(0.0),
        };
        
        Ok((result, api_tracking))
    }

    /// Calculate TTS score for a given stock symbol (backward compatibility)
    pub async fn calculate_tts(&self, symbol: &str) -> Result<TTSResult> {
        let mut calculator = TTSCalculator::new();
        let (result, _) = calculator.calculate_tts_with_tracking(symbol).await?;
        Ok(result)
    }

    /// Collect price data from multiple API sources
    async fn collect_price_data(&mut self, symbol: &str) -> Result<Vec<PricePoint>> {
        let mut price_data = Vec::new();

        // Try FMP first (better data quality)
        if let Ok(data) = self.fetch_fmp_price_data(symbol).await {
            price_data = data;
            self.primary_api_source = "FMP".to_string();
            info!("Collected {} price points from FMP for {}", price_data.len(), symbol);
        } else {
            // Fallback to Alpha Vantage
            if let Ok(data) = self.fetch_alpha_vantage_price_data(symbol).await {
                price_data = data;
                self.primary_api_source = "Alpha Vantage".to_string();
                info!("Collected {} price points from Alpha Vantage for {}", price_data.len(), symbol);
            } else {
                warn!("Failed to collect price data from both FMP and Alpha Vantage for {}", symbol);
            }
        }

        Ok(price_data)
    }

    /// Fetch price data from FMP
    async fn fetch_fmp_price_data(&mut self, symbol: &str) -> Result<Vec<PricePoint>> {
        let api_key = std::env::var("FMP_API_KEY")
            .map_err(|_| buenotea_core::Error::MissingApiKey("FMP".to_string()))?;

        let url = format!(
            "https://financialmodelingprep.com/api/v3/historical-price-full/{}?apikey={}",
            symbol, api_key
        );

        let response = self.client.get(&url).send().await?;
        let json: Value = response.json().await?;
        
        // Store raw API response
        self.api_endpoints_used.push(url.clone());
        self.raw_api_responses.insert("FMP".to_string(), json.clone());

        let mut price_points = Vec::new();
        
        if let Some(historical_data) = json["historical"].as_array() {
            for item in historical_data.iter().take(200) { // Limit to 200 days
                if let (Some(date_str), Some(open), Some(high), Some(low), Some(close), Some(volume)) = (
                    item["date"].as_str(),
                    item["open"].as_f64(),
                    item["high"].as_f64(),
                    item["low"].as_f64(),
                    item["close"].as_f64(),
                    item["volume"].as_u64(),
                ) {
                    if let Ok(date) = chrono::DateTime::parse_from_rfc3339(&format!("{}T00:00:00Z", date_str)) {
                        price_points.push(PricePoint {
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

        // Sort by date (oldest first)
        price_points.sort_by_key(|p| p.date);
        Ok(price_points)
    }

    /// Fetch price data from Alpha Vantage
    async fn fetch_alpha_vantage_price_data(&mut self, symbol: &str) -> Result<Vec<PricePoint>> {
        let api_key = std::env::var("ALPHA_VANTAGE_API_KEY")
            .map_err(|_| buenotea_core::Error::MissingApiKey("Alpha Vantage".to_string()))?;

        let url = format!(
            "https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol={}&apikey={}&outputsize=full",
            symbol, api_key
        );

        let response = self.client.get(&url).send().await?;
        let json: Value = response.json().await?;
        
        // Store raw API response
        self.api_endpoints_used.push(url.clone());
        self.raw_api_responses.insert("Alpha Vantage".to_string(), json.clone());

        let mut price_points = Vec::new();
        
        if let Some(time_series) = json["Time Series (Daily)"].as_object() {
            for (date_str, data) in time_series.iter().take(200) { // Limit to 200 days
                if let (Some(open), Some(high), Some(low), Some(close), Some(volume)) = (
                    data["1. open"].as_str().and_then(|s| s.parse::<f64>().ok()),
                    data["2. high"].as_str().and_then(|s| s.parse::<f64>().ok()),
                    data["3. low"].as_str().and_then(|s| s.parse::<f64>().ok()),
                    data["4. close"].as_str().and_then(|s| s.parse::<f64>().ok()),
                    data["5. volume"].as_str().and_then(|s| s.parse::<u64>().ok()),
                ) {
                    if let Ok(date) = chrono::DateTime::parse_from_rfc3339(&format!("{}T00:00:00Z", date_str)) {
                        price_points.push(PricePoint {
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

        // Sort by date (oldest first)
        price_points.sort_by_key(|p| p.date);
        Ok(price_points)
    }

    /// Calculate scores for individual technical indicators using -1.0 to +1.0 scale
    fn calculate_indicator_scores(&self, price_data: &[PricePoint], indicators: &IndicatorValues) -> Result<TTSIndicators> {
        use super::indicators::*;
        
        let current_price = price_data.last().map(|p| p.close).unwrap_or(0.0);

        // RSI Score (-1.0 to +1.0)
        let rsi_score = if let Some(rsi) = indicators.rsi_14 {
            score_rsi(rsi)
        } else {
            0.0 // Neutral when no data
        };

        // MACD Score (-1.0 to +1.0)
        let macd_score = if let (Some(macd), Some(signal)) = (indicators.macd, indicators.macd_signal) {
            score_macd(macd, signal)
        } else {
            0.0 // Neutral when no data
        };

        // Bollinger Bands Score (-1.0 to +1.0)
        let bollinger_score = if let (Some(upper), Some(middle), Some(lower)) = 
            (indicators.bollinger_upper, indicators.bollinger_middle, indicators.bollinger_lower) {
            score_bollinger_bands(current_price, upper, middle, lower)
        } else {
            0.0 // Neutral when no data
        };

        // Moving Averages Score (-1.0 to +1.0)
        let ma_score = if let (Some(sma_20), Some(sma_50), Some(sma_200)) = 
            (indicators.sma_20, indicators.sma_50, indicators.sma_200) {
            score_moving_averages(current_price, sma_20, sma_50, sma_200)
        } else {
            0.0 // Neutral when no data
        };

        // Stochastic Score (-1.0 to +1.0)
        let stochastic_score = if let (Some(k), Some(d)) = (indicators.stochastic_k, indicators.stochastic_d) {
            score_stochastic(k, d)
        } else {
            0.0 // Neutral when no data
        };

        // Williams %R Score (-1.0 to +1.0)
        let williams_score = if let Some(wr) = indicators.williams_r {
            score_williams_r(wr)
        } else {
            0.0 // Neutral when no data
        };

        // ATR Score (-1.0 to +1.0)
        let atr_score = if let Some(atr) = indicators.atr_14 {
            // Calculate average ATR for comparison
            let avg_atr = price_data.iter()
                .take(20)
                .map(|p| p.high - p.low)
                .sum::<f64>() / 20.0;
            score_atr(atr, current_price, avg_atr)
        } else {
            0.0 // Neutral when no data
        };

        // Volume Score (-1.0 to +1.0)
        let volume_score = self.calculate_volume_score(price_data);

        Ok(TTSIndicators {
            rsi_score,
            macd_score,
            bollinger_score,
            ma_score,
            stochastic_score,
            williams_score,
            atr_score,
            volume_score,
        })
    }


    /// Calculate volume score (-1.0 to +1.0)
    fn calculate_volume_score(&self, price_data: &[PricePoint]) -> f64 {
        use super::indicators::score_volume;
        
        if price_data.len() < 20 {
            return 0.0; // Neutral when insufficient data
        }

        let recent_volumes: Vec<u64> = price_data[price_data.len() - 5..].iter().map(|p| p.volume).collect();
        let avg_recent_volume: f64 = recent_volumes.iter().sum::<u64>() as f64 / recent_volumes.len() as f64;
        
        let historical_volumes: Vec<u64> = price_data[price_data.len() - 20..price_data.len() - 5].iter().map(|p| p.volume).collect();
        let avg_historical_volume: f64 = historical_volumes.iter().sum::<u64>() as f64 / historical_volumes.len() as f64;

        // Calculate price change for volume confirmation
        let current_price = price_data.last().unwrap().close;
        let previous_price = price_data[price_data.len() - 2].close;
        let price_change = (current_price - previous_price) / previous_price;

        score_volume(avg_recent_volume as u64, avg_historical_volume as u64, price_change)
    }

    /// Calculate trend analysis
    fn calculate_trend_analysis(&self, price_data: &[PricePoint]) -> Result<TrendAnalysis> {
        if price_data.len() < 20 {
            return Ok(TrendAnalysis {
                short_term: TrendDirection::Neutral,
                medium_term: TrendDirection::Neutral,
                long_term: TrendDirection::Neutral,
                strength: 50.0,
                consistency: 50.0,
            });
        }

        // Short-term trend (5 days)
        let short_term = self.calculate_trend_direction(&price_data, 5)?;
        
        // Medium-term trend (15 days)
        let medium_term = self.calculate_trend_direction(&price_data, 15)?;
        
        // Long-term trend (30 days)
        let long_term = self.calculate_trend_direction(&price_data, 30.min(price_data.len()))?;

        // Calculate trend strength and consistency
        let (strength, consistency) = self.calculate_trend_metrics(&price_data)?;

        Ok(TrendAnalysis {
            short_term,
            medium_term,
            long_term,
            strength,
            consistency,
        })
    }

    /// Calculate trend direction for a given period
    fn calculate_trend_direction(&self, price_data: &[PricePoint], period: usize) -> Result<TrendDirection> {
        if price_data.len() < period {
            return Ok(TrendDirection::Neutral);
        }

        let recent_prices = &price_data[price_data.len() - period..];
        let start_price = recent_prices[0].close;
        let end_price = recent_prices.last().unwrap().close;
        
        let price_change = (end_price - start_price) / start_price;
        let price_change_percentage = price_change * 100.0;

        if price_change_percentage > 5.0 {
            Ok(TrendDirection::StrongBullish)
        } else if price_change_percentage > 2.0 {
            Ok(TrendDirection::Bullish)
        } else if price_change_percentage < -5.0 {
            Ok(TrendDirection::StrongBearish)
        } else if price_change_percentage < -2.0 {
            Ok(TrendDirection::Bearish)
        } else {
            Ok(TrendDirection::Neutral)
        }
    }

    /// Calculate trend metrics
    fn calculate_trend_metrics(&self, price_data: &[PricePoint]) -> Result<(f64, f64)> {
        if price_data.len() < 20 {
            return Ok((50.0, 50.0));
        }

        // Calculate strength based on price momentum
        let recent_prices: Vec<f64> = price_data[price_data.len() - 10..].iter().map(|p| p.close).collect();
        let mut positive_days = 0;
        
        for i in 1..recent_prices.len() {
            if recent_prices[i] > recent_prices[i - 1] {
                positive_days += 1;
            }
        }
        
        let strength = (positive_days as f64 / (recent_prices.len() - 1) as f64) * 100.0;
        
        // Calculate consistency (how smooth the trend is)
        let mut consistency = 50.0;
        if price_data.len() >= 20 {
            let volatility = self.calculate_volatility(&recent_prices);
            consistency = (100.0 - (volatility * 1000.0)).max(0.0).min(100.0);
        }

        Ok((strength, consistency))
    }

    /// Calculate volatility
    fn calculate_volatility(&self, prices: &[f64]) -> f64 {
        if prices.len() < 2 {
            return 0.0;
        }

        let mean = prices.iter().sum::<f64>() / prices.len() as f64;
        let variance = prices.iter()
            .map(|&price| (price - mean).powi(2))
            .sum::<f64>() / prices.len() as f64;
        
        variance.sqrt()
    }

    /// Calculate support and resistance levels
    fn calculate_support_resistance(&self, price_data: &[PricePoint]) -> Result<SupportResistance> {
        if price_data.len() < 20 {
            let current_price = price_data.last().map(|p| p.close).unwrap_or(0.0);
            return Ok(SupportResistance {
                support_level: current_price * 0.95,
                resistance_level: current_price * 1.05,
                support_distance: 5.0,
                resistance_distance: 5.0,
                support_strength: 50.0,
                resistance_strength: 50.0,
            });
        }

        let recent_data = &price_data[price_data.len() - 50..]; // Use last 50 days
        
        // Find support (recent lows)
        let support_level = recent_data.iter().map(|p| p.low).fold(f64::INFINITY, f64::min);
        
        // Find resistance (recent highs)
        let resistance_level = recent_data.iter().map(|p| p.high).fold(0.0, f64::max);
        
        let current_price = price_data.last().unwrap().close;
        
        let support_distance = ((current_price - support_level) / current_price) * 100.0;
        let resistance_distance = ((resistance_level - current_price) / current_price) * 100.0;

        // Calculate strength based on how many times levels were tested
        let support_strength = self.calculate_level_strength(recent_data, support_level, true)?;
        let resistance_strength = self.calculate_level_strength(recent_data, resistance_level, false)?;

        Ok(SupportResistance {
            support_level,
            resistance_level,
            support_distance: support_distance.max(0.0),
            resistance_distance: resistance_distance.max(0.0),
            support_strength,
            resistance_strength,
        })
    }

    /// Calculate level strength
    fn calculate_level_strength(&self, price_data: &[PricePoint], level: f64, is_support: bool) -> Result<f64> {
        let mut touches = 0;
        let tolerance = level * 0.02; // 2% tolerance

        for price_point in price_data {
            let test_level = if is_support { price_point.low } else { price_point.high };
            if (test_level - level).abs() <= tolerance {
                touches += 1;
            }
        }

        // Convert touches to strength score (0-100)
        let strength = (touches as f64 / price_data.len() as f64) * 100.0;
        Ok(strength.min(100.0))
    }

    /// Calculate volume analysis
    fn calculate_volume_analysis(&self, price_data: &[PricePoint]) -> Result<VolumeAnalysis> {
        if price_data.is_empty() {
            return Ok(VolumeAnalysis {
                current_volume: 0,
                avg_volume: 0,
                volume_ratio: 1.0,
                volume_trend: VolumeTrend::Stable,
                vp_relationship: VolumePriceRelationship::Neutral,
            });
        }

        let current_volume = price_data.last().unwrap().volume;
        
        let avg_volume = if price_data.len() >= 20 {
            let sum: u64 = price_data[price_data.len() - 20..].iter().map(|p| p.volume).sum();
            sum / 20
        } else {
            let sum: u64 = price_data.iter().map(|p| p.volume).sum();
            sum / price_data.len() as u64
        };

        let volume_ratio = current_volume as f64 / avg_volume as f64;

        let volume_trend = if volume_ratio > 1.2 {
            VolumeTrend::Increasing
        } else if volume_ratio < 0.8 {
            VolumeTrend::Decreasing
        } else {
            VolumeTrend::Stable
        };

        // Determine volume-price relationship
        let vp_relationship = if price_data.len() >= 5 {
            let recent_prices = &price_data[price_data.len() - 5..];
            let recent_volumes = &price_data[price_data.len() - 5..];
            
            let price_trend = recent_prices.last().unwrap().close - recent_prices[0].close;
            let volume_trend_avg = recent_volumes.iter().map(|p| p.volume).sum::<u64>() as f64 / recent_volumes.len() as f64;
            let early_volume_avg = price_data[price_data.len() - 10..price_data.len() - 5].iter().map(|p| p.volume).sum::<u64>() as f64 / 5.0;
            
            if price_trend > 0.0 && volume_trend_avg > early_volume_avg {
                VolumePriceRelationship::BullishDivergence
            } else if price_trend < 0.0 && volume_trend_avg > early_volume_avg {
                VolumePriceRelationship::BearishDivergence
            } else {
                VolumePriceRelationship::Neutral
            }
        } else {
            VolumePriceRelationship::Neutral
        };

        Ok(VolumeAnalysis {
            current_volume,
            avg_volume,
            volume_ratio,
            volume_trend,
            vp_relationship,
        })
    }

    /// Calculate risk assessment
    fn calculate_risk_assessment(&self, price_data: &[PricePoint], indicators: &IndicatorValues) -> Result<RiskAssessment> {
        let current_price = price_data.last().map(|p| p.close).unwrap_or(0.0);
        
        // Calculate volatility score
        let volatility_score = if price_data.len() >= 20 {
            let recent_prices: Vec<f64> = price_data[price_data.len() - 20..].iter().map(|p| p.close).collect();
            let volatility = self.calculate_volatility(&recent_prices);
            let volatility_percentage = (volatility / current_price) * 100.0;
            
            if volatility_percentage > 5.0 {
                100.0 // Very high volatility
            } else if volatility_percentage > 3.0 {
                75.0 // High volatility
            } else if volatility_percentage > 2.0 {
                50.0 // Medium volatility
            } else {
                25.0 // Low volatility
            }
        } else {
            50.0
        };

        // Determine risk level
        let risk_level = if volatility_score >= 75.0 {
            RiskLevel::VeryHigh
        } else if volatility_score >= 50.0 {
            RiskLevel::High
        } else if volatility_score >= 25.0 {
            RiskLevel::Medium
        } else {
            RiskLevel::Low
        };

        // Calculate maximum drawdown risk
        let max_drawdown_risk = self.calculate_max_drawdown_risk(price_data)?;

        // Calculate stop loss recommendation
        let stop_loss = if let Some(atr) = indicators.atr_14 {
            current_price - (atr * 2.0) // 2x ATR stop loss
        } else {
            current_price * 0.92 // 8% stop loss as fallback
        };

        // Calculate risk-reward ratio
        let risk_reward_ratio = self.calculate_risk_reward_ratio(price_data, current_price, stop_loss)?;

        Ok(RiskAssessment {
            volatility_score,
            risk_level,
            max_drawdown_risk,
            stop_loss,
            risk_reward_ratio,
        })
    }

    /// Calculate maximum drawdown risk
    fn calculate_max_drawdown_risk(&self, price_data: &[PricePoint]) -> Result<f64> {
        if price_data.len() < 10 {
            return Ok(10.0); // Default 10% risk
        }

        let mut max_drawdown = 0.0;
        let mut peak = 0.0;

        for price_point in price_data {
            let price = price_point.close;
            if price > peak {
                peak = price;
            }
            
            let drawdown = (peak - price) / peak;
            if drawdown > max_drawdown {
                max_drawdown = drawdown;
            }
        }

        Ok(max_drawdown * 100.0) // Convert to percentage
    }

    /// Calculate risk-reward ratio
    fn calculate_risk_reward_ratio(&self, price_data: &[PricePoint], current_price: f64, stop_loss: f64) -> Result<f64> {
        if price_data.len() < 20 {
            return Ok(1.0); // Default 1:1 ratio
        }

        // Use recent highs as potential targets
        let recent_high = price_data[price_data.len() - 20..].iter().map(|p| p.high).fold(0.0, f64::max);
        
        let risk = current_price - stop_loss;
        let reward = recent_high - current_price;
        
        if risk > 0.0 {
            Ok(reward / risk)
        } else {
            Ok(1.0)
        }
    }

    /// Calculate final TTS score from indicator scores and trend analysis (-1.0 to +1.0)
    fn calculate_final_tts_score(&self, indicators: &TTSIndicators, trend_analysis: &TrendAnalysis) -> Result<f64> {
        // Weight the different components
        let mut weighted_score = 0.0;

        // Technical indicators (70% weight)
        let indicators_avg = (
            indicators.rsi_score +
            indicators.macd_score +
            indicators.bollinger_score +
            indicators.ma_score +
            indicators.stochastic_score +
            indicators.williams_score +
            indicators.atr_score +
            indicators.volume_score
        ) / 8.0;
        
        weighted_score += indicators_avg * 0.7;

        // Trend analysis (30% weight)
        let trend_score = (
            trend_analysis.short_term.score() +
            trend_analysis.medium_term.score() +
            trend_analysis.long_term.score()
        ) / 3.0;
        
        weighted_score += trend_score * 0.3;

        // Ensure score is within bounds
        Ok(weighted_score.max(-1.0).min(1.0))
    }

    /// Generate trading signal from TTS score (-1.0 to +1.0)
    fn generate_trading_signal(&self, tts_score: f64) -> TTSSignal {
        TTSSignal::from_score(tts_score)
    }

    /// Calculate confidence score
    fn calculate_confidence_score(&self, price_data: &[PricePoint], indicators: &IndicatorValues) -> f64 {
        let mut confidence = 0.0;

        // Data completeness factor (30% weight)
        let data_completeness = (price_data.len() as f64 / 50.0).min(1.0); // 50 days = 100% completeness
        confidence += data_completeness * 0.3;

        // Indicator availability factor (40% weight)
        let indicator_count = [
            indicators.rsi_14,
            indicators.macd,
            indicators.bollinger_upper,
            indicators.sma_20,
            indicators.stochastic_k,
            indicators.williams_r,
            indicators.atr_14,
        ].iter().filter(|&&x| x.is_some()).count();
        
        let indicator_completeness = indicator_count as f64 / 7.0;
        confidence += indicator_completeness * 0.4;

        // Data recency factor (30% weight)
        confidence += 0.3; // Assume recent data

        confidence.min(1.0) // Cap at 100%
    }

    /// Generate flags for the analysis
    fn generate_flags(&self, price_data: &[PricePoint], indicators: &IndicatorValues) -> Vec<String> {
        let mut flags = Vec::new();

        if price_data.len() < 20 {
            flags.push("Insufficient historical data".to_string());
        }

        if indicators.rsi_14.is_none() {
            flags.push("RSI calculation failed".to_string());
        }

        if indicators.macd.is_none() {
            flags.push("MACD calculation failed".to_string());
        }

        if let Some(rsi) = indicators.rsi_14 {
            if rsi >= 70.0 {
                flags.push("RSI indicates overbought conditions".to_string());
            } else if rsi <= 30.0 {
                flags.push("RSI indicates oversold conditions".to_string());
            }
        }

        if let Some(stoch) = indicators.stochastic_k {
            if stoch >= 80.0 {
                flags.push("Stochastic indicates overbought conditions".to_string());
            } else if stoch <= 20.0 {
                flags.push("Stochastic indicates oversold conditions".to_string());
            }
        }

        flags
    }
}

impl Default for TTSCalculator {
    fn default() -> Self {
        Self::new()
    }
}
