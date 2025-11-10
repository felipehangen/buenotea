// Market Regime data storage operations for Supabase
// Uses time-series approach to track market regime changes over time

use buenotea_core::Result;
use buenotea_infrastructure::DatabaseClient;
use buenotea_infrastructure::market_regime_models::{CreateMarketRegimeRecord, MarketRegimeInsert, MarketRegimeRecord};
use tracing::info;

/// Market Regime data storage operations
/// Stores historical time-series data in market_regime_history table
pub struct MarketRegimeStorage {
    db_client: DatabaseClient,
}

impl MarketRegimeStorage {
    /// Create a new market regime storage instance
    pub fn new(db_client: DatabaseClient) -> Self {
        Self { db_client }
    }

    /// Create from environment variables
    pub fn from_env() -> Result<Self> {
        let db_client = DatabaseClient::from_env()?;
        Ok(Self::new(db_client))
    }

    /// Store a market regime record in the database (time-series history)
    /// Each analysis is stored as a new record to track regime changes over time
    pub async fn store_market_regime_record(&self, record: &CreateMarketRegimeRecord) -> Result<i64> {
        info!("Storing market regime record for date: {}", record.analysis_date);
        
        let url = format!("{}/rest/v1/market_regime_history", self.db_client.config().supabase_url);
        
        let insert_record: MarketRegimeInsert = record.clone().into();
        
        // Debug: log the insert record structure
        info!("Insert record fields: analysis_date={}, market_regime={}, advancing_stocks={:?}", 
              insert_record.analysis_date, 
              insert_record.market_regime,
              insert_record.advancing_stocks);

        let response = self.db_client
            .http_client()
            .post(&url)
            .header("apikey", &self.db_client.config().supabase_api_key)
            .header("Authorization", format!("Bearer {}", self.db_client.config().supabase_api_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation")
            .json(&insert_record)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(buenotea_core::Error::DatabaseError(format!(
                "Failed to store market regime record: {} - {}",
                status,
                error_text
            )));
        }

        // Parse the response to get the ID
        let response_data: serde_json::Value = response.json().await?;
        let id = response_data[0]["id"]
            .as_i64()
            .ok_or_else(|| buenotea_core::Error::DatabaseError("No ID returned from database".to_string()))?;

        info!("Successfully stored market regime record with ID: {}", id);
        Ok(id)
    }

    /// Get the latest market regime record (from market_regime view)
    pub async fn get_latest_market_regime(&self) -> Result<Option<MarketRegimeRecord>> {
        info!("Getting latest market regime record");
        
        let url = format!(
            "{}/rest/v1/market_regime?select=*&limit=1",
            self.db_client.config().supabase_url
        );

        let response = self.db_client
            .http_client()
            .get(&url)
            .header("apikey", &self.db_client.config().supabase_api_key)
            .header("Authorization", format!("Bearer {}", self.db_client.config().supabase_api_key))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(buenotea_core::Error::DatabaseError(format!(
                "Failed to get market regime record: {} - {}",
                status,
                error_text
            )));
        }

        let records: Vec<MarketRegimeRecord> = response.json().await?;
        Ok(records.into_iter().next())
    }

    /// Get market regime history over the past N days
    pub async fn get_market_regime_history(&self, days: i32) -> Result<Vec<MarketRegimeRecord>> {
        info!("Getting market regime history ({} days)", days);
        
        let url = format!(
            "{}/rest/v1/rpc/get_market_regime_history",
            self.db_client.config().supabase_url
        );

        let body = serde_json::json!({
            "days_back": days
        });

        let response = self.db_client
            .http_client()
            .post(&url)
            .header("apikey", &self.db_client.config().supabase_api_key)
            .header("Authorization", format!("Bearer {}", self.db_client.config().supabase_api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(buenotea_core::Error::DatabaseError(format!(
                "Failed to get market regime history: {} - {}",
                status,
                error_text
            )));
        }

        let records: Vec<MarketRegimeRecord> = response.json().await?;
        info!("Retrieved {} market regime records", records.len());
        Ok(records)
    }

    /// Get market regime changes in the past N days
    pub async fn get_regime_changes(&self, days: i32) -> Result<Vec<serde_json::Value>> {
        info!("Getting market regime changes ({} days)", days);
        
        let url = format!(
            "{}/rest/v1/rpc/get_market_regime_changes",
            self.db_client.config().supabase_url
        );

        let body = serde_json::json!({
            "days_back": days
        });

        let response = self.db_client
            .http_client()
            .post(&url)
            .header("apikey", &self.db_client.config().supabase_api_key)
            .header("Authorization", format!("Bearer {}", self.db_client.config().supabase_api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(buenotea_core::Error::DatabaseError(format!(
                "Failed to get regime changes: {} - {}",
                status,
                error_text
            )));
        }

        let changes: Vec<serde_json::Value> = response.json().await?;
        info!("Retrieved {} regime changes", changes.len());
        Ok(changes)
    }
}

