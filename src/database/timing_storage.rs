// Timing (TTS) data storage operations for Supabase

use crate::error::Result;
use crate::database::{DatabaseClient, timing_models::*};
use tracing::info;

/// Timing data storage operations
pub struct TimingStorage {
    client: DatabaseClient,
}

impl TimingStorage {
    /// Create a new timing storage instance
    pub fn new(client: DatabaseClient) -> Self {
        Self { client }
    }

    /// Create from environment variables
    pub fn from_env() -> Result<Self> {
        let client = DatabaseClient::from_env()?;
        Ok(Self::new(client))
    }

    /// Save a timing record to the database using REST API
    pub async fn save_timing_record(&self, record: CreateTimingRecord) -> Result<TimingRecord> {
        info!("Saving timing record for {}", record.symbol);
        self.client.insert_timing(&record).await
    }

    /// Update a timing record with AI explanations using REST API
    pub async fn update_timing_record(&self, id: i32, update: UpdateTimingRecord) -> Result<TimingRecord> {
        info!("Updating timing record with ID: {}", id);
        self.client.update_timing(id, &update).await
    }

    /// Get timing records with optional filtering (simplified for REST API)
    pub async fn get_timing_records(&self, params: TimingQueryParams) -> Result<Vec<TimingRecord>> {
        info!("Querying timing records with params: {:?}", params);
        let records = self.client.get_timing_records(&params).await?;
        info!("Retrieved {} timing records", records.len());
        Ok(records)
    }

    /// Get the latest timing record for a symbol
    pub async fn get_latest_timing_record(&self, symbol: &str) -> Result<Option<TimingRecord>> {
        info!("Getting latest timing record for {}", symbol);
        self.client.get_latest_timing(symbol).await
    }
}
