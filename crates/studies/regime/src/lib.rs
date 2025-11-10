// Market Regime Analysis Module
// Analyzes market conditions and provides regime-based TTS scores

pub mod models;
pub mod calculator;
pub mod market_regime_models;
pub mod market_regime_storage;

pub use models::*;
pub use calculator::MarketRegimeCalculator;
// Re-export market_regime_models types, but exclude ChatGPTMarketAnalysis (already in models)
pub use market_regime_models::{MarketRegimeRecord, create_market_regime_record_with_tracking};
pub use market_regime_storage::MarketRegimeStorage;
