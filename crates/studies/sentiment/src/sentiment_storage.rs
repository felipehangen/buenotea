// Sentiment data storage operations for Supabase
// Uses time-series approach to track sentiment (QSS) over time

use buenotea_core::Result;
use buenotea_infrastructure::DatabaseClient;
use buenotea_infrastructure::sentiment_models::{CreateSentimentRecord, SentimentInsert, SentimentRecord};
use tracing::info;

/// Sentiment data storage operations
/// Stores historical time-series data in sentiment_history table
pub struct SentimentStorage {
    db_client: DatabaseClient,
}

impl SentimentStorage {
    /// Create a new sentiment storage instance
    pub fn new(db_client: DatabaseClient) -> Self {
        Self { db_client }
    }

    /// Create from environment variables
    pub fn from_env() -> Result<Self> {
        let db_client = DatabaseClient::from_env()?;
        Ok(Self::new(db_client))
    }

    /// Store a sentiment record in the database (time-series history)
    /// Each analysis is stored as a new record to track sentiment changes over time
    pub async fn store_sentiment_record(&self, record: &CreateSentimentRecord) -> Result<i64> {
        info!("Storing sentiment record for symbol: {} on date: {}", 
              record.symbol, record.analysis_date);

        let url = format!("{}/rest/v1/sentiment_history", self.db_client.config().supabase_url);

        let insert_record: SentimentInsert = record.clone().into();

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
                "Failed to store sentiment record: {} - {}",
                status,
                error_text
            )));
        }

        // Parse the response to get the ID
        let response_data: serde_json::Value = response.json().await?;
        let id = response_data[0]["id"]
            .as_i64()
            .ok_or_else(|| buenotea_core::Error::DatabaseError("No ID returned from database".to_string()))?;

        info!("Successfully stored sentiment record for {} with ID: {}", record.symbol, id);
        Ok(id)
    }

    /// Store multiple sentiment records in batch (time-series history)
    /// Each analysis is stored as a new record
    pub async fn store_multiple_records(&self, records: &[CreateSentimentRecord]) -> Result<Vec<i64>> {
        info!("Storing {} sentiment records", records.len());

        let url = format!("{}/rest/v1/sentiment_history", self.db_client.config().supabase_url);

        let insert_records: Vec<SentimentInsert> = records
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
                "Failed to store sentiment records: {} - {}",
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

        info!("Successfully stored {} sentiment records", ids.len());
        Ok(ids)
    }

    /// Get the latest sentiment record for a specific symbol (from sentiment view)
    pub async fn get_latest_sentiment(&self, symbol: &str) -> Result<Option<SentimentRecord>> {
        info!("Getting latest sentiment record for {}", symbol);

        let url = format!(
            "{}/rest/v1/sentiment?symbol=eq.{}",
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
                "Failed to get latest sentiment record: {} - {}",
                status,
                error_text
            )));
        }

        let records: Vec<SentimentRecord> = response.json().await?;
        Ok(records.into_iter().next())
    }

    /// Get all latest sentiment records (from sentiment view)
    pub async fn get_all_latest_sentiment(&self) -> Result<Vec<SentimentRecord>> {
        info!("Getting all latest sentiment records");

        let url = format!(
            "{}/rest/v1/sentiment?select=*",
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
                "Failed to get all latest sentiment records: {} - {}",
                status,
                error_text
            )));
        }

        let records: Vec<SentimentRecord> = response.json().await?;
        info!("Retrieved {} latest sentiment records", records.len());
        Ok(records)
    }

    /// Get sentiment history for a specific symbol over the past N days
    pub async fn get_sentiment_history(&self, symbol: &str, days: i32) -> Result<Vec<SentimentRecord>> {
        info!("Getting sentiment history for {} ({} days)", symbol, days);

        let url = format!(
            "{}/rest/v1/rpc/get_sentiment_history",
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
                "Failed to get sentiment history: {} - {}",
                status,
                error_text
            )));
        }

        let records: Vec<SentimentRecord> = response.json().await?;
        info!("Retrieved {} sentiment records for {}", records.len(), symbol);
        Ok(records)
    }

    /// Get sentiment score changes for a specific symbol in the past N days
    pub async fn get_sentiment_changes(&self, symbol: &str, days: i32) -> Result<Vec<serde_json::Value>> {
        info!("Getting sentiment changes for {} ({} days)", symbol, days);

        let url = format!(
            "{}/rest/v1/rpc/get_sentiment_changes",
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
                "Failed to get sentiment changes: {} - {}",
                status,
                error_text
            )));
        }

        let changes: Vec<serde_json::Value> = response.json().await?;
        info!("Retrieved {} sentiment changes for {}", changes.len(), symbol);
        Ok(changes)
    }

    /// Test the database connection
    pub async fn test_connection(&self) -> Result<()> {
        // Test by querying the sentiment view
        let url = format!("{}/rest/v1/sentiment?limit=1", self.db_client.config().supabase_url);
        
        let response = self.db_client
            .http_client()
            .get(&url)
            .header("apikey", &self.db_client.config().supabase_api_key)
            .header("Authorization", format!("Bearer {}", self.db_client.config().supabase_api_key))
            .send()
            .await?;

        if response.status().is_success() {
            info!("✅ Database connection test successful");
            Ok(())
        } else {
            Err(buenotea_core::Error::DatabaseError(
                format!("Database connection test failed: {}", response.status())
            ))
        }
    }
}
