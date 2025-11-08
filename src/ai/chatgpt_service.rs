// ChatGPT integration for generating TTS explanations and trading suggestions

use crate::error::Result;
use crate::timing::TTSResult;
// use crate::timing::TTSResult as RegimeTTSResult;
// use crate::database::regime_models::ChatGPTAnalysis;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{info, error};

/// ChatGPT API response structure
#[derive(Debug, Serialize, Deserialize)]
struct ChatGPTResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    content: String,
}

/// ChatGPT service for generating AI explanations
pub struct ChatGPTService {
    client: Client,
    api_key: String,
}

impl ChatGPTService {
    /// Create a new ChatGPT service
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    /// Generate a generic response from ChatGPT
    pub async fn generate_response(&self, prompt: &str) -> Result<String> {
        info!("Generating ChatGPT response");
        
        match self.call_chatgpt_api(prompt).await {
            Ok(response) => {
                info!("✅ Generated ChatGPT response successfully");
                Ok(response)
            }
            Err(e) => {
                error!("❌ Failed to generate ChatGPT response: {}", e);
                Err(e)
            }
        }
    }

    /// Generate explanation for TTS analysis
    pub async fn generate_tts_explanation(&self, tts_result: &TTSResult) -> Result<(String, String)> {
        info!("Generating ChatGPT explanation for {} TTS analysis", tts_result.symbol);

        let prompt = self.create_tts_prompt(tts_result);
        
        match self.call_chatgpt_api(&prompt).await {
            Ok(response) => {
                let (explanation, suggestion) = self.parse_chatgpt_response(&response)?;
                info!("Successfully generated ChatGPT explanation for {}", tts_result.symbol);
                Ok((explanation, suggestion))
            }
            Err(e) => {
                error!("Failed to generate ChatGPT explanation: {}", e);
                // Return fallback explanations
                Ok(self.generate_fallback_explanations(tts_result))
            }
        }
    }

    /// Create a detailed prompt for ChatGPT
    fn create_tts_prompt(&self, tts_result: &TTSResult) -> String {
        format!(
            r#"You are a professional financial analyst and trading expert. Analyze the following Technical Trading Score (TTS) data for {} and provide:

1. A clear explanation of what the TTS score ({:.1}/100) reveals about the stock's technical position
2. A specific trading suggestion based on the analysis

TTS Analysis Data:
- Overall Score: {:.1}/100 (Trading Signal: {:?})
- Confidence: {:.1}%

Technical Indicators:
- RSI: {:.1}/100
- MACD: {:.1}/100  
- Bollinger Bands: {:.1}/100
- Moving Averages: {:.1}/100
- Stochastic: {:.1}/100
- Williams %R: {:.1}/100
- ATR: {:.1}/100
- Volume: {:.1}/100

Trend Analysis:
- Short-term: {:?}
- Medium-term: {:?}
- Long-term: {:?}
- Trend Strength: {:.1}/100
- Trend Consistency: {:.1}/100

Support & Resistance:
- Support: ${:.2} ({:.1}% below current)
- Resistance: ${:.2} ({:.1}% above current)

Risk Assessment:
- Risk Level: {:?}
- Volatility: {:.1}/100
- Max Drawdown Risk: {:.1}%
- Stop Loss: ${:.2}
- Risk/Reward Ratio: {:.2}

Flags: {:?}

Please provide:
1. EXPLANATION: What does this TTS score reveal about the stock's technical position? (2-3 sentences)
2. SUGGESTION: What specific trading action would you recommend? (1-2 sentences)

Format your response as:
EXPLANATION: [your explanation]
SUGGESTION: [your suggestion]"#,
            tts_result.symbol,
            tts_result.tts_score,
            tts_result.tts_score,
            tts_result.trading_signal,
            tts_result.confidence_score * 100.0,
            tts_result.indicators.rsi_score,
            tts_result.indicators.macd_score,
            tts_result.indicators.bollinger_score,
            tts_result.indicators.ma_score,
            tts_result.indicators.stochastic_score,
            tts_result.indicators.williams_score,
            tts_result.indicators.atr_score,
            tts_result.indicators.volume_score,
            tts_result.trend_analysis.short_term,
            tts_result.trend_analysis.medium_term,
            tts_result.trend_analysis.long_term,
            tts_result.trend_analysis.strength,
            tts_result.trend_analysis.consistency,
            tts_result.support_resistance.support_level,
            tts_result.support_resistance.support_distance,
            tts_result.support_resistance.resistance_level,
            tts_result.support_resistance.resistance_distance,
            tts_result.risk_assessment.risk_level,
            tts_result.risk_assessment.volatility_score,
            tts_result.risk_assessment.max_drawdown_risk,
            tts_result.risk_assessment.stop_loss,
            tts_result.risk_assessment.risk_reward_ratio,
            tts_result.flags
        )
    }

    /// Call ChatGPT API
    async fn call_chatgpt_api(&self, prompt: &str) -> Result<String> {
        let url = "https://api.openai.com/v1/chat/completions";
        
        let request_body = json!({
            "model": "gpt-3.5-turbo",
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "max_tokens": 500,
            "temperature": 0.7
        });

        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(crate::error::Error::ApiError(
                "ChatGPT".to_string(),
                format!("API call failed with status {}: {}", status, error_text)
            ));
        }

        let chatgpt_response: ChatGPTResponse = response.json().await?;
        
        if let Some(choice) = chatgpt_response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(crate::error::Error::ApiError(
                "ChatGPT".to_string(),
                "No response choices returned from ChatGPT API".to_string()
            ))
        }
    }

    /// Parse ChatGPT response to extract explanation and suggestion
    fn parse_chatgpt_response(&self, response: &str) -> Result<(String, String)> {
        let lines: Vec<&str> = response.lines().collect();
        
        let mut explanation = String::new();
        let mut suggestion = String::new();
        
        let mut current_section = "";
        
        for line in lines {
            let trimmed = line.trim();
            
            if trimmed.starts_with("EXPLANATION:") {
                current_section = "explanation";
                explanation = trimmed.replace("EXPLANATION:", "").trim().to_string();
            } else if trimmed.starts_with("SUGGESTION:") {
                current_section = "suggestion";
                suggestion = trimmed.replace("SUGGESTION:", "").trim().to_string();
            } else if !trimmed.is_empty() && !current_section.is_empty() {
                // Continue building the current section
                match current_section {
                    "explanation" => {
                        if !explanation.is_empty() {
                            explanation.push(' ');
                        }
                        explanation.push_str(trimmed);
                    }
                    "suggestion" => {
                        if !suggestion.is_empty() {
                            suggestion.push(' ');
                        }
                        suggestion.push_str(trimmed);
                    }
                    _ => {}
                }
            }
        }
        
        // Fallback if parsing failed
        if explanation.is_empty() || suggestion.is_empty() {
            let parts: Vec<&str> = response.split('\n').collect();
            if parts.len() >= 2 {
                explanation = parts[0].trim().to_string();
                suggestion = parts[1].trim().to_string();
            } else {
                explanation = response.trim().to_string();
                suggestion = "Consider reviewing technical indicators before making trading decisions.".to_string();
            }
        }
        
        Ok((explanation, suggestion))
    }

    /// Generate fallback explanations when ChatGPT API is unavailable
    fn generate_fallback_explanations(&self, tts_result: &TTSResult) -> (String, String) {
        let explanation = match tts_result.trading_signal {
            crate::timing::TTSSignal::StrongBuy => {
                format!("{} shows strong bullish technical indicators with a TTS score of {:.1}/100. Multiple indicators align favorably, suggesting strong upward momentum potential.", 
                    tts_result.symbol, tts_result.tts_score)
            }
            crate::timing::TTSSignal::Buy => {
                format!("{} displays positive technical signals with a TTS score of {:.1}/100. The analysis indicates favorable conditions for potential upward movement.", 
                    tts_result.symbol, tts_result.tts_score)
            }
            crate::timing::TTSSignal::Neutral => {
                format!("{} shows mixed technical signals with a TTS score of {:.1}/100. The indicators are conflicting, suggesting a wait-and-see approach may be prudent.", 
                    tts_result.symbol, tts_result.tts_score)
            }
            crate::timing::TTSSignal::Sell => {
                format!("{} exhibits bearish technical indicators with a TTS score of {:.1}/100. The analysis suggests potential downward pressure on the stock.", 
                    tts_result.symbol, tts_result.tts_score)
            }
            crate::timing::TTSSignal::StrongSell => {
                format!("{} shows strong bearish technical signals with a TTS score of {:.1}/100. Multiple indicators point to significant downward momentum risk.", 
                    tts_result.symbol, tts_result.tts_score)
            }
        };

        let suggestion = match tts_result.trading_signal {
            crate::timing::TTSSignal::StrongBuy => {
                format!("Consider establishing a long position with a stop loss at ${:.2}. Monitor for any reversal signals.", tts_result.risk_assessment.stop_loss)
            }
            crate::timing::TTSSignal::Buy => {
                format!("A cautious long position may be appropriate with tight risk management. Set stop loss at ${:.2}.", tts_result.risk_assessment.stop_loss)
            }
            crate::timing::TTSSignal::Neutral => {
                "Wait for clearer technical signals before taking a position. Monitor key support and resistance levels.".to_string()
            }
            crate::timing::TTSSignal::Sell => {
                format!("Consider reducing position size or implementing protective measures. Monitor support at ${:.2}.", tts_result.support_resistance.support_level)
            }
            crate::timing::TTSSignal::StrongSell => {
                format!("Strongly consider exiting long positions or establishing short positions. Use stop loss at ${:.2}.", tts_result.risk_assessment.stop_loss)
            }
        };

        (explanation, suggestion)
    }

    // TODO: Implement market regime analysis
    /*
    /// Generate regime analysis explanation for regime TTS results
    pub async fn generate_regime_analysis(&self, regime_result: &RegimeTTSResult) -> Result<ChatGPTAnalysis> {
        info!("Generating ChatGPT regime analysis for {} TTS analysis", regime_result.symbol);

        let prompt = self.create_regime_prompt(regime_result);
        
        match self.call_chatgpt_api(&prompt).await {
            Ok(response) => {
                let analysis = self.parse_regime_response(&response, regime_result)?;
                info!("Successfully generated ChatGPT regime analysis for {}", regime_result.symbol);
                Ok(analysis)
            }
            Err(e) => {
                error!("Failed to generate ChatGPT regime analysis: {}", e);
                // Return fallback analysis
                Ok(self.generate_fallback_regime_analysis(regime_result))
            }
        }
    }

    /// Create a detailed prompt for regime analysis
    fn create_regime_prompt(&self, regime_result: &RegimeTTSResult) -> String {
        format!(
            r#"You are a professional financial analyst and trading expert specializing in market regime analysis. Analyze the following Time To Sell (TTS) analysis with market regime detection for {} and provide:

1. REGIME ANALYSIS: Explain what the detected market regime ({}) reveals about the current market environment and its impact on this stock
2. TTS INTERPRETATION: Explain what the TTS score ({:.3}) means for position management and timing
3. TRADING RECOMMENDATION: Provide specific actionable trading advice based on the regime and TTS analysis

TTS Analysis Data:
- Symbol: {}
- TTS Score: {:.3} (Range: -1.0 to +1.0)
- Trading Signal: {} {}
- Market Regime: {} {} (Multiplier: {:.2}x)
- Confidence: {:.1}%

TTS Component Scores:
- Momentum Score: {:.3} (30% weight)
- Volatility Score: {:.3} (25% weight) 
- Volume Score: {:.3} (20% weight)
- Support/Resistance Score: {:.3} (15% weight)
- Market Correlation Score: {:.3} (10% weight)

Technical Indicators:
- RSI (14-day): {:.1}
- MACD: {:.3}
- MACD Signal: {:.3}
- SMA 20-day: ${:.2}
- SMA 50-day: ${:.2}
- SMA 200-day: ${:.2}
- ATR (14-day): ${:.2}
- Bollinger Bands: ${:.2} - ${:.2} - ${:.2}

Market Context:
- SPY Price: ${:.2}
- SPY 20-day Change: {:.2}%
- SPY 50-day Change: {:.2}%
- VIX: {:.1}
- Sector Relative Performance: {:.2}%

Risk Assessment:
- Risk Level: {}
- Volatility Score: {:.1}%
- Max Drawdown Risk: {:.1}%
- Stop Loss: ${:.2}
- Risk-Reward Ratio: {:.1}:1
- Position Size: {:.1}%

Warning Flags: {}

Please provide your analysis in this exact format:
REGIME ANALYSIS: [Your explanation of the market regime and its implications]
TTS INTERPRETATION: [Your explanation of what the TTS score means]
TRADING RECOMMENDATION: [Your specific trading advice]"#,
            regime_result.symbol,
            regime_result.market_regime,
            regime_result.tts_score,
            regime_result.symbol,
            regime_result.tts_score,
            regime_result.trading_signal.emoji(),
            regime_result.trading_signal.description(),
            regime_result.market_regime.emoji(),
            regime_result.market_regime.description(),
            regime_result.market_regime.tts_multiplier(),
            regime_result.confidence_score * 100.0,
            regime_result.components.momentum_score,
            regime_result.components.volatility_score,
            regime_result.components.volume_score,
            regime_result.components.support_resistance_score,
            regime_result.components.market_correlation_score,
            regime_result.technical_indicators.rsi_14.unwrap_or(0.0),
            regime_result.technical_indicators.macd.unwrap_or(0.0),
            regime_result.technical_indicators.macd_signal.unwrap_or(0.0),
            regime_result.technical_indicators.sma_20.unwrap_or(0.0),
            regime_result.technical_indicators.sma_50.unwrap_or(0.0),
            regime_result.technical_indicators.sma_200.unwrap_or(0.0),
            regime_result.technical_indicators.atr_14.unwrap_or(0.0),
            regime_result.technical_indicators.bollinger_upper.unwrap_or(0.0),
            regime_result.technical_indicators.bollinger_middle.unwrap_or(0.0),
            regime_result.technical_indicators.bollinger_lower.unwrap_or(0.0),
            regime_result.market_context.spy_price.unwrap_or(0.0),
            regime_result.market_context.spy_20d_change.unwrap_or(0.0) * 100.0,
            regime_result.market_context.spy_50d_change.unwrap_or(0.0) * 100.0,
            regime_result.market_context.vix.unwrap_or(0.0),
            regime_result.market_context.sector_relative_performance.unwrap_or(0.0) * 100.0,
            regime_result.risk_assessment.risk_level,
            regime_result.risk_assessment.volatility_score,
            regime_result.risk_assessment.max_drawdown_risk,
            regime_result.risk_assessment.stop_loss.unwrap_or(0.0),
            regime_result.risk_assessment.risk_reward_ratio.unwrap_or(0.0),
            regime_result.risk_assessment.position_size * 100.0,
            if regime_result.flags.is_empty() { "None" } else { &regime_result.flags.join(", ") }
        )
    }

    /// Parse ChatGPT response for regime analysis
    fn parse_regime_response(&self, response: &str, regime_result: &RegimeTTSResult) -> Result<ChatGPTAnalysis> {
        let lines: Vec<&str> = response.lines().collect();
        let mut regime_analysis = String::new();
        let mut tts_interpretation = String::new();
        let mut trading_recommendation = String::new();

        let mut current_section = "";
        
        for line in lines {
            let line = line.trim();
            if line.starts_with("REGIME ANALYSIS:") {
                current_section = "regime";
                regime_analysis = line.replace("REGIME ANALYSIS:", "").trim().to_string();
            } else if line.starts_with("TTS INTERPRETATION:") {
                current_section = "tts";
                tts_interpretation = line.replace("TTS INTERPRETATION:", "").trim().to_string();
            } else if line.starts_with("TRADING RECOMMENDATION:") {
                current_section = "trading";
                trading_recommendation = line.replace("TRADING RECOMMENDATION:", "").trim().to_string();
            } else if !line.is_empty() && !current_section.is_empty() {
                match current_section {
                    "regime" => regime_analysis.push_str(&format!(" {}", line)),
                    "tts" => tts_interpretation.push_str(&format!(" {}", line)),
                    "trading" => trading_recommendation.push_str(&format!(" {}", line)),
                    _ => {}
                }
            }
        }

        // If parsing failed, use fallback
        if regime_analysis.is_empty() || tts_interpretation.is_empty() || trading_recommendation.is_empty() {
            return Ok(self.generate_fallback_regime_analysis(regime_result));
        }

        Ok(ChatGPTAnalysis::new(
            regime_analysis.trim().to_string(),
            tts_interpretation.trim().to_string(),
            trading_recommendation.trim().to_string(),
            "gpt-4".to_string(),
        ))
    }

    /// Generate fallback regime analysis when ChatGPT fails
    fn generate_fallback_regime_analysis(&self, regime_result: &RegimeTTSResult) -> ChatGPTAnalysis {
        let regime_analysis = format!(
            "Market regime analysis for {} indicates a {} market environment with a {:.2}x multiplier effect on the TTS score. The {} regime suggests {}",
            regime_result.symbol,
            regime_result.market_regime,
            regime_result.market_regime.tts_multiplier(),
            regime_result.market_regime,
            regime_result.market_regime.description()
        );

        let tts_interpretation = format!(
            "The TTS score of {:.3} for {} indicates a {} signal. This score is derived from technical analysis components: momentum ({:.3}), volatility ({:.3}), volume ({:.3}), support/resistance ({:.3}), and market correlation ({:.3}). The confidence level is {:.1}%.",
            regime_result.tts_score,
            regime_result.symbol,
            regime_result.trading_signal.description(),
            regime_result.components.momentum_score,
            regime_result.components.volatility_score,
            regime_result.components.volume_score,
            regime_result.components.support_resistance_score,
            regime_result.components.market_correlation_score,
            regime_result.confidence_score * 100.0
        );

        let trading_recommendation = format!(
            "Based on the {} market regime and {} TTS signal, consider {} position sizing. Risk level is {} with a recommended stop loss at ${:.2} and {:.1}% position size. Monitor for: {}",
            regime_result.market_regime,
            regime_result.trading_signal.description(),
            if regime_result.risk_assessment.position_size > 0.5 { "increasing" } else if regime_result.risk_assessment.position_size < 0.3 { "reducing" } else { "maintaining current" },
            regime_result.risk_assessment.risk_level,
            regime_result.risk_assessment.stop_loss.unwrap_or(0.0),
            regime_result.risk_assessment.position_size * 100.0,
            if regime_result.flags.is_empty() { "no specific warnings" } else { &regime_result.flags.join(", ") }
        );

        ChatGPTAnalysis::new(
            regime_analysis,
            tts_interpretation,
            trading_recommendation,
            "fallback".to_string(),
        )
    }
    */
}

impl Default for ChatGPTService {
    fn default() -> Self {
        let api_key = std::env::var("OPENAI_API_KEY").unwrap_or_default();
        Self::new(api_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::timing::models::*;
    use chrono::Utc;

    fn create_mock_tts_result() -> TTSResult {
        TTSResult {
            symbol: "AAPL".to_string(),
            tts_score: 65.5,
            trading_signal: TTSSignal::Buy,
            indicators: TTSIndicators {
                rsi_score: 70.0,
                macd_score: 80.0,
                bollinger_score: 60.0,
                ma_score: 75.0,
                stochastic_score: 65.0,
                williams_score: 70.0,
                atr_score: 55.0,
                volume_score: 80.0,
            },
            trend_analysis: TrendAnalysis {
                short_term: TrendDirection::Bullish,
                medium_term: TrendDirection::Bullish,
                long_term: TrendDirection::Neutral,
                strength: 75.0,
                consistency: 80.0,
            },
            support_resistance: SupportResistance {
                support_level: 150.0,
                resistance_level: 200.0,
                support_distance: 5.0,
                resistance_distance: 10.0,
                support_strength: 80.0,
                resistance_strength: 70.0,
            },
            volume_analysis: VolumeAnalysis {
                current_volume: 50000000,
                avg_volume: 45000000,
                volume_ratio: 1.11,
                volume_trend: VolumeTrend::Increasing,
                vp_relationship: VolumePriceRelationship::BullishDivergence,
            },
            risk_assessment: RiskAssessment {
                volatility_score: 60.0,
                risk_level: RiskLevel::Medium,
                max_drawdown_risk: 8.5,
                stop_loss: 145.0,
                risk_reward_ratio: 2.0,
            },
            timestamp: Utc::now(),
            confidence_score: 0.85,
            flags: vec!["High volume confirmation".to_string()],
        }
    }

    #[test]
    fn test_fallback_explanations() {
        let service = ChatGPTService::new("test_key".to_string());
        let tts_result = create_mock_tts_result();
        
        let (explanation, suggestion) = service.generate_fallback_explanations(&tts_result);
        
        assert!(explanation.contains("AAPL"));
        assert!(explanation.contains("65.5"));
        assert!(suggestion.contains("stop loss"));
    }

    #[test]
    fn test_prompt_creation() {
        let service = ChatGPTService::new("test_key".to_string());
        let tts_result = create_mock_tts_result();
        
        let prompt = service.create_tts_prompt(&tts_result);
        
        assert!(prompt.contains("AAPL"));
        assert!(prompt.contains("65.5"));
        assert!(prompt.contains("EXPLANATION:"));
        assert!(prompt.contains("SUGGESTION:"));
    }
}
