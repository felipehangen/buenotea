// Sentiment analysis module for QSS (Quantitative Sentiment Score) calculations
// This module provides the core functionality for calculating buy/sell signals
// based on multiple data sources and sentiment indicators.

pub mod calculator;
pub mod models;
pub mod sentiment_storage;

// Re-export main types for easy access
pub use calculator::QSSCalculator;
pub use models::*;
pub use sentiment_storage::SentimentStorage;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sentiment_module_compiles() {
        // Basic test to ensure the module compiles correctly
        let _calculator = QSSCalculator::new();
    }
}
