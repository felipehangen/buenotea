use crate::database::models::{ApiUrls, SentimentRecord, SentimentInsert};
use crate::database::DatabaseClient;
use crate::sentiment::models::QSSResult;
use crate::error::Result;
use tracing::info;

pub struct SentimentStorage {
    db_client: DatabaseClient,
}

impl SentimentStorage {
    pub fn new(db_client: DatabaseClient) -> Self {
        Self { db_client }
    }

    pub fn from_env() -> Result<Self> {
        let db_client = DatabaseClient::from_env()?;
        Ok(Self::new(db_client))
    }

    pub async fn store_sentiment_result(
        &self,
        symbol: &str,
        qss_result: &QSSResult,
        api_urls: &ApiUrls,
    ) -> Result<SentimentRecord> {
        info!("Storing sentiment result for {}", symbol);
        
        // Generate GPT explanation
        let gpt_explanation = self.generate_gpt_explanation(symbol, qss_result).await?;
        
        // Convert to database record
        let record = SentimentRecord::from_qss_result(
            symbol.to_string(),
            qss_result,
            api_urls,
            gpt_explanation,
        );

        // Convert to insert record (without auto-generated fields)
        let insert_record = SentimentInsert::from(record);

        // Insert into database
        let inserted_record = self.db_client.insert_sentiment(&insert_record).await?;
        
        info!("Successfully stored sentiment result for {} with ID: {}", 
              symbol, inserted_record.id.unwrap_or(0));
        
        Ok(inserted_record)
    }

    pub async fn get_latest_sentiment(&self, symbol: &str) -> Result<Option<SentimentRecord>> {
        self.db_client.get_latest_sentiment(symbol).await
    }

    pub async fn get_sentiment_history(&self, symbol: &str, limit: Option<i32>) -> Result<Vec<SentimentRecord>> {
        self.db_client.get_sentiment_history(symbol, limit).await
    }

    pub async fn get_all_latest_sentiment(&self) -> Result<Vec<SentimentRecord>> {
        self.db_client.get_all_latest_sentiment().await
    }

    pub async fn test_connection(&self) -> Result<()> {
        self.db_client.test_connection().await
    }

    async fn generate_gpt_explanation(&self, symbol: &str, qss_result: &QSSResult) -> Result<String> {
        // For now, generate a simple explanation based on the QSS score and components
        // In the future, this could call OpenAI's API for more sophisticated explanations
        
        let signal_desc = match qss_result.trading_signal {
            crate::sentiment::models::TradingSignal::StrongBuy => "Strong Buy",
            crate::sentiment::models::TradingSignal::WeakBuy => "Weak Buy", 
            crate::sentiment::models::TradingSignal::Hold => "Hold",
            crate::sentiment::models::TradingSignal::WeakSell => "Weak Sell",
            crate::sentiment::models::TradingSignal::StrongSell => "Strong Sell",
        };

        let confidence_pct = (qss_result.confidence_score * 100.0) as u32;
        
        let mut explanation = format!(
            "{} shows {} sentiment with QSS score of {:.3}. ",
            symbol, signal_desc, qss_result.qss_score
        );

        // Add component analysis
        if qss_result.components.earnings_revisions > 0.5 {
            explanation.push_str("Earnings revisions are very positive. ");
        } else if qss_result.components.earnings_revisions > 0.2 {
            explanation.push_str("Earnings revisions are moderately positive. ");
        } else if qss_result.components.earnings_revisions < -0.2 {
            explanation.push_str("Earnings revisions are negative. ");
        }

        if qss_result.components.relative_strength > 0.3 {
            explanation.push_str("Relative strength is strong. ");
        } else if qss_result.components.relative_strength < -0.3 {
            explanation.push_str("Relative strength is weak. ");
        }

        // Add missing data warnings
        if !qss_result.flags.is_empty() {
            explanation.push_str(&format!("Note: {}", qss_result.flags.join(", ")));
        }

        explanation.push_str(&format!(
            " Recommendation: {} with {}% confidence.",
            signal_desc, confidence_pct
        ));

        Ok(explanation)
    }
}
