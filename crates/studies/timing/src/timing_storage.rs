// Timing (TTS) data storage operations for Supabase
// Uses time-series approach to track timing signals over time

use buenotea_core::Result;
use buenotea_infrastructure::DatabaseClient;
use buenotea_infrastructure::timing_models::{CreateTimingRecord, TimingInsert, TimingRecord};
use tracing::info;

/// Timing data storage operations
/// Stores historical time-series data in timing_history table
pub struct TimingStorage {
    db_client: DatabaseClient,
}

impl TimingStorage {
    /// Create a new timing storage instance
    pub fn new(db_client: DatabaseClient) -> Self {
        Self { db_client }
    }

    /// Create from environment variables
    pub fn from_env() -> Result<Self> {
        let db_client = DatabaseClient::from_env()?;
        Ok(Self::new(db_client))
    }

    /// Store a timing record in the database (time-series history)
    /// Each analysis is stored as a new record to track signal changes over time
    pub async fn store_timing_record(&self, record: &CreateTimingRecord) -> Result<i64> {
        info!("Storing timing record for symbol: {}", record.symbol);
        
        let url = format!("{}/rest/v1/timing_history", self.db_client.config().supabase_url);
        
        let insert_record: TimingInsert = record.clone().into();

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
                "Failed to store timing record: {} - {}",
                status,
                error_text
            )));
        }

        // Parse the response to get the ID
        let response_data: serde_json::Value = response.json().await?;
        let id = response_data[0]["id"]
            .as_i64()
            .ok_or_else(|| buenotea_core::Error::DatabaseError("No ID returned from database".to_string()))?;

        info!("Successfully stored timing record with ID: {}", id);
        Ok(id)
    }

    /// Store multiple timing records in batch (time-series history)
    pub async fn store_multiple_records(&self, records: &[CreateTimingRecord]) -> Result<Vec<i64>> {
        info!("Storing {} timing records in batch", records.len());
        
        let url = format!("{}/rest/v1/timing_history", self.db_client.config().supabase_url);
        
        let insert_records: Vec<TimingInsert> = records
            .iter()
            .map(|record| record.clone().into())
            .collect();

        let response = self.db_client
            .http_client()
            .post(&url)
            .header("apikey", &self.db_client.config().supabase_api_key)
            .header("Authorization", format!("Bearer {}", self.db_client.config().supabase_api_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation")
            .json(&insert_records)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(buenotea_core::Error::DatabaseError(format!(
                "Failed to store timing records: {} - {}",
                status,
                error_text
            )));
        }

        // Parse the response to get the IDs
        let response_data: serde_json::Value = response.json().await?;
        let ids: Vec<i64> = response_data
            .as_array()
            .ok_or_else(|| buenotea_core::Error::DatabaseError("Expected array response".to_string()))?
            .iter()
            .filter_map(|record| record["id"].as_i64())
            .collect();

        info!("Successfully stored {} timing records", ids.len());
        Ok(ids)
    }

    /// Get the latest timing record for a symbol (from timing view)
    pub async fn get_latest_timing_record(&self, symbol: &str) -> Result<Option<TimingRecord>> {
        info!("Getting latest timing record for {}", symbol);
        
        let url = format!(
            "{}/rest/v1/timing?symbol=eq.{}&select=*",
            self.db_client.config().supabase_url,
            symbol
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
                "Failed to get timing record: {} - {}",
                status,
                error_text
            )));
        }

        let records: Vec<TimingRecord> = response.json().await?;
        Ok(records.into_iter().next())
    }

    /// Get timing history for a symbol over the past N days
    pub async fn get_timing_history(&self, symbol: &str, days: i32) -> Result<Vec<TimingRecord>> {
        info!("Getting timing history for {} ({} days)", symbol, days);
        
        let url = format!(
            "{}/rest/v1/rpc/get_timing_history",
            self.db_client.config().supabase_url
        );

        let body = serde_json::json!({
            "stock_symbol": symbol,
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
                "Failed to get timing history: {} - {}",
                status,
                error_text
            )));
        }

        let records: Vec<TimingRecord> = response.json().await?;
        info!("Retrieved {} timing records", records.len());
        Ok(records)
    }

    /// Get all timing signal changes in the past N days
    pub async fn get_signal_changes(&self, days: i32) -> Result<Vec<serde_json::Value>> {
        info!("Getting timing signal changes ({} days)", days);
        
        let url = format!(
            "{}/rest/v1/rpc/get_timing_signal_changes",
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
                "Failed to get signal changes: {} - {}",
                status,
                error_text
            )));
        }

        let changes: Vec<serde_json::Value> = response.json().await?;
        info!("Retrieved {} signal changes", changes.len());
        Ok(changes)
    }

    /// Get all stocks with a specific trading signal
    pub async fn get_stocks_by_signal(&self, signal: &str) -> Result<Vec<serde_json::Value>> {
        info!("Getting stocks with {} signal", signal);
        
        let url = format!(
            "{}/rest/v1/rpc/get_stocks_by_signal",
            self.db_client.config().supabase_url
        );

        let body = serde_json::json!({
            "signal_type": signal
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
                "Failed to get stocks by signal: {} - {}",
                status,
                error_text
            )));
        }

        let stocks: Vec<serde_json::Value> = response.json().await?;
        info!("Retrieved {} stocks", stocks.len());
        Ok(stocks)
    }
}
