// Fundamentals analysis storage implementation for Supabase

use buenotea_infrastructure::DatabaseClient;
use crate::fundamentals_models::{FundamentalsRecord, FundamentalsInsert, FundamentalsApiUrls};
use crate::models::FundamentalsResult;
use buenotea_core::Result;
use tracing::info;

/// Storage handler for fundamentals analysis results
pub struct FundamentalsStorage {
    client: DatabaseClient,
}

impl FundamentalsStorage {
    /// Create a new fundamentals storage handler
    pub fn new(client: DatabaseClient) -> Self {
        Self { client }
    }

    /// Create from environment variables (Supabase URL and key)
    pub fn from_env() -> Result<Self> {
        let client = DatabaseClient::from_env()?;
        Ok(Self::new(client))
    }

    /// Test the database connection
    pub async fn test_connection(&self) -> Result<()> {
        info!("Testing fundamentals storage connection...");
        self.client.test_connection().await?;
        info!("✅ Fundamentals storage connection successful!");
        Ok(())
    }

    /// Store a fundamentals analysis result in the database
    pub async fn store_fundamentals_result(
        &self,
        symbol: &str,
        result: &FundamentalsResult,
        api_urls: &FundamentalsApiUrls,
        gpt_explanation: Option<String>,
        gpt_trading_suggestion: Option<String>,
    ) -> Result<FundamentalsRecord> {
        info!("Storing fundamentals analysis for {}", symbol);

        let record = FundamentalsInsert {
            symbol: symbol.to_string(),
            analysis_date: result.timestamp,
            
            // Core scoring data
            fundamentals_score: result.fundamentals_score,
            trading_signal: result.trading_signal.to_string(),
            confidence_score: result.confidence_score,
            
            // Component scores
            profitability_score: result.components.profitability,
            growth_score: result.components.growth,
            valuation_score: result.components.valuation,
            financial_strength_score: result.components.financial_strength,
            efficiency_score: result.components.efficiency,
            
            // Profitability metrics
            roe: result.metrics.profitability.roe,
            roa: result.metrics.profitability.roa,
            roic: result.metrics.profitability.roic,
            net_profit_margin: result.metrics.profitability.net_profit_margin,
            gross_profit_margin: result.metrics.profitability.gross_profit_margin,
            operating_profit_margin: result.metrics.profitability.operating_profit_margin,
            ebitda_margin: result.metrics.profitability.ebitda_margin,
            
            // Growth metrics
            revenue_growth_yoy: result.metrics.growth.revenue_growth_yoy,
            revenue_growth_qoq: result.metrics.growth.revenue_growth_qoq,
            eps_growth_yoy: result.metrics.growth.eps_growth_yoy,
            eps_growth_qoq: result.metrics.growth.eps_growth_qoq,
            net_income_growth_yoy: result.metrics.growth.net_income_growth_yoy,
            book_value_growth_yoy: result.metrics.growth.book_value_growth_yoy,
            operating_cash_flow_growth_yoy: result.metrics.growth.operating_cash_flow_growth_yoy,
            
            // Valuation metrics
            pe_ratio: result.metrics.valuation.pe_ratio,
            peg_ratio: result.metrics.valuation.peg_ratio,
            ps_ratio: result.metrics.valuation.ps_ratio,
            pb_ratio: result.metrics.valuation.pb_ratio,
            pcf_ratio: result.metrics.valuation.pcf_ratio,
            ev_ebitda: result.metrics.valuation.ev_ebitda,
            ev_sales: result.metrics.valuation.ev_sales,
            pfcf_ratio: result.metrics.valuation.pfcf_ratio,
            
            // Financial strength metrics
            debt_to_equity: result.metrics.financial_strength.debt_to_equity,
            debt_to_assets: result.metrics.financial_strength.debt_to_assets,
            current_ratio: result.metrics.financial_strength.current_ratio,
            quick_ratio: result.metrics.financial_strength.quick_ratio,
            interest_coverage: result.metrics.financial_strength.interest_coverage,
            cash_to_debt: result.metrics.financial_strength.cash_to_debt,
            equity_multiplier: result.metrics.financial_strength.equity_multiplier,
            altman_z_score: result.metrics.financial_strength.altman_z_score,
            
            // Efficiency metrics
            asset_turnover: result.metrics.efficiency.asset_turnover,
            inventory_turnover: result.metrics.efficiency.inventory_turnover,
            receivables_turnover: result.metrics.efficiency.receivables_turnover,
            payables_turnover: result.metrics.efficiency.payables_turnover,
            working_capital_turnover: result.metrics.efficiency.working_capital_turnover,
            days_sales_outstanding: result.metrics.efficiency.days_sales_outstanding,
            days_inventory_outstanding: result.metrics.efficiency.days_inventory_outstanding,
            days_payables_outstanding: result.metrics.efficiency.days_payables_outstanding,
            
            // Company metadata
            sector: result.meta.sector.clone(),
            industry: result.meta.industry.clone(),
            market_cap_category: result.meta.market_cap_category.clone(),
            beta: result.meta.beta,
            dividend_yield: result.meta.dividend_yield,
            payout_ratio: result.meta.payout_ratio,
            shares_outstanding: result.meta.shares_outstanding,
            market_cap: result.meta.market_cap,
            enterprise_value: result.meta.enterprise_value,
            
            // Analysis metadata
            computation_time_ms: Some(result.meta.computation_time_ms as i32),
            data_points_count: Some(result.meta.data_points_count as i32),
            data_freshness: Some(result.meta.data_freshness),
            flags: Some(result.flags.clone()),
            
            // API tracking
            profitability_api_url: api_urls.profitability_api_url.clone(),
            profitability_api_source: api_urls.profitability_api_source.clone(),
            profitability_data_available: Some(api_urls.profitability_data_available),
            profitability_raw_data: api_urls.profitability_raw_data.clone(),
            
            growth_api_url: api_urls.growth_api_url.clone(),
            growth_api_source: api_urls.growth_api_source.clone(),
            growth_data_available: Some(api_urls.growth_data_available),
            growth_raw_data: api_urls.growth_raw_data.clone(),
            
            valuation_api_url: api_urls.valuation_api_url.clone(),
            valuation_api_source: api_urls.valuation_api_source.clone(),
            valuation_data_available: Some(api_urls.valuation_data_available),
            valuation_raw_data: api_urls.valuation_raw_data.clone(),
            
            financial_strength_api_url: api_urls.financial_strength_api_url.clone(),
            financial_strength_api_source: api_urls.financial_strength_api_source.clone(),
            financial_strength_data_available: Some(api_urls.financial_strength_data_available),
            financial_strength_raw_data: api_urls.financial_strength_raw_data.clone(),
            
            efficiency_api_url: api_urls.efficiency_api_url.clone(),
            efficiency_api_source: api_urls.efficiency_api_source.clone(),
            efficiency_data_available: Some(api_urls.efficiency_data_available),
            efficiency_raw_data: api_urls.efficiency_raw_data.clone(),
            
            // AI Analysis
            gpt_explanation,
            gpt_trading_suggestion,
        };

        // Insert into the actual Supabase database
        let stored_record = self.client.insert_fundamentals(&record).await?;
        info!("✅ Fundamentals analysis stored successfully for {} with ID: {}", 
            symbol, stored_record.id.unwrap_or(0));
        Ok(stored_record)
    }

    /// Get all latest fundamentals analysis records
    pub async fn get_all_latest_fundamentals(&self) -> Result<Vec<FundamentalsRecord>> {
        info!("Retrieving all latest fundamentals analysis records...");
        
        let records = self.client.get_all_latest_fundamentals().await?;
        info!("✅ Retrieved {} latest fundamentals records", records.len());
        Ok(records)
    }

    /// Get fundamentals history for a specific symbol
    pub async fn get_fundamentals_history(&self, symbol: &str) -> Result<Vec<FundamentalsRecord>> {
        info!("Retrieving fundamentals history for {}", symbol);
        
        let records = self.client.get_fundamentals_by_symbol(symbol, Some(100)).await?;
        info!("✅ Retrieved {} fundamentals records for {}", records.len(), symbol);
        Ok(records)
    }

    /// Get fundamentals analysis by ID
    pub async fn get_fundamentals_by_id(&self, id: i64) -> Result<FundamentalsRecord> {
        info!("Retrieving fundamentals analysis with ID: {}", id);
        
        // Get all records and find the one with matching ID
        let all_records = self.client.get_fundamentals_by_symbol("", Some(1000)).await?;
        if let Some(record) = all_records.iter().find(|r| r.id == Some(id)) {
            info!("✅ Retrieved fundamentals record with ID: {}", id);
            Ok(record.clone())
        } else {
            Err(buenotea_core::Error::ApiError(
                "Supabase".to_string(),
                format!("No fundamentals record found with ID: {}", id)
            ))
        }
    }

    /// Delete fundamentals analysis by ID
    pub async fn delete_fundamentals(&self, id: i64) -> Result<()> {
        info!("Deleting fundamentals analysis with ID: {}", id);
        
        let url = format!("{}/rest/v1/fundamentals?id=eq.{}", 
                         self.client.config().supabase_url, id);
        
        let response = self.client.http_client()
            .delete(&url)
            .header("apikey", &self.client.config().supabase_api_key)
            .header("Authorization", format!("Bearer {}", self.client.config().supabase_api_key))
            .send()
            .await?;

        if response.status().is_success() {
            info!("✅ Deleted fundamentals analysis with ID: {}", id);
            Ok(())
        } else {
            let error_text = response.text().await?;
            Err(buenotea_core::Error::ApiError(
                "Supabase".to_string(),
                format!("Failed to delete fundamentals with ID {}: {}", id, error_text)
            ))
        }
    }

    /// Get fundamentals analysis statistics
    pub async fn get_fundamentals_stats(&self) -> Result<serde_json::Value> {
        info!("Retrieving fundamentals analysis statistics...");
        
        let all_records = self.client.get_fundamentals_by_symbol("", Some(1000)).await?;
        
        let mut symbols: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut scores: Vec<f64> = Vec::new();
        let mut rating_distribution: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
        
        for record in &all_records {
            symbols.insert(record.symbol.clone());
            scores.push(record.fundamentals_score);
            
            let trading_signal = &record.trading_signal;
            *rating_distribution.entry(trading_signal.clone()).or_insert(0) += 1;
        }
        
        let stats = serde_json::json!({
            "total_records": all_records.len(),
            "unique_symbols": symbols.len(),
            "average_score": if scores.is_empty() { 0.0 } else { scores.iter().sum::<f64>() / scores.len() as f64 },
            "rating_distribution": rating_distribution,
            "data_freshness": if all_records.is_empty() { 0.0 } else {
                all_records.iter().map(|r| r.data_freshness.unwrap_or(0.0)).sum::<f64>() / all_records.len() as f64
            }
        });
        
        info!("✅ Retrieved fundamentals statistics");
        Ok(stats)
    }
}