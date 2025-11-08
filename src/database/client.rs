use crate::database::models::{DatabaseConfig, SentimentRecord, SentimentInsert};
use crate::database::timing_models::{TimingRecord, CreateTimingRecord, UpdateTimingRecord, TimingQueryParams};
use crate::database::fundamentals_models::{FundamentalsRecord, FundamentalsInsert};
use crate::error::Result;
use reqwest::Client;
// use serde_json::json; // Not used
use std::collections::HashMap;

pub struct DatabaseClient {
    client: Client,
    config: DatabaseConfig,
}

impl DatabaseClient {
    pub fn new(config: DatabaseConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    pub fn from_env() -> Result<Self> {
        let config = DatabaseConfig::from_env()?;
        Ok(Self::new(config))
    }

    /// Get the HTTP client (for direct API calls)
    pub fn http_client(&self) -> &Client {
        &self.client
    }

    /// Get the database configuration
    pub fn config(&self) -> &DatabaseConfig {
        &self.config
    }

    pub async fn insert_sentiment(&self, record: &SentimentInsert) -> Result<SentimentRecord> {
        let url = format!("{}/rest/v1/{}", self.config.supabase_url, self.config.table_name);
        
        let response = self.client
            .post(&url)
            .header("apikey", &self.config.supabase_api_key)
            .header("Authorization", format!("Bearer {}", self.config.supabase_api_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation")
            .json(record)
            .send()
            .await?;

        if response.status().is_success() {
            let inserted_records: Vec<SentimentRecord> = response.json().await?;
            if let Some(inserted) = inserted_records.first() {
                Ok(inserted.clone())
            } else {
                Err(crate::error::Error::ApiError(
                    "Supabase".to_string(),
                    "No record returned after insertion".to_string(),
                ))
            }
        } else {
            let error_text = response.text().await?;
            Err(crate::error::Error::ApiError(
                "Supabase".to_string(),
                format!("Insert failed: {}", error_text),
            ))
        }
    }

    pub async fn get_latest_sentiment(&self, symbol: &str) -> Result<Option<SentimentRecord>> {
        let url = format!(
            "{}/rest/v1/{}?symbol=eq.{}&order=analysis_date.desc&limit=1",
            self.config.supabase_url, self.config.table_name, symbol
        );
        
        let response = self.client
            .get(&url)
            .header("apikey", &self.config.supabase_api_key)
            .header("Authorization", format!("Bearer {}", self.config.supabase_api_key))
            .send()
            .await?;

        if response.status().is_success() {
            let records: Vec<SentimentRecord> = response.json().await?;
            Ok(records.first().cloned())
        } else {
            let error_text = response.text().await?;
            Err(crate::error::Error::ApiError(
                "Supabase".to_string(),
                format!("Query failed: {}", error_text),
            ))
        }
    }

    pub async fn get_sentiment_history(&self, symbol: &str, limit: Option<i32>) -> Result<Vec<SentimentRecord>> {
        let limit_str = limit.map(|l| l.to_string()).unwrap_or_else(|| "100".to_string());
        let url = format!(
            "{}/rest/v1/{}?symbol=eq.{}&order=analysis_date.desc&limit={}",
            self.config.supabase_url, self.config.table_name, symbol, limit_str
        );
        
        let response = self.client
            .get(&url)
            .header("apikey", &self.config.supabase_api_key)
            .header("Authorization", format!("Bearer {}", self.config.supabase_api_key))
            .send()
            .await?;

        if response.status().is_success() {
            let records: Vec<SentimentRecord> = response.json().await?;
            Ok(records)
        } else {
            let error_text = response.text().await?;
            Err(crate::error::Error::ApiError(
                "Supabase".to_string(),
                format!("Query failed: {}", error_text),
            ))
        }
    }

    pub async fn get_all_latest_sentiment(&self) -> Result<Vec<SentimentRecord>> {
        // Get latest sentiment for each symbol using a view or subquery
        let url = format!(
            "{}/rest/v1/{}?select=*&order=symbol,analysis_date.desc",
            self.config.supabase_url, self.config.table_name
        );
        
        let response = self.client
            .get(&url)
            .header("apikey", &self.config.supabase_api_key)
            .header("Authorization", format!("Bearer {}", self.config.supabase_api_key))
            .send()
            .await?;

        if response.status().is_success() {
            let all_records: Vec<SentimentRecord> = response.json().await?;
            
            // Group by symbol and get the latest for each
            let mut latest_by_symbol: HashMap<String, SentimentRecord> = HashMap::new();
            for record in all_records {
                let symbol = record.symbol.clone();
                if let Some(existing) = latest_by_symbol.get(&symbol) {
                    if record.analysis_date > existing.analysis_date {
                        latest_by_symbol.insert(symbol, record);
                    }
                } else {
                    latest_by_symbol.insert(symbol, record);
                }
            }
            
            Ok(latest_by_symbol.into_values().collect())
        } else {
            let error_text = response.text().await?;
            Err(crate::error::Error::ApiError(
                "Supabase".to_string(),
                format!("Query failed: {}", error_text),
            ))
        }
    }

    pub async fn test_connection(&self) -> Result<()> {
        let url = format!("{}/rest/v1/", self.config.supabase_url);
        
        let response = self.client
            .get(&url)
            .header("apikey", &self.config.supabase_api_key)
            .header("Authorization", format!("Bearer {}", self.config.supabase_api_key))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let error_text = response.text().await?;
            Err(crate::error::Error::ApiError(
                "Supabase".to_string(),
                format!("Connection test failed: {}", error_text),
            ))
        }
    }

    // Timing (TTS) database operations
    pub async fn insert_timing(&self, record: &CreateTimingRecord) -> Result<TimingRecord> {
        let url = format!("{}/rest/v1/timing", self.config.supabase_url);
        
        let response = self.client
            .post(&url)
            .header("apikey", &self.config.supabase_api_key)
            .header("Authorization", format!("Bearer {}", self.config.supabase_api_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation")
            .json(record)
            .send()
            .await?;

        if response.status().is_success() {
            let inserted_records: Vec<TimingRecord> = response.json().await?;
            if let Some(inserted) = inserted_records.first() {
                Ok(inserted.clone())
            } else {
                Err(crate::error::Error::ApiError(
                    "Supabase".to_string(),
                    "No timing record returned after insertion".to_string(),
                ))
            }
        } else {
            let error_text = response.text().await?;
            Err(crate::error::Error::ApiError(
                "Supabase".to_string(),
                format!("Timing insert failed: {}", error_text),
            ))
        }
    }

    pub async fn update_timing(&self, id: i32, update: &UpdateTimingRecord) -> Result<TimingRecord> {
        let url = format!("{}/rest/v1/timing?id=eq.{}", self.config.supabase_url, id);
        
        let response = self.client
            .patch(&url)
            .header("apikey", &self.config.supabase_api_key)
            .header("Authorization", format!("Bearer {}", self.config.supabase_api_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation")
            .json(update)
            .send()
            .await?;

        if response.status().is_success() {
            let updated_records: Vec<TimingRecord> = response.json().await?;
            if let Some(updated) = updated_records.first() {
                Ok(updated.clone())
            } else {
                Err(crate::error::Error::ApiError(
                    "Supabase".to_string(),
                    "No timing record returned after update".to_string(),
                ))
            }
        } else {
            let error_text = response.text().await?;
            Err(crate::error::Error::ApiError(
                "Supabase".to_string(),
                format!("Timing update failed: {}", error_text),
            ))
        }
    }

    pub async fn get_timing_records(&self, params: &TimingQueryParams) -> Result<Vec<TimingRecord>> {
        let mut query_params = Vec::new();
        
        if let Some(symbol) = &params.symbol {
            query_params.push(format!("symbol=eq.{}", symbol));
        }
        
        if let Some(limit) = params.limit {
            query_params.push(format!("limit={}", limit));
        }
        
        if let Some(offset) = params.offset {
            query_params.push(format!("offset={}", offset));
        }

        let query_string = if query_params.is_empty() {
            "".to_string()
        } else {
            format!("?{}", query_params.join("&"))
        };

        let url = format!("{}/rest/v1/timing{}", self.config.supabase_url, query_string);
        
        let response = self.client
            .get(&url)
            .header("apikey", &self.config.supabase_api_key)
            .header("Authorization", format!("Bearer {}", self.config.supabase_api_key))
            .header("Accept", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let timing_records: Vec<TimingRecord> = response.json().await?;
            Ok(timing_records)
        } else {
            let error_text = response.text().await?;
            Err(crate::error::Error::ApiError(
                "Supabase".to_string(),
                format!("Timing query failed: {}", error_text),
            ))
        }
    }

    pub async fn get_latest_timing(&self, symbol: &str) -> Result<Option<TimingRecord>> {
        let url = format!("{}/rest/v1/timing?symbol=eq.{}&order=created_at.desc&limit=1", 
                         self.config.supabase_url, symbol);
        
        let response = self.client
            .get(&url)
            .header("apikey", &self.config.supabase_api_key)
            .header("Authorization", format!("Bearer {}", self.config.supabase_api_key))
            .header("Accept", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let timing_records: Vec<TimingRecord> = response.json().await?;
            Ok(timing_records.first().cloned())
        } else {
            let error_text = response.text().await?;
            Err(crate::error::Error::ApiError(
                "Supabase".to_string(),
                format!("Latest timing query failed: {}", error_text),
            ))
        }
    }

    // Fundamentals database operations
    pub async fn insert_fundamentals(&self, record: &FundamentalsInsert) -> Result<FundamentalsRecord> {
        let url = format!("{}/rest/v1/fundamentals", self.config.supabase_url);
        
        let response = self.client
            .post(&url)
            .header("apikey", &self.config.supabase_api_key)
            .header("Authorization", format!("Bearer {}", self.config.supabase_api_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation")
            .json(record)
            .send()
            .await?;

        if response.status().is_success() {
            let inserted_records: Vec<FundamentalsRecord> = response.json().await?;
            if let Some(inserted) = inserted_records.first() {
                Ok(inserted.clone())
            } else {
                Err(crate::error::Error::ApiError(
                    "Supabase".to_string(),
                    "No fundamentals record returned after insertion".to_string(),
                ))
            }
        } else {
            let error_text = response.text().await?;
            Err(crate::error::Error::ApiError(
                "Supabase".to_string(),
                format!("Fundamentals insert failed: {}", error_text),
            ))
        }
    }

    pub async fn get_fundamentals_by_symbol(&self, symbol: &str, limit: Option<i32>) -> Result<Vec<FundamentalsRecord>> {
        let limit_str = limit.map(|l| l.to_string()).unwrap_or_else(|| "100".to_string());
        let url = format!(
            "{}/rest/v1/fundamentals?symbol=eq.{}&order=analysis_date.desc&limit={}",
            self.config.supabase_url, symbol, limit_str
        );
        
        let response = self.client
            .get(&url)
            .header("apikey", &self.config.supabase_api_key)
            .header("Authorization", format!("Bearer {}", self.config.supabase_api_key))
            .header("Accept", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let fundamentals_records: Vec<FundamentalsRecord> = response.json().await?;
            Ok(fundamentals_records)
        } else {
            let error_text = response.text().await?;
            Err(crate::error::Error::ApiError(
                "Supabase".to_string(),
                format!("Fundamentals query failed: {}", error_text),
            ))
        }
    }

    pub async fn get_all_latest_fundamentals(&self) -> Result<Vec<FundamentalsRecord>> {
        // Use the latest_fundamentals view we created
        let url = format!("{}/rest/v1/latest_fundamentals", self.config.supabase_url);
        
        let response = self.client
            .get(&url)
            .header("apikey", &self.config.supabase_api_key)
            .header("Authorization", format!("Bearer {}", self.config.supabase_api_key))
            .header("Accept", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let fundamentals_records: Vec<FundamentalsRecord> = response.json().await?;
            Ok(fundamentals_records)
        } else {
            let error_text = response.text().await?;
            Err(crate::error::Error::ApiError(
                "Supabase".to_string(),
                format!("Latest fundamentals query failed: {}", error_text),
            ))
        }
    }
}
