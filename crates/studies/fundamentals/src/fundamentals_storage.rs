// Fundamentals data storage operations for Supabase
// Uses time-series approach to track fundamentals over time

use buenotea_core::Result;
use buenotea_infrastructure::DatabaseClient;
use buenotea_infrastructure::fundamentals_models::{CreateFundamentalsRecord, FundamentalsInsert, FundamentalsRecord};
use tracing::info;

/// Fundamentals data storage operations
/// Stores historical time-series data in fundamentals_history table
pub struct FundamentalsStorage {
    db_client: DatabaseClient,
}

impl FundamentalsStorage {
    /// Create a new fundamentals storage instance
    pub fn new(db_client: DatabaseClient) -> Self {
        Self { db_client }
    }

    /// Create from environment variables
    pub fn from_env() -> Result<Self> {
        let db_client = DatabaseClient::from_env()?;
        Ok(Self::new(db_client))
    }

    /// Store a fundamentals record in the database (time-series history)
    /// Each analysis is stored as a new record to track fundamentals changes over time
    pub async fn store_fundamentals_record(&self, record: &CreateFundamentalsRecord) -> Result<i64> {
        info!("Storing fundamentals record for symbol: {} on date: {}", 
              record.symbol, record.analysis_date);

        let url = format!("{}/rest/v1/fundamentals_history", self.db_client.config().supabase_url);

        let insert_record: FundamentalsInsert = record.clone().into();

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
                "Failed to store fundamentals record: {} - {}",
                status,
                error_text
            )));
        }

        // Parse the response to get the ID
        let response_data: serde_json::Value = response.json().await?;
        let id = response_data[0]["id"]
            .as_i64()
            .ok_or_else(|| buenotea_core::Error::DatabaseError("No ID returned from database".to_string()))?;

        info!("Successfully stored fundamentals record for {} with ID: {}", record.symbol, id);
        Ok(id)
    }

    /// Store multiple fundamentals records in batch (time-series history)
    /// Each analysis is stored as a new record
    pub async fn store_multiple_records(&self, records: &[CreateFundamentalsRecord]) -> Result<Vec<i64>> {
        info!("Storing {} fundamentals records", records.len());

        let url = format!("{}/rest/v1/fundamentals_history", self.db_client.config().supabase_url);

        let insert_records: Vec<FundamentalsInsert> = records
            .iter()
            .map(|r| r.clone().into())
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
                "Failed to store fundamentals records: {} - {}",
                status,
                error_text
            )));
        }

        // Parse the response to get the IDs
        let response_data: Vec<serde_json::Value> = response.json().await?;
        let ids: Vec<i64> = response_data
            .iter()
            .filter_map(|item| item["id"].as_i64())
            .collect();

        info!("Successfully stored {} fundamentals records", ids.len());
        Ok(ids)
    }

    /// Get the latest fundamentals record for a specific symbol (from fundamentals view)
    pub async fn get_latest_fundamentals(&self, symbol: &str) -> Result<Option<FundamentalsRecord>> {
        info!("Getting latest fundamentals record for {}", symbol);

        let url = format!(
            "{}/rest/v1/fundamentals?symbol=eq.{}",
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
                "Failed to get latest fundamentals record: {} - {}",
                status,
                error_text
            )));
        }

        let records: Vec<FundamentalsRecord> = response.json().await?;
        Ok(records.into_iter().next())
    }

    /// Get all latest fundamentals records (from fundamentals view)
    pub async fn get_all_latest_fundamentals(&self) -> Result<Vec<FundamentalsRecord>> {
        info!("Getting all latest fundamentals records");

        let url = format!(
            "{}/rest/v1/fundamentals?select=*",
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
                "Failed to get all latest fundamentals records: {} - {}",
                status,
                error_text
            )));
        }

        let records: Vec<FundamentalsRecord> = response.json().await?;
        info!("Retrieved {} latest fundamentals records", records.len());
        Ok(records)
    }

    /// Get fundamentals history for a specific symbol over the past N days
    pub async fn get_fundamentals_history(&self, symbol: &str, days: i32) -> Result<Vec<FundamentalsRecord>> {
        info!("Getting fundamentals history for {} ({} days)", symbol, days);

        let url = format!(
            "{}/rest/v1/rpc/get_fundamentals_history",
            self.db_client.config().supabase_url
        );

        let body = serde_json::json!({
            "target_symbol": symbol,
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
                "Failed to get fundamentals history: {} - {}",
                status,
                error_text
            )));
        }

        let records: Vec<FundamentalsRecord> = response.json().await?;
        info!("Retrieved {} fundamentals records for {}", records.len(), symbol);
        Ok(records)
    }

    /// Get fundamentals score changes for a specific symbol in the past N days
    pub async fn get_fundamentals_changes(&self, symbol: &str, days: i32) -> Result<Vec<serde_json::Value>> {
        info!("Getting fundamentals changes for {} ({} days)", symbol, days);

        let url = format!(
            "{}/rest/v1/rpc/get_fundamentals_changes",
            self.db_client.config().supabase_url
        );

        let body = serde_json::json!({
            "target_symbol": symbol,
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
                "Failed to get fundamentals changes: {} - {}",
                status,
                error_text
            )));
        }

        let changes: Vec<serde_json::Value> = response.json().await?;
        info!("Retrieved {} fundamentals changes for {}", changes.len(), symbol);
        Ok(changes)
    }
}
