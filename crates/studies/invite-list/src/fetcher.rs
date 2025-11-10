use buenotea_core::{Error, Result};
use crate::models::{SP500Stock, ApiConfig};
use reqwest;
use serde_json::Value;
use std::collections::HashMap;

/// Fetches S&P 500 stock list from Financial Modeling Prep API
pub struct SP500Fetcher {
    api_config: ApiConfig,
    client: reqwest::Client,
}

impl SP500Fetcher {
    pub fn new(api_config: ApiConfig) -> Self {
        Self {
            api_config,
            client: reqwest::Client::new(),
        }
    }

    /// Fetches the complete S&P 500 stock list
    pub async fn fetch_sp500_list(&self) -> Result<Vec<SP500Stock>> {
        let url = format!(
            "https://financialmodelingprep.com/api/v3/sp500_constituent?apikey={}",
            self.api_config.fmp_api_key
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::ApiError("FMP".to_string(), format!("Failed to fetch S&P 500 list: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::ApiError("FMP".to_string(), format!(
                "API request failed with status: {}",
                response.status()
            )));
        }

        let json_data: Value = response
            .json()
            .await
            .map_err(|e| Error::ApiError("FMP".to_string(), format!("Failed to parse JSON response: {}", e)))?;

        let stocks = self.parse_sp500_response(json_data)?;
        Ok(stocks)
    }

    /// Fetches additional company data for a specific stock
    pub async fn fetch_company_data(&self, symbol: &str) -> Result<Value> {
        let url = format!(
            "https://financialmodelingprep.com/api/v3/profile/{}?apikey={}",
            symbol, self.api_config.fmp_api_key
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::ApiError("FMP".to_string(), format!("Failed to fetch company data for {}: {}", symbol, e)))?;

        if !response.status().is_success() {
            return Err(Error::ApiError("FMP".to_string(), format!(
                "API request failed for {} with status: {}",
                symbol, response.status()
            )));
        }

        let json_data: Value = response
            .json()
            .await
            .map_err(|e| Error::ApiError("FMP".to_string(), format!("Failed to parse JSON response for {}: {}", symbol, e)))?;

        Ok(json_data)
    }

    /// Fetches financial data for a specific stock
    pub async fn fetch_financial_data(&self, symbol: &str) -> Result<Value> {
        let url = format!(
            "https://financialmodelingprep.com/api/v3/ratios/{}?apikey={}",
            symbol, self.api_config.fmp_api_key
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::ApiError("FMP".to_string(), format!("Failed to fetch financial data for {}: {}", symbol, e)))?;

        if !response.status().is_success() {
            return Err(Error::ApiError("FMP".to_string(), format!(
                "API request failed for {} with status: {}",
                symbol, response.status()
            )));
        }

        let json_data: Value = response
            .json()
            .await
            .map_err(|e| Error::ApiError("FMP".to_string(), format!("Failed to parse JSON response for {}: {}", symbol, e)))?;

        Ok(json_data)
    }

    /// Fetches price data for a specific stock
    pub async fn fetch_price_data(&self, symbol: &str) -> Result<Value> {
        let url = format!(
            "https://financialmodelingprep.com/api/v3/historical-price-full/{}?apikey={}",
            symbol, self.api_config.fmp_api_key
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::ApiError("FMP".to_string(), format!("Failed to fetch price data for {}: {}", symbol, e)))?;

        if !response.status().is_success() {
            return Err(Error::ApiError("FMP".to_string(), format!(
                "API request failed for {} with status: {}",
                symbol, response.status()
            )));
        }

        let json_data: Value = response
            .json()
            .await
            .map_err(|e| Error::ApiError("FMP".to_string(), format!("Failed to parse JSON response for {}: {}", symbol, e)))?;

        Ok(json_data)
    }

    /// Parses the S&P 500 API response into SP500Stock structs
    fn parse_sp500_response(&self, json_data: Value) -> Result<Vec<SP500Stock>> {
        let stocks_array = json_data
            .as_array()
            .ok_or_else(|| Error::ApiError("FMP".to_string(), "Expected array in S&P 500 response".to_string()))?;

        let mut stocks = Vec::new();

        for stock_data in stocks_array {
            let stock = SP500Stock {
                symbol: stock_data["symbol"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
                name: stock_data["name"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
                sector: stock_data["sector"]
                    .as_str()
                    .map(|s| s.to_string()),
                industry: stock_data["industry"]
                    .as_str()
                    .map(|s| s.to_string()),
                market_cap: stock_data["marketCap"]
                    .as_i64(),
                current_price: stock_data["price"]
                    .as_f64(),
            };

            if !stock.symbol.is_empty() {
                stocks.push(stock);
            }
        }

        Ok(stocks)
    }

    /// Fetches all data for a single stock (company, financial, price)
    pub async fn fetch_complete_stock_data(&self, symbol: &str) -> Result<(Value, Value, Value)> {
        let (company_data, financial_data, price_data) = tokio::try_join!(
            self.fetch_company_data(symbol),
            self.fetch_financial_data(symbol),
            self.fetch_price_data(symbol)
        )?;

        Ok((company_data, financial_data, price_data))
    }

    /// Fetches data for multiple stocks in parallel (with rate limiting)
    pub async fn fetch_multiple_stocks_data(&self, symbols: &[String]) -> Result<HashMap<String, (Value, Value, Value)>> {
        let mut results = HashMap::new();
        let _tasks: Vec<tokio::task::JoinHandle<Option<(String, (Value, Value, Value))>>> = Vec::new();

        // Process stocks in batches to respect rate limits
        const BATCH_SIZE: usize = 5;
        for chunk in symbols.chunks(BATCH_SIZE) {
            let mut batch_tasks = Vec::new();
            
            for symbol in chunk {
                let fetcher = self.clone();
                let symbol_clone = symbol.clone();
                let task = tokio::spawn(async move {
                    match fetcher.fetch_complete_stock_data(&symbol_clone).await {
                        Ok(data) => Some((symbol_clone, data)),
                        Err(e) => {
                            eprintln!("Failed to fetch data for {}: {}", symbol_clone, e);
                            None
                        }
                    }
                });
                batch_tasks.push(task);
            }

            // Wait for batch to complete
            for task in batch_tasks {
                if let Ok(Some((symbol, data))) = task.await {
                    results.insert(symbol, data);
                }
            }

            // Rate limiting delay between batches
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        }

        Ok(results)
    }
}

impl Clone for SP500Fetcher {
    fn clone(&self) -> Self {
        Self {
            api_config: self.api_config.clone(),
            client: reqwest::Client::new(),
        }
    }
}
