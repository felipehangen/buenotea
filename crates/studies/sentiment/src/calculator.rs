// Simplified QSS (Quantitative Sentiment Score) calculator implementation
// This module contains the core logic for calculating sentiment scores

use buenotea_core::Result;
use super::models::*;
use chrono::Utc;
use std::time::Instant;
use tracing::{info, warn};
use reqwest::Client;
use serde_json::Value;

// Helper structs for detailed data collection
#[derive(Debug, Default)]
struct RSIData {
    rsi: Option<f64>,
    source: Option<String>,
}

#[derive(Debug, Default)]
struct PriceData {
    current_price: Option<f64>,
    price_15d_ago: Option<f64>,
    price_30d_ago: Option<f64>,
    return_15d: Option<f64>,
    return_30d: Option<f64>,
    volume_ratio: Option<f64>,
}

#[derive(Debug, Default)]
struct EarningsData {
    current_eps: Option<f64>,
    previous_eps: Option<f64>,
    eps_change_percentage: Option<f64>,
    current_revenue: Option<i64>,
    previous_revenue: Option<i64>,
    revenue_change_percentage: Option<f64>,
    analyst_count: Option<i32>,
}

/// Main QSS calculator that combines multiple data sources
pub struct QSSCalculator {
    client: Client,
}

impl QSSCalculator {
    /// Create a new QSS calculator
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Calculate the complete QSS score for a given symbol
    pub async fn calculate_qss(&self, symbol: &str) -> Result<QSSResult> {
        let start_time = Instant::now();
        info!("Starting QSS calculation for {}", symbol);

        // Simplified calculation - using mock data for now
        // In a real implementation, this would fetch data from APIs
        
        // Step 1: Calculate earnings revisions (40% weight)
        let earnings_revisions = self.calculate_earnings_revisions(symbol).await?;
        
        // Step 2: Calculate relative strength (30% weight)
        let relative_strength = self.calculate_relative_strength(symbol).await?;
        
        // Step 3: Calculate short interest (20% weight)
        let short_interest = self.calculate_short_interest(symbol).await?;
        
        // Step 4: Calculate options flow (10% weight)
        let options_flow = self.calculate_options_flow(symbol).await?;

        // Calculate final QSS score
        let components = QSSComponents {
            earnings_revisions,
            relative_strength,
            short_interest,
            options_flow,
        };

        let qss_score = components.calculate_qss();
        let trading_signal = self.generate_trading_signal(qss_score);
        let confidence_score = self.calculate_confidence_score(&components);

        let computation_time = start_time.elapsed().as_millis() as u64;

        // Collect additional detailed data
        let (rsi_data, price_data, earnings_data) = self.collect_detailed_data(symbol).await?;
        
        // Collect market benchmark data
        let (market_benchmark, sector_benchmark) = self.collect_market_benchmark_data(symbol).await?;
        
        // Calculate relative performance
        let relative_to_market = if let (Some(stock_return), Some(market_return)) = (price_data.return_15d, market_benchmark) {
            Some(stock_return * 100.0 - market_return) // Stock return vs market return
        } else {
            None
        };
        
        let relative_to_sector = if let (Some(stock_return), Some(sector_return)) = (price_data.return_15d, sector_benchmark) {
            Some(stock_return * 100.0 - sector_return) // Stock return vs sector return
        } else {
            None
        };
        
        // Create metadata with detailed data
        let meta = QSSMeta {
            computation_time_ms: computation_time,
            data_points_count: components.valid_components_count(),
            trend_direction: qss_score,
            data_freshness: 0.95,
            rsi_14: rsi_data.rsi,
            rsi_source: rsi_data.source,
            current_price: price_data.current_price,
            price_15d_ago: price_data.price_15d_ago,
            price_30d_ago: price_data.price_30d_ago,
            return_15d: price_data.return_15d,
            return_30d: price_data.return_30d,
            current_eps_estimate: earnings_data.current_eps,
            previous_eps_estimate: earnings_data.previous_eps,
            eps_change_percentage: earnings_data.eps_change_percentage,
            current_revenue_estimate: earnings_data.current_revenue,
            previous_revenue_estimate: earnings_data.previous_revenue,
            revenue_change_percentage: earnings_data.revenue_change_percentage,
            analyst_count: earnings_data.analyst_count,
            market_benchmark_return: market_benchmark,
            sector_benchmark_return: sector_benchmark,
            relative_to_market,
            relative_to_sector,
            volume_ratio: price_data.volume_ratio,
        };

        let flags = vec![
            "no_short_data".to_string(),
            "no_options_data".to_string(),
        ];

        Ok(QSSResult {
            symbol: symbol.to_string(),
            qss_score,
            trading_signal,
            components,
            flags,
            confidence_score,
            timestamp: Utc::now(),
            meta,
        })
    }

    async fn calculate_earnings_revisions(&self, symbol: &str) -> Result<f64> {
        info!("Fetching earnings estimates for {}", symbol);
        
        // Try Alpha Vantage first
        if let Ok(alpha_key) = std::env::var("ALPHA_VANTAGE_API_KEY") {
            match self.get_alpha_vantage_earnings(symbol, &alpha_key).await {
                Ok(score) => {
                    info!("✅ Got earnings data from Alpha Vantage for {}: {}", symbol, score);
                    return Ok(score);
                }
                Err(e) => {
                    warn!("❌ Alpha Vantage failed for {}: {}", symbol, e);
                }
            }
        } else {
            warn!("⚠️  No Alpha Vantage API key found");
        }

        // Fallback to FMP
        if let Ok(fmp_key) = std::env::var("FMP_API_KEY") {
            match self.get_fmp_earnings(symbol, &fmp_key).await {
                Ok(score) => {
                    info!("✅ Got earnings data from FMP for {}: {}", symbol, score);
                    return Ok(score);
                }
                Err(e) => {
                    warn!("❌ FMP failed for {}: {}", symbol, e);
                }
            }
        } else {
            warn!("⚠️  No FMP API key found");
        }

        warn!("⚠️  No earnings data available for {}, using 0.0", symbol);
        Ok(0.0)
    }

    async fn get_alpha_vantage_earnings(&self, symbol: &str, api_key: &str) -> Result<f64> {
        let url = format!(
            "https://www.alphavantage.co/query?function=EARNINGS_ESTIMATES&symbol={}&apikey={}",
            symbol, api_key
        );

        info!("🔍 Calling Alpha Vantage earnings API: {}", url);
        let response = self.client.get(&url).send().await?;
        let json: Value = response.json().await?;

        // Log the response to see what we're getting
        info!("📊 Alpha Vantage response keys: {:?}", json.as_object().map(|o| o.keys().collect::<Vec<_>>()));

        // Check for error messages
        if let Some(error_msg) = json.get("Error Message") {
            warn!("❌ Alpha Vantage error: {}", error_msg);
            return Err(buenotea_core::Error::ApiError(
                "Alpha Vantage".to_string(),
                error_msg.as_str().unwrap_or("Unknown error").to_string(),
            ));
        }

        if let Some(note) = json.get("Note") {
            warn!("⚠️  Alpha Vantage rate limit: {}", note);
            return Err(buenotea_core::Error::RateLimitExceeded("Alpha Vantage".to_string()));
        }

        // Try different earnings endpoints
        let endpoints = vec![
            "annualEarningsEstimates",
            "quarterlyEarningsEstimates", 
            "annualEarnings",
            "quarterlyEarnings"
        ];

        for endpoint in endpoints {
            if let Some(data) = json.get(endpoint) {
                info!("📈 Found {} data for {}", endpoint, symbol);
                
                if let Some(estimates_array) = data.as_array() {
                    info!("📊 {} has {} records", endpoint, estimates_array.len());
                    
                    if estimates_array.len() >= 2 {
                        // Try to get EPS data
                        let current = estimates_array[0].get("estimatedEps")
                            .or_else(|| estimates_array[0].get("reportedEPS"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("0");
                        let previous = estimates_array[1].get("estimatedEps")
                            .or_else(|| estimates_array[1].get("reportedEPS"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("0");
                        
                        info!("📊 Current EPS: {}, Previous EPS: {}", current, previous);
                        
                        let current_eps: f64 = current.parse().unwrap_or(0.0);
                        let previous_eps: f64 = previous.parse().unwrap_or(0.0);
                        
                        if previous_eps != 0.0 {
                            let revision = (current_eps - previous_eps) / previous_eps.abs();
                            let normalized = revision.max(-1.0).min(1.0);
                            info!("✅ Calculated earnings revision: {} (normalized: {})", revision, normalized);
                            return Ok(normalized);
                        }
                    }
                }
            }
        }

        warn!("⚠️  No earnings data found in Alpha Vantage response");
        Err(buenotea_core::Error::InvalidResponseFormat(
            "Alpha Vantage".to_string(),
            "No earnings estimates found".to_string(),
        ))
    }

    async fn get_fmp_earnings(&self, symbol: &str, api_key: &str) -> Result<f64> {
        // Try multiple FMP endpoints for earnings data
        let endpoints = vec![
            format!("https://financialmodelingprep.com/api/v3/analyst-estimates/{}?apikey={}", symbol, api_key),
            format!("https://financialmodelingprep.com/api/v3/earnings-surprises/{}?apikey={}", symbol, api_key),
            format!("https://financialmodelingprep.com/api/v3/income-statement/{}?apikey={}", symbol, api_key),
        ];

        for url in endpoints {
            info!("🔍 Calling FMP API: {}", url);
            
            match self.client.get(&url).send().await {
                Ok(response) => {
                    let json: Value = response.json().await?;
                    info!("📊 FMP response type: {}", json.get("symbol").unwrap_or(&serde_json::Value::Null));
                    
                    if let Some(estimates_array) = json.as_array() {
                        info!("📊 FMP returned {} records", estimates_array.len());
                        
                        if estimates_array.len() >= 2 {
                            // Try different EPS field names
                            let eps_fields = vec!["estimatedEps", "eps", "epsActual", "reportedEPS"];
                            
                            for field in eps_fields {
                                let current = estimates_array[0].get(field).and_then(|v| v.as_f64()).unwrap_or(0.0);
                                let previous = estimates_array[1].get(field).and_then(|v| v.as_f64()).unwrap_or(0.0);
                                
                                if previous != 0.0 && current != 0.0 {
                                    let revision = (current - previous) / previous.abs();
                                    let normalized = revision.max(-1.0).min(1.0);
                                    info!("✅ FMP earnings revision from {}: {} (normalized: {})", field, revision, normalized);
                                    return Ok(normalized);
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    warn!("❌ FMP API call failed: {}", e);
                }
            }
        }

        Err(buenotea_core::Error::InvalidResponseFormat(
            "FMP".to_string(),
            "No earnings data found in any FMP endpoint".to_string(),
        ))
    }

    async fn calculate_relative_strength(&self, symbol: &str) -> Result<f64> {
        info!("Calculating relative strength for {}", symbol);
        
        // Try FMP for historical price data
        if let Ok(fmp_key) = std::env::var("FMP_API_KEY") {
            match self.get_fmp_relative_strength(symbol, &fmp_key).await {
                Ok(score) => {
                    info!("✅ Got relative strength from FMP for {}: {}", symbol, score);
                    return Ok(score);
                }
                Err(e) => {
                    warn!("❌ FMP relative strength failed for {}: {}", symbol, e);
                }
            }
        } else {
            warn!("⚠️  No FMP API key found");
        }

        warn!("⚠️  No relative strength data available for {}, using 0.0", symbol);
        Ok(0.0)
    }

    async fn get_fmp_relative_strength(&self, symbol: &str, api_key: &str) -> Result<f64> {
        // Try multiple approaches for relative strength
        let endpoints = vec![
            format!("https://financialmodelingprep.com/api/v3/historical-price-full/{}?apikey={}", symbol, api_key),
            format!("https://financialmodelingprep.com/api/v3/quote/{}?apikey={}", symbol, api_key),
            format!("https://financialmodelingprep.com/api/v3/technical-indicator/{}?period=14&type=rsi&apikey={}", symbol, api_key),
        ];

        for (i, url) in endpoints.iter().enumerate() {
            info!("🔍 Calling FMP price API {}: {}", i+1, url);
            
            match self.client.get(url).send().await {
                Ok(response) => {
                    let json: Value = response.json().await?;
                    info!("📊 FMP price response keys: {:?}", json.as_object().map(|o| o.keys().collect::<Vec<_>>()));
                    
                    // Try historical data for RSI calculation
                    if let Some(historical_data) = json.get("historical") {
                        if let Some(prices_array) = historical_data.as_array() {
                            info!("📊 Found {} historical price records", prices_array.len());
                            
                            if prices_array.len() >= 15 {
                                let mut closes: Vec<f64> = Vec::new();
                                
                                // Extract closing prices (most recent first)
                                for price_data in prices_array.iter().take(30) { // Get more data for better RSI
                                    if let Some(close) = price_data.get("close").and_then(|v| v.as_f64()) {
                                        closes.push(close);
                                    }
                                }
                                
                                if closes.len() >= 14 {
                                    let rsi = self.calculate_rsi(&closes);
                                    info!("📈 Calculated RSI: {}", rsi);
                                    
                                    // Also calculate price momentum
                                    let price_change_14d = (closes[0] - closes[13]) / closes[13] * 100.0;
                                    let price_change_30d = if closes.len() > 29 { (closes[0] - closes[29]) / closes[29] * 100.0 } else { price_change_14d };
                                    
                                    info!("📊 14-day price change: {:.2}%, 30-day: {:.2}%", price_change_14d, price_change_30d);
                                    
                                    // Combine RSI and price momentum for sentiment
                                    let rsi_sentiment = if rsi > 70.0 {
                                        -((rsi - 70.0) / 30.0).min(1.0)
                                    } else if rsi < 30.0 {
                                        (30.0 - rsi) / 30.0
                                    } else {
                                        0.0
                                    };
                                    
                                    let momentum_sentiment = (price_change_14d / 10.0).max(-1.0).min(1.0); // Normalize to [-1,1]
                                    
                                    // Weighted combination: 70% RSI, 30% momentum
                                    let combined_sentiment = 0.7 * rsi_sentiment + 0.3 * momentum_sentiment;
                                    
                                    info!("✅ Combined relative strength: {} (RSI: {}, Momentum: {})", combined_sentiment, rsi_sentiment, momentum_sentiment);
                                    return Ok(combined_sentiment);
                                }
                            }
                        }
                    }
                    
                    // Try direct quote data for current price and change
                    if let Some(quote_array) = json.as_array() {
                        if let Some(quote) = quote_array.first() {
                            if let Some(price_change_percent) = quote.get("changesPercentage").and_then(|v| v.as_f64()) {
                                info!("📊 Found direct price change: {}%", price_change_percent);
                                let sentiment = (price_change_percent / 10.0).max(-1.0).min(1.0);
                                return Ok(sentiment);
                            }
                        }
                    }
                }
                Err(e) => {
                    warn!("❌ FMP price API call {} failed: {}", i+1, e);
                }
            }
        }

        Err(buenotea_core::Error::InvalidResponseFormat(
            "FMP".to_string(),
            "No price data found in any FMP endpoint".to_string(),
        ))
    }

    fn calculate_rsi(&self, prices: &[f64]) -> f64 {
        if prices.len() < 14 {
            return 50.0; // Default neutral RSI
        }

        let mut gains = Vec::new();
        let mut losses = Vec::new();

        // Calculate price changes
        for i in 1..prices.len() {
            let change = prices[i-1] - prices[i]; // prices[0] is most recent
            if change > 0.0 {
                gains.push(change);
                losses.push(0.0);
            } else {
                gains.push(0.0);
                losses.push(-change);
            }
        }

        // Calculate average gain and loss over 14 periods
        let avg_gain: f64 = gains.iter().take(14).sum::<f64>() / 14.0;
        let avg_loss: f64 = losses.iter().take(14).sum::<f64>() / 14.0;

        if avg_loss == 0.0 {
            return 100.0;
        }

        let rs = avg_gain / avg_loss;
        100.0 - (100.0 / (1.0 + rs))
    }

    async fn calculate_short_interest(&self, symbol: &str) -> Result<f64> {
        info!("Trying to get short interest data for {}", symbol);
        
        // Try Finnhub for basic sentiment data
        if let Ok(finnhub_key) = std::env::var("FINNHUB_API_KEY") {
            match self.get_finnhub_sentiment(symbol, &finnhub_key).await {
                Ok(score) => {
                    info!("✅ Got sentiment data from Finnhub for {}: {}", symbol, score);
                    return Ok(score);
                }
                Err(e) => {
                    warn!("❌ Finnhub sentiment failed for {}: {}", symbol, e);
                }
            }
        } else {
            warn!("⚠️  No Finnhub API key found");
        }

        warn!("⚠️  No short interest data available for {}, using 0.0", symbol);
        Ok(0.0)
    }

    async fn calculate_options_flow(&self, symbol: &str) -> Result<f64> {
        info!("Trying to get options flow data for {}", symbol);
        
        // Try to get analyst recommendations as a proxy for options sentiment
        if let Ok(fmp_key) = std::env::var("FMP_API_KEY") {
            match self.get_fmp_analyst_recommendations(symbol, &fmp_key).await {
                Ok(score) => {
                    info!("✅ Got analyst recommendations from FMP for {}: {}", symbol, score);
                    return Ok(score);
                }
                Err(e) => {
                    warn!("❌ FMP analyst recommendations failed for {}: {}", symbol, e);
                }
            }
        }

        warn!("⚠️  No options flow data available for {}, using 0.0", symbol);
        Ok(0.0)
    }

    async fn get_finnhub_sentiment(&self, symbol: &str, api_key: &str) -> Result<f64> {
        // Try Finnhub news sentiment
        let url = format!(
            "https://finnhub.io/api/v1/company-news?symbol={}&from={}&to={}&token={}",
            symbol,
            chrono::Utc::now().date_naive() - chrono::Duration::days(7),
            chrono::Utc::now().date_naive(),
            api_key
        );

        info!("🔍 Calling Finnhub news API: {}", url);
        let response = self.client.get(&url).send().await?;
        let json: Value = response.json().await?;

        if let Some(news_array) = json.as_array() {
            info!("📰 Found {} news articles", news_array.len());
            
            if !news_array.is_empty() {
                // Simple sentiment analysis based on news sentiment scores
                let mut total_sentiment = 0.0;
                let mut valid_articles = 0;
                
                for article in news_array.iter().take(10) { // Limit to recent articles
                    if let Some(sentiment) = article.get("sentiment").and_then(|v| v.as_f64()) {
                        total_sentiment += sentiment;
                        valid_articles += 1;
                    }
                }
                
                if valid_articles > 0 {
                    let avg_sentiment = total_sentiment / valid_articles as f64;
                    // Convert Finnhub sentiment (-1 to 1) to our scale
                    info!("📊 Average news sentiment: {} from {} articles", avg_sentiment, valid_articles);
                    return Ok(avg_sentiment);
                }
            }
        }

        Err(buenotea_core::Error::InvalidResponseFormat(
            "Finnhub".to_string(),
            "No news sentiment data found".to_string(),
        ))
    }

    async fn get_fmp_analyst_recommendations(&self, symbol: &str, api_key: &str) -> Result<f64> {
        let url = format!(
            "https://financialmodelingprep.com/api/v3/analyst-stock-recommendations/{}?apikey={}",
            symbol, api_key
        );

        info!("🔍 Calling FMP analyst recommendations API: {}", url);
        let response = self.client.get(&url).send().await?;
        let json: Value = response.json().await?;

        if let Some(recommendations_array) = json.as_array() {
            info!("📊 Found {} analyst recommendations", recommendations_array.len());
            
            if !recommendations_array.is_empty() {
                let mut total_score = 0.0;
                let mut count = 0;
                
                for rec in recommendations_array.iter().take(5) { // Recent recommendations
                    if let Some(rating) = rec.get("rating").and_then(|v| v.as_str()) {
                        let score = match rating.to_uppercase().as_str() {
                            "STRONG_BUY" | "BUY" => 1.0,
                            "HOLD" => 0.0,
                            "SELL" | "STRONG_SELL" => -1.0,
                            _ => 0.0,
                        };
                        total_score += score;
                        count += 1;
                        info!("📊 Recommendation: {} -> score: {}", rating, score);
                    }
                }
                
                if count > 0 {
                    let avg_score = total_score / count as f64;
                    info!("📊 Average analyst score: {} from {} recommendations", avg_score, count);
                    return Ok(avg_score);
                }
            }
        }

        Err(buenotea_core::Error::InvalidResponseFormat(
            "FMP".to_string(),
            "No analyst recommendations found".to_string(),
        ))
    }

    fn generate_trading_signal(&self, qss_score: f64) -> TradingSignal {
        match qss_score {
            x if x >= 0.5 => TradingSignal::StrongBuy,
            x if x >= 0.2 => TradingSignal::WeakBuy,
            x if x <= -0.5 => TradingSignal::StrongSell,
            x if x <= -0.2 => TradingSignal::WeakSell,
            _ => TradingSignal::Hold,
        }
    }

    fn calculate_confidence_score(&self, components: &QSSComponents) -> f64 {
        // Simple confidence based on number of valid components
        let valid_count = components.valid_components_count();
        match valid_count {
            4 => 1.0,  // All components available
            3 => 0.8,  // Most components available
            2 => 0.6,  // Half components available
            1 => 0.4,  // Few components available
            _ => 0.2,  // Very few components available
        }
    }

    async fn collect_detailed_data(&self, symbol: &str) -> Result<(RSIData, PriceData, EarningsData)> {
        info!("🔍 Collecting detailed data for {}", symbol);
        
        let mut rsi_data = RSIData::default();
        let mut price_data = PriceData::default();
        let mut earnings_data = EarningsData::default();

        // Collect RSI and price data from FMP
        if let Ok(fmp_key) = std::env::var("FMP_API_KEY") {
            if let Ok(response) = self.client
                .get(&format!("https://financialmodelingprep.com/api/v3/historical-price-full/{}?apikey={}", symbol, fmp_key))
                .send()
                .await
            {
                if let Ok(json) = response.json::<Value>().await {
                    if let Some(historical_data) = json.get("historical") {
                        if let Some(prices_array) = historical_data.as_array() {
                            if prices_array.len() >= 30 {
                                let mut closes: Vec<f64> = Vec::new();
                                for price_entry in prices_array.iter().take(30) {
                                    if let Some(close) = price_entry.get("close").and_then(|v| v.as_f64()) {
                                        closes.push(close);
                                    }
                                }
                                
                                if closes.len() >= 14 {
                                    rsi_data.rsi = Some(self.calculate_rsi(&closes));
                                    rsi_data.source = Some("FMP".to_string());
                                    
                                    // Extract price and volume data
                                    price_data.current_price = closes.first().copied();
                                    price_data.price_15d_ago = closes.get(14).copied();
                                    price_data.price_30d_ago = closes.get(29).copied();
                                    
                                    if let (Some(current), Some(price_15d)) = (price_data.current_price, price_data.price_15d_ago) {
                                        price_data.return_15d = Some((current - price_15d) / price_15d);
                                    }
                                    
                                    if let (Some(current), Some(price_30d)) = (price_data.current_price, price_data.price_30d_ago) {
                                        price_data.return_30d = Some((current - price_30d) / price_30d);
                                    }
                                    
                                    // Calculate volume ratio (current volume vs 30-day average)
                                    let mut volumes: Vec<i64> = Vec::new();
                                    for price_entry in prices_array.iter().take(30) {
                                        if let Some(volume) = price_entry.get("volume").and_then(|v| v.as_i64()) {
                                            volumes.push(volume);
                                        }
                                    }
                                    
                                    if volumes.len() >= 30 {
                                        let current_volume = volumes[0];
                                        let avg_volume_30d = volumes.iter().sum::<i64>() as f64 / volumes.len() as f64;
                                        price_data.volume_ratio = Some(current_volume as f64 / avg_volume_30d);
                                        
                                        info!("✅ Collected volume data: current volume={}, 30d avg={:.0}, ratio={:.2}", 
                                              current_volume, avg_volume_30d, price_data.volume_ratio.unwrap_or(0.0));
                                    }
                                    
                                    info!("✅ Collected price data: current=${:.2}, 15d return={:.2}%", 
                                          price_data.current_price.unwrap_or(0.0),
                                          price_data.return_15d.unwrap_or(0.0) * 100.0);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Collect earnings and revenue data from Alpha Vantage
        if let Ok(alpha_key) = std::env::var("ALPHA_VANTAGE_API_KEY") {
            // Try earnings estimates first
            if let Ok(response) = self.client
                .get(&format!("https://www.alphavantage.co/query?function=EARNINGS_ESTIMATES&symbol={}&apikey={}", symbol, alpha_key))
                .send()
                .await
            {
                if let Ok(json) = response.json::<Value>().await {
                    // Try different earnings endpoints
                    let endpoints = vec!["annualEarningsEstimates", "quarterlyEarningsEstimates"];
                    
                    for endpoint in endpoints {
                        if let Some(data) = json.get(endpoint) {
                            if let Some(estimates_array) = data.as_array() {
                                if estimates_array.len() >= 2 {
                                    let current = estimates_array[0].get("estimatedEps").and_then(|v| v.as_str()).unwrap_or("0");
                                    let previous = estimates_array[1].get("estimatedEps").and_then(|v| v.as_str()).unwrap_or("0");
                                    
                                    let current_eps: f64 = current.parse().unwrap_or(0.0);
                                    let previous_eps: f64 = previous.parse().unwrap_or(0.0);
                                    
                                    if current_eps != 0.0 && previous_eps != 0.0 {
                                        earnings_data.current_eps = Some(current_eps);
                                        earnings_data.previous_eps = Some(previous_eps);
                                        earnings_data.eps_change_percentage = Some((current_eps - previous_eps) / previous_eps.abs() * 100.0);
                                        
                                        info!("✅ Collected Alpha Vantage earnings data: current EPS=${:.2}, change={:.2}%", 
                                              current_eps, earnings_data.eps_change_percentage.unwrap_or(0.0));
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Try earnings history as fallback
            if earnings_data.current_eps.is_none() {
                if let Ok(response) = self.client
                    .get(&format!("https://www.alphavantage.co/query?function=EARNINGS&symbol={}&apikey={}", symbol, alpha_key))
                    .send()
                    .await
                {
                    if let Ok(json) = response.json::<Value>().await {
                        if let Some(annual_earnings) = json.get("annualEarnings") {
                            if let Some(earnings_array) = annual_earnings.as_array() {
                                if earnings_array.len() >= 2 {
                                    let current = earnings_array[0].get("reportedEPS").and_then(|v| v.as_str()).unwrap_or("0");
                                    let previous = earnings_array[1].get("reportedEPS").and_then(|v| v.as_str()).unwrap_or("0");
                                    
                                    let current_eps: f64 = current.parse().unwrap_or(0.0);
                                    let previous_eps: f64 = previous.parse().unwrap_or(0.0);
                                    
                                    if current_eps != 0.0 && previous_eps != 0.0 {
                                        earnings_data.current_eps = Some(current_eps);
                                        earnings_data.previous_eps = Some(previous_eps);
                                        earnings_data.eps_change_percentage = Some((current_eps - previous_eps) / previous_eps.abs() * 100.0);
                                        
                                        info!("✅ Collected Alpha Vantage earnings history: current EPS=${:.2}, change={:.2}%", 
                                              current_eps, earnings_data.eps_change_percentage.unwrap_or(0.0));
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Try income statement for revenue data
            if let Ok(response) = self.client
                .get(&format!("https://www.alphavantage.co/query?function=INCOME_STATEMENT&symbol={}&apikey={}", symbol, alpha_key))
                .send()
                .await
            {
                if let Ok(json) = response.json::<Value>().await {
                    if let Some(annual_reports) = json.get("annualReports") {
                        if let Some(reports_array) = annual_reports.as_array() {
                            if reports_array.len() >= 2 {
                                let current = reports_array[0].get("totalRevenue").and_then(|v| v.as_str()).unwrap_or("0");
                                let previous = reports_array[1].get("totalRevenue").and_then(|v| v.as_str()).unwrap_or("0");
                                
                                let current_revenue: i64 = current.parse().unwrap_or(0);
                                let previous_revenue: i64 = previous.parse().unwrap_or(0);
                                
                                if current_revenue != 0 && previous_revenue != 0 {
                                    earnings_data.current_revenue = Some(current_revenue);
                                    earnings_data.previous_revenue = Some(previous_revenue);
                                    earnings_data.revenue_change_percentage = Some((current_revenue as f64 - previous_revenue as f64) / previous_revenue as f64 * 100.0);
                                    
                                    info!("✅ Collected revenue data: current=${}, change={:.2}%", 
                                          current_revenue, earnings_data.revenue_change_percentage.unwrap_or(0.0));
                                }
                            }
                        }
                    }
                }
            }
        }

        // Try Finnhub for revenue estimates
        if let Ok(finnhub_key) = std::env::var("FINNHUB_API_KEY") {
            if let Ok(response) = self.client
                .get(&format!("https://finnhub.io/api/v1/company-revenue-estimates?symbol={}&token={}", symbol, finnhub_key))
                .send()
                .await
            {
                if let Ok(json) = response.json::<Value>().await {
                    if let Some(revenue_estimates) = json.get("data") {
                        if let Some(estimates_array) = revenue_estimates.as_array() {
                            if estimates_array.len() >= 2 {
                                let current = estimates_array[0].get("revenue").and_then(|v| v.as_i64()).unwrap_or(0);
                                let previous = estimates_array[1].get("revenue").and_then(|v| v.as_i64()).unwrap_or(0);
                                
                                if current != 0 && previous != 0 {
                                    // Only update if we don't already have revenue data from Alpha Vantage
                                    if earnings_data.current_revenue.is_none() {
                                        earnings_data.current_revenue = Some(current);
                                        earnings_data.previous_revenue = Some(previous);
                                        earnings_data.revenue_change_percentage = Some((current as f64 - previous as f64) / previous as f64 * 100.0);
                                        
                                        info!("✅ Collected Finnhub revenue estimates: current=${}, change={:.2}%", 
                                              current, earnings_data.revenue_change_percentage.unwrap_or(0.0));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Collect analyst recommendations and earnings estimates from FMP
        if let Ok(fmp_key) = std::env::var("FMP_API_KEY") {
            // Get analyst recommendations count
            if let Ok(response) = self.client
                .get(&format!("https://financialmodelingprep.com/api/v3/analyst-stock-recommendations/{}?apikey={}", symbol, fmp_key))
                .send()
                .await
            {
                if let Ok(json) = response.json::<Value>().await {
                    if let Some(recommendations_array) = json.as_array() {
                        earnings_data.analyst_count = Some(recommendations_array.len() as i32);
                        info!("✅ Found {} analyst recommendations", recommendations_array.len());
                    }
                }
            }

            // Try FMP earnings estimates if we don't have EPS data yet
            if earnings_data.current_eps.is_none() {
                if let Ok(response) = self.client
                    .get(&format!("https://financialmodelingprep.com/api/v3/analyst-estimates/{}?apikey={}", symbol, fmp_key))
                    .send()
                    .await
                {
                    if let Ok(json) = response.json::<Value>().await {
                        if let Some(estimates_array) = json.as_array() {
                            if estimates_array.len() >= 2 {
                                let current = estimates_array[0].get("estimatedEps").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                let previous = estimates_array[1].get("estimatedEps").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                
                                if current != 0.0 && previous != 0.0 {
                                    earnings_data.current_eps = Some(current);
                                    earnings_data.previous_eps = Some(previous);
                                    earnings_data.eps_change_percentage = Some((current - previous) / previous.abs() * 100.0);
                                    
                                    info!("✅ Collected FMP earnings estimates: current EPS=${:.2}, change={:.2}%", 
                                          current, earnings_data.eps_change_percentage.unwrap_or(0.0));
                                }
                            }
                        }
                    }
                }
            }

            // Try FMP income statement for revenue if we don't have it yet
            if earnings_data.current_revenue.is_none() {
                if let Ok(response) = self.client
                    .get(&format!("https://financialmodelingprep.com/api/v3/income-statement/{}?apikey={}", symbol, fmp_key))
                    .send()
                    .await
                {
                    if let Ok(json) = response.json::<Value>().await {
                        if let Some(income_statements) = json.as_array() {
                            if income_statements.len() >= 2 {
                                let current = income_statements[0].get("revenue").and_then(|v| v.as_i64()).unwrap_or(0);
                                let previous = income_statements[1].get("revenue").and_then(|v| v.as_i64()).unwrap_or(0);
                                
                                if current != 0 && previous != 0 {
                                    earnings_data.current_revenue = Some(current);
                                    earnings_data.previous_revenue = Some(previous);
                                    earnings_data.revenue_change_percentage = Some((current as f64 - previous as f64) / previous as f64 * 100.0);
                                    
                                    info!("✅ Collected FMP revenue data: current=${}, change={:.2}%", 
                                          current, earnings_data.revenue_change_percentage.unwrap_or(0.0));
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok((rsi_data, price_data, earnings_data))
    }

    async fn collect_market_benchmark_data(&self, symbol: &str) -> Result<(Option<f64>, Option<f64>)> {
        info!("🔍 Collecting market benchmark data for {}", symbol);
        
        let mut market_benchmark = None;
        let mut sector_benchmark = None;

        // Get S&P 500 data for market benchmark using FMP
        if let Ok(fmp_key) = std::env::var("FMP_API_KEY") {
            // Get S&P 500 historical data
            if let Ok(response) = self.client
                .get(&format!("https://financialmodelingprep.com/api/v3/historical-price-full/SPY?apikey={}", fmp_key))
                .send()
                .await
            {
                if let Ok(json) = response.json::<Value>().await {
                    if let Some(historical_data) = json.get("historical") {
                        if let Some(prices_array) = historical_data.as_array() {
                            if prices_array.len() >= 30 {
                                let mut closes: Vec<f64> = Vec::new();
                                for price_entry in prices_array.iter().take(30) {
                                    if let Some(close) = price_entry.get("close").and_then(|v| v.as_f64()) {
                                        closes.push(close);
                                    }
                                }
                                
                                if closes.len() >= 15 {
                                    // Calculate 15-day return for S&P 500
                                    let market_return_15d = (closes[0] - closes[14]) / closes[14];
                                    market_benchmark = Some(market_return_15d * 100.0); // Convert to percentage
                                    
                                    info!("✅ Calculated S&P 500 benchmark return: {:.2}%", market_benchmark.unwrap_or(0.0));
                                }
                            }
                        }
                    }
                }
            }

            // Try to get sector-specific ETF data based on the stock symbol
            let sector_etf = self.get_sector_etf(symbol);
            if let Some(etf) = sector_etf {
                if let Ok(response) = self.client
                    .get(&format!("https://financialmodelingprep.com/api/v3/historical-price-full/{}?apikey={}", etf, fmp_key))
                    .send()
                    .await
                {
                    if let Ok(json) = response.json::<Value>().await {
                        if let Some(historical_data) = json.get("historical") {
                            if let Some(prices_array) = historical_data.as_array() {
                                if prices_array.len() >= 15 {
                                    let mut closes: Vec<f64> = Vec::new();
                                    for price_entry in prices_array.iter().take(15) {
                                        if let Some(close) = price_entry.get("close").and_then(|v| v.as_f64()) {
                                            closes.push(close);
                                        }
                                    }
                                    
                                    if closes.len() >= 15 {
                                        // Calculate 15-day return for sector ETF
                                        let sector_return_15d = (closes[0] - closes[14]) / closes[14];
                                        sector_benchmark = Some(sector_return_15d * 100.0); // Convert to percentage
                                        
                                        info!("✅ Calculated {} sector benchmark return: {:.2}%", etf, sector_benchmark.unwrap_or(0.0));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok((market_benchmark, sector_benchmark))
    }

    fn get_sector_etf(&self, symbol: &str) -> Option<&'static str> {
        // Map major stocks to their sector ETFs
        match symbol {
            "AAPL" | "MSFT" | "GOOGL" | "NVDA" | "TSLA" | "META" | "AMZN" => Some("XLK"), // Technology
            "JPM" | "BAC" | "WFC" | "GS" => Some("XLF"), // Financials
            "JNJ" | "PFE" | "UNH" => Some("XLV"), // Healthcare
            "XOM" | "CVX" | "COP" => Some("XLE"), // Energy
            "KO" | "PEP" | "WMT" => Some("XLP"), // Consumer Staples
            "HD" | "MCD" | "NKE" => Some("XLY"), // Consumer Discretionary
            _ => Some("SPY"), // Default to S&P 500 if sector unknown
        }
    }
}