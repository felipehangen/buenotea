use super::models::DatabaseConfig;
use buenotea_core::Result;
use reqwest::Client;
use serde::{Serialize, de::DeserializeOwned};

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

    /// Generic insert method
    pub async fn insert<T: Serialize, R: DeserializeOwned>(&self, table: &str, record: &T) -> Result<R> {
        let url = format!("{}/rest/v1/{}", self.config.supabase_url, table);
        
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
            let records: Vec<R> = response.json().await?;
            records.into_iter().next().ok_or_else(|| {
                buenotea_core::Error::DatabaseError("No record returned after insert".to_string())
            })
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(buenotea_core::Error::DatabaseError(format!("Insert failed: {}", error_text)))
        }
    }

    /// Generic upsert method
    pub async fn upsert<T: Serialize, R: DeserializeOwned>(&self, table: &str, record: &T) -> Result<R> {
        let url = format!("{}/rest/v1/{}", self.config.supabase_url, table);
        
        let response = self.client
            .post(&url)
            .header("apikey", &self.config.supabase_api_key)
            .header("Authorization", format!("Bearer {}", self.config.supabase_api_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation,resolution=merge-duplicates")
            .json(record)
            .send()
            .await?;

        if response.status().is_success() {
            let records: Vec<R> = response.json().await?;
            records.into_iter().next().ok_or_else(|| {
                buenotea_core::Error::DatabaseError("No record returned after upsert".to_string())
            })
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(buenotea_core::Error::DatabaseError(format!("Upsert failed: {}", error_text)))
        }
    }

    /// Generic query method
    pub async fn query<R: DeserializeOwned>(&self, table: &str, filter: Option<&str>) -> Result<Vec<R>> {
        let mut url = format!("{}/rest/v1/{}", self.config.supabase_url, table);
        if let Some(filter_str) = filter {
            url.push_str(&format!("?{}", filter_str));
        }
        
        let response = self.client
            .get(&url)
            .header("apikey", &self.config.supabase_api_key)
            .header("Authorization", format!("Bearer {}", self.config.supabase_api_key))
            .send()
            .await?;

        if response.status().is_success() {
            let records: Vec<R> = response.json().await?;
            Ok(records)
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(buenotea_core::Error::DatabaseError(format!("Query failed: {}", error_text)))
        }
    }

    /// Generic update method
    pub async fn update<T: Serialize, R: DeserializeOwned>(&self, table: &str, filter: &str, update: &T) -> Result<Vec<R>> {
        let url = format!("{}/rest/v1/{}?{}", self.config.supabase_url, table, filter);
        
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
            let records: Vec<R> = response.json().await?;
            Ok(records)
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(buenotea_core::Error::DatabaseError(format!("Update failed: {}", error_text)))
        }
    }

    /// Generic delete method
    pub async fn delete(&self, table: &str, filter: &str) -> Result<()> {
        let url = format!("{}/rest/v1/{}?{}", self.config.supabase_url, table, filter);
        
        let response = self.client
            .delete(&url)
            .header("apikey", &self.config.supabase_api_key)
            .header("Authorization", format!("Bearer {}", self.config.supabase_api_key))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(buenotea_core::Error::DatabaseError(format!("Delete failed: {}", error_text)))
        }
    }
}
