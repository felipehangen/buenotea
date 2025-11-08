// Database storage functions for invite list analysis

use crate::error::Result;
use crate::invite_list::models::{InviteListRecord, InviteListInsert};
use crate::database::DatabaseClient;
use tracing::info;

/// Storage operations for invite list data
pub struct InviteListStorage {
    db_client: DatabaseClient,
}

impl InviteListStorage {
    /// Create a new invite list storage instance
    pub fn new(db_client: DatabaseClient) -> Self {
        Self { db_client }
    }

    /// Create from environment variables
    pub fn from_env() -> Result<Self> {
        let db_client = DatabaseClient::from_env()?;
        Ok(Self::new(db_client))
    }

    /// Store an invite list record in the database
    pub async fn store_invite_list_record(&self, record: &InviteListRecord) -> Result<i64> {
        info!("Storing invite list record for symbol: {}", record.symbol);
        
        let url = format!("{}/rest/v1/invite_list", self.db_client.config().supabase_url);
        
        let insert_record = InviteListInsert {
            symbol: record.symbol.clone(),
            company_name: record.company_name.clone(),
            sector: record.sector.clone(),
            industry: record.industry.clone(),
            market_cap: record.market_cap,
            current_price: record.current_price,
            is_safe_to_trade: record.is_safe_to_trade,
            safety_score: record.safety_score,
            safety_reasoning: record.safety_reasoning.clone(),
            has_recent_earnings: record.has_recent_earnings,
            has_positive_revenue: record.has_positive_revenue,
            has_stable_price: record.has_stable_price,
            has_sufficient_volume: record.has_sufficient_volume,
            has_analyst_coverage: record.has_analyst_coverage,
            risk_level: record.risk_level.clone(),
            volatility_rating: record.volatility_rating.clone(),
            liquidity_rating: record.liquidity_rating.clone(),
            data_source: record.data_source.clone(),
            last_updated: record.last_updated,
            data_freshness_score: record.data_freshness_score,
            analysis_date: record.analysis_date,
            analysis_duration_ms: record.analysis_duration_ms,
            warning_flags: record.warning_flags.clone(),
            missing_data_components: record.missing_data_components.clone(),
            raw_company_data: record.raw_company_data.clone(),
            raw_financial_data: record.raw_financial_data.clone(),
            raw_price_data: record.raw_price_data.clone(),
        };

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
            return Err(crate::error::Error::DatabaseError(format!(
                "Failed to store invite list record: {} - {}",
                status,
                error_text
            )));
        }

        // Parse the response to get the ID
        let response_data: serde_json::Value = response.json().await?;
        let id = response_data[0]["id"]
            .as_i64()
            .ok_or_else(|| crate::error::Error::DatabaseError("No ID returned from database".to_string()))?;

        info!("Successfully stored invite list record with ID: {}", id);
        Ok(id)
    }

    /// Store multiple invite list records in batch
    pub async fn store_multiple_records(&self, records: &[InviteListRecord]) -> Result<Vec<i64>> {
        info!("Storing {} invite list records in batch", records.len());
        
        let url = format!("{}/rest/v1/invite_list", self.db_client.config().supabase_url);
        
        let insert_records: Vec<InviteListInsert> = records
            .iter()
            .map(|record| InviteListInsert {
                symbol: record.symbol.clone(),
                company_name: record.company_name.clone(),
                sector: record.sector.clone(),
                industry: record.industry.clone(),
                market_cap: record.market_cap,
                current_price: record.current_price,
                is_safe_to_trade: record.is_safe_to_trade,
                safety_score: record.safety_score,
                safety_reasoning: record.safety_reasoning.clone(),
                has_recent_earnings: record.has_recent_earnings,
                has_positive_revenue: record.has_positive_revenue,
                has_stable_price: record.has_stable_price,
                has_sufficient_volume: record.has_sufficient_volume,
                has_analyst_coverage: record.has_analyst_coverage,
                risk_level: record.risk_level.clone(),
                volatility_rating: record.volatility_rating.clone(),
                liquidity_rating: record.liquidity_rating.clone(),
                data_source: record.data_source.clone(),
                last_updated: record.last_updated,
                data_freshness_score: record.data_freshness_score,
                analysis_date: record.analysis_date,
                analysis_duration_ms: record.analysis_duration_ms,
                warning_flags: record.warning_flags.clone(),
                missing_data_components: record.missing_data_components.clone(),
                raw_company_data: record.raw_company_data.clone(),
                raw_financial_data: record.raw_financial_data.clone(),
                raw_price_data: record.raw_price_data.clone(),
            })
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
            return Err(crate::error::Error::DatabaseError(format!(
                "Failed to store invite list records: {} - {}",
                status,
                error_text
            )));
        }

        // Parse the response to get the IDs
        let response_data: Vec<serde_json::Value> = response.json().await?;
        let ids: Result<Vec<i64>> = response_data
            .iter()
            .map(|item| {
                item["id"]
                    .as_i64()
                    .ok_or_else(|| crate::error::Error::DatabaseError("No ID returned from database".to_string()))
            })
            .collect();

        let ids = ids?;
        info!("Successfully stored {} invite list records", ids.len());
        Ok(ids)
    }

    /// Get all safe stocks from the database
    pub async fn get_safe_stocks(&self) -> Result<Vec<InviteListRecord>> {
        info!("Fetching safe stocks from database");
        
        let url = format!(
            "{}/rest/v1/invite_list?is_safe_to_trade=eq.true&select=*&order=safety_score.desc",
            self.db_client.config().supabase_url
        );

        let response = self.db_client
            .http_client()
            .get(&url)
            .header("apikey", &self.db_client.config().supabase_api_key)
            .header("Authorization", format!("Bearer {}", self.db_client.config().supabase_api_key))
            .header("Accept", "application/json")
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(crate::error::Error::DatabaseError(format!(
                "Failed to fetch safe stocks: {} - {}",
                status,
                error_text
            )));
        }

        let records: Vec<InviteListRecord> = response.json().await?;
        info!("Fetched {} safe stocks from database", records.len());
        Ok(records)
    }

    /// Get stocks by sector
    pub async fn get_stocks_by_sector(&self, sector: &str) -> Result<Vec<InviteListRecord>> {
        info!("Fetching stocks for sector: {}", sector);
        
        let url = format!(
            "{}/rest/v1/invite_list?sector=eq.{}&select=*&order=safety_score.desc",
            self.db_client.config().supabase_url,
            urlencoding::encode(sector)
        );

        let response = self.db_client
            .http_client()
            .get(&url)
            .header("apikey", &self.db_client.config().supabase_api_key)
            .header("Authorization", format!("Bearer {}", self.db_client.config().supabase_api_key))
            .header("Accept", "application/json")
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(crate::error::Error::DatabaseError(format!(
                "Failed to fetch stocks for sector {}: {} - {}",
                sector, status, error_text
            )));
        }

        let records: Vec<InviteListRecord> = response.json().await?;
        info!("Fetched {} stocks for sector {}", records.len(), sector);
        Ok(records)
    }

    /// Get stocks by risk level
    pub async fn get_stocks_by_risk_level(&self, risk_level: &str) -> Result<Vec<InviteListRecord>> {
        info!("Fetching stocks for risk level: {}", risk_level);
        
        let url = format!(
            "{}/rest/v1/invite_list?risk_level=eq.{}&select=*&order=safety_score.desc",
            self.db_client.config().supabase_url,
            urlencoding::encode(risk_level)
        );

        let response = self.db_client
            .http_client()
            .get(&url)
            .header("apikey", &self.db_client.config().supabase_api_key)
            .header("Authorization", format!("Bearer {}", self.db_client.config().supabase_api_key))
            .header("Accept", "application/json")
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(crate::error::Error::DatabaseError(format!(
                "Failed to fetch stocks for risk level {}: {} - {}",
                risk_level, status, error_text
            )));
        }

        let records: Vec<InviteListRecord> = response.json().await?;
        info!("Fetched {} stocks for risk level {}", records.len(), risk_level);
        Ok(records)
    }

    /// Get a specific stock by symbol
    pub async fn get_stock_by_symbol(&self, symbol: &str) -> Result<Option<InviteListRecord>> {
        info!("Fetching stock data for symbol: {}", symbol);
        
        let url = format!(
            "{}/rest/v1/invite_list?symbol=eq.{}&select=*",
            self.db_client.config().supabase_url,
            urlencoding::encode(symbol)
        );

        let response = self.db_client
            .http_client()
            .get(&url)
            .header("apikey", &self.db_client.config().supabase_api_key)
            .header("Authorization", format!("Bearer {}", self.db_client.config().supabase_api_key))
            .header("Accept", "application/json")
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(crate::error::Error::DatabaseError(format!(
                "Failed to fetch stock {}: {} - {}",
                symbol, status, error_text
            )));
        }

        let records: Vec<InviteListRecord> = response.json().await?;
        Ok(records.into_iter().next())
    }

    /// Update an existing invite list record
    pub async fn update_invite_list_record(&self, record: &InviteListRecord) -> Result<()> {
        if let Some(id) = record.id {
            info!("Updating invite list record with ID: {}", id);
            
            let url = format!(
                "{}/rest/v1/invite_list?id=eq.{}",
                self.db_client.config().supabase_url,
                id
            );

            let update_data = InviteListInsert {
                symbol: record.symbol.clone(),
                company_name: record.company_name.clone(),
                sector: record.sector.clone(),
                industry: record.industry.clone(),
                market_cap: record.market_cap,
                current_price: record.current_price,
                is_safe_to_trade: record.is_safe_to_trade,
                safety_score: record.safety_score,
                safety_reasoning: record.safety_reasoning.clone(),
                has_recent_earnings: record.has_recent_earnings,
                has_positive_revenue: record.has_positive_revenue,
                has_stable_price: record.has_stable_price,
                has_sufficient_volume: record.has_sufficient_volume,
                has_analyst_coverage: record.has_analyst_coverage,
                risk_level: record.risk_level.clone(),
                volatility_rating: record.volatility_rating.clone(),
                liquidity_rating: record.liquidity_rating.clone(),
                data_source: record.data_source.clone(),
                last_updated: record.last_updated,
                data_freshness_score: record.data_freshness_score,
                analysis_date: record.analysis_date,
                analysis_duration_ms: record.analysis_duration_ms,
                warning_flags: record.warning_flags.clone(),
                missing_data_components: record.missing_data_components.clone(),
                raw_company_data: record.raw_company_data.clone(),
                raw_financial_data: record.raw_financial_data.clone(),
                raw_price_data: record.raw_price_data.clone(),
            };

            let response = self.db_client
                .http_client()
                .patch(&url)
                .header("apikey", &self.db_client.config().supabase_api_key)
                .header("Authorization", format!("Bearer {}", self.db_client.config().supabase_api_key))
                .header("Content-Type", "application/json")
                .header("Prefer", "return=representation")
                .json(&update_data)
                .send()
                .await?;

            if !response.status().is_success() {
                let status = response.status();
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                return Err(crate::error::Error::DatabaseError(format!(
                    "Failed to update invite list record: {} - {}",
                    status,
                    error_text
                )));
            }

            info!("Successfully updated invite list record with ID: {}", id);
            Ok(())
        } else {
            Err(crate::error::Error::DatabaseError("Cannot update record without ID".to_string()))
        }
    }

    /// Delete an invite list record
    pub async fn delete_invite_list_record(&self, id: i64) -> Result<()> {
        info!("Deleting invite list record with ID: {}", id);
        
        let url = format!(
            "{}/rest/v1/invite_list?id=eq.{}",
            self.db_client.config().supabase_url,
            id
        );

        let response = self.db_client
            .http_client()
            .delete(&url)
            .header("apikey", &self.db_client.config().supabase_api_key)
            .header("Authorization", format!("Bearer {}", self.db_client.config().supabase_api_key))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(crate::error::Error::DatabaseError(format!(
                "Failed to delete invite list record: {} - {}",
                status,
                error_text
            )));
        }

        info!("Successfully deleted invite list record with ID: {}", id);
        Ok(())
    }

    /// Get all stocks from the invite_list table with pagination
    pub async fn get_all_stocks(&self) -> Result<Vec<InviteListRecord>> {
        info!("Fetching all stocks from invite_list table");
        
        let url = format!(
            "{}/rest/v1/invite_list?select=*&order=symbol&limit=1000",
            self.db_client.config().supabase_url
        );

        let response = self.db_client
            .http_client()
            .get(&url)
            .header("apikey", &self.db_client.config().supabase_api_key)
            .header("Authorization", format!("Bearer {}", self.db_client.config().supabase_api_key))
            .header("Accept", "application/json")
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(crate::error::Error::DatabaseError(format!(
                "Failed to fetch all stocks: {} - {}",
                status,
                error_text
            )));
        }

        let records: Vec<InviteListRecord> = response.json().await?;
        info!("Fetched {} stocks from invite_list table", records.len());
        Ok(records)
    }

    /// Get just the stock symbols from the invite_list table (faster query)
    pub async fn get_all_stock_symbols(&self) -> Result<Vec<String>> {
        info!("Fetching stock symbols from invite_list table");
        
        let url = format!(
            "{}/rest/v1/invite_list?select=symbol&order=symbol&limit=1000",
            self.db_client.config().supabase_url
        );

        let response = self.db_client
            .http_client()
            .get(&url)
            .header("apikey", &self.db_client.config().supabase_api_key)
            .header("Authorization", format!("Bearer {}", self.db_client.config().supabase_api_key))
            .header("Accept", "application/json")
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(crate::error::Error::DatabaseError(format!(
                "Failed to fetch stock symbols: {} - {}",
                status,
                error_text
            )));
        }

        let records: Vec<serde_json::Value> = response.json().await?;
        let symbols: Vec<String> = records
            .into_iter()
            .filter_map(|record| record["symbol"].as_str().map(|s| s.to_string()))
            .collect();
        
        info!("Fetched {} stock symbols from invite_list table", symbols.len());
        Ok(symbols)
    }

    /// Clear all invite list records (use with caution)
    pub async fn clear_all_records(&self) -> Result<()> {
        info!("Clearing all invite list records");
        
        let url = format!("{}/rest/v1/invite_list", self.db_client.config().supabase_url);

        let response = self.db_client
            .http_client()
            .delete(&url)
            .header("apikey", &self.db_client.config().supabase_api_key)
            .header("Authorization", format!("Bearer {}", self.db_client.config().supabase_api_key))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(crate::error::Error::DatabaseError(format!(
                "Failed to clear invite list records: {} - {}",
                status,
                error_text
            )));
        }

        info!("Successfully cleared all invite list records");
        Ok(())
    }
}
