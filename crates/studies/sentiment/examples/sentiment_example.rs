// Example demonstrating how to use the QSS sentiment analysis system
// This shows how to calculate buy/sell signals for stocks

use buenotea_sentiment::{QSSCalculator, QSSResult};
use buenotea_sentiment::models::TradingSignal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("=== QSS Sentiment Analysis Example ===\n");

    // Create a QSS calculator (automatically uses API keys from environment variables)
    let calculator = QSSCalculator::new();
    
    // Check if we have API keys available
    let has_api_keys = std::env::var("POLYGON_API_KEY").is_ok() && 
                      std::env::var("FMP_API_KEY").is_ok() &&
                      std::env::var("FINNHUB_API_KEY").is_ok() &&
                      std::env::var("ALPHA_VANTAGE_API_KEY").is_ok();
    
    if has_api_keys {
        println!("✅ Using real API keys from environment variables");
    } else {
        println!("⚠️  No API keys found in environment - using mock data for demonstration");
        println!("   To use real data, set these environment variables:");
        println!("   - POLYGON_API_KEY");
        println!("   - FMP_API_KEY"); 
        println!("   - FINNHUB_API_KEY");
        println!("   - ALPHA_VANTAGE_API_KEY");
    }
    println!();

    // Example symbols to analyze
    let symbols = vec!["AAPL", "MSFT", "GOOGL", "TSLA", "NVDA"];

    println!("Analyzing sentiment for the following symbols:");
    for symbol in &symbols {
        println!("  - {}", symbol);
    }
    println!();

    let mut results = Vec::new();

    // Calculate QSS for each symbol
    for symbol in symbols {
        println!("Calculating QSS for {}...", symbol);
        
        match calculator.calculate_qss(symbol).await {
            Ok(result) => {
                results.push(result.clone());
                print_qss_result(&result);
            }
            Err(e) => {
                println!("❌ Error calculating QSS for {}: {}", symbol, e);
            }
        }
        println!();
    }

    // Generate portfolio-level analysis
    if !results.is_empty() {
        generate_portfolio_analysis(&results);
    }

    println!("=== Analysis Complete ===");
    Ok(())
}

/// Print a formatted QSS result
fn print_qss_result(result: &QSSResult) {
    println!("📊 {} Analysis Results", result.symbol);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Main score and signal
    println!("🎯 QSS Score: {:.3} {}", result.qss_score, result.trading_signal.emoji());
    println!("📈 Trading Signal: {:?}", result.trading_signal);
    println!("📝 Description: {}", result.trading_signal.description());
    println!("💰 Position Size: {:.1}% of portfolio", result.trading_signal.position_size() * 100.0);
    
    // Confidence and metadata
    println!("🎲 Confidence: {:.1}%", result.confidence_score * 100.0);
    println!("⏱️  Computation Time: {}ms", result.meta.computation_time_ms);
    println!("📊 Data Points: {}", result.meta.data_points_count);
    println!("📅 Trend Direction: {:.3}", result.meta.trend_direction);
    
    // Component breakdown
    println!("\n🔍 Component Analysis:");
    println!("  📈 Earnings Revisions: {:.3} (40% weight)", result.components.earnings_revisions);
    println!("  💪 Relative Strength:  {:.3} (30% weight)", result.components.relative_strength);
    println!("  📉 Short Interest:     {:.3} (20% weight)", result.components.short_interest);
    println!("  🎯 Options Flow:       {:.3} (10% weight)", result.components.options_flow);
    
    // Flags and warnings
    if !result.flags.is_empty() {
        println!("\n⚠️  Flags:");
        for flag in &result.flags {
            println!("  - {}", flag);
        }
    }
}

/// Generate portfolio-level analysis
fn generate_portfolio_analysis(results: &[QSSResult]) {
    println!("📈 PORTFOLIO ANALYSIS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Count signals by type
    let mut signal_counts = std::collections::HashMap::new();
    let mut total_confidence = 0.0;
    let mut total_qss = 0.0;
    
    for result in results {
        let signal_type = format!("{:?}", result.trading_signal);
        *signal_counts.entry(signal_type).or_insert(0) += 1;
        total_confidence += result.confidence_score;
        total_qss += result.qss_score;
    }
    
    let count = results.len() as f64;
    let avg_confidence = total_confidence / count;
    let avg_qss = total_qss / count;
    
    println!("📊 Signal Distribution:");
    for (signal, count) in &signal_counts {
        println!("  {}: {} stocks", signal, count);
    }
    
    println!("\n📈 Portfolio Metrics:");
    println!("  Average QSS Score: {:.3}", avg_qss);
    println!("  Average Confidence: {:.1}%", avg_confidence * 100.0);
    
    // Generate portfolio recommendation
    let portfolio_signal = match avg_qss {
        qss if qss >= 0.6 => "🟢 Strong Bullish - Consider increasing equity exposure",
        qss if qss >= 0.2 => "🟡 Mild Bullish - Maintain current allocation",
        qss if qss >= -0.2 => "⚪ Neutral - Balanced approach recommended",
        qss if qss >= -0.6 => "🟠 Mild Bearish - Consider reducing equity exposure",
        _ => "🔴 Strong Bearish - Consider defensive positioning",
    };
    
    println!("\n🎯 Portfolio Recommendation:");
    println!("  {}", portfolio_signal);
    
    // Top and bottom performers
    let mut sorted_results = results.to_vec();
    sorted_results.sort_by(|a, b| b.qss_score.partial_cmp(&a.qss_score).unwrap());
    
    if sorted_results.len() >= 2 {
        println!("\n🏆 Top Performer: {} ({:.3})", 
                sorted_results[0].symbol, sorted_results[0].qss_score);
        println!("📉 Bottom Performer: {} ({:.3})", 
                sorted_results.last().unwrap().symbol, sorted_results.last().unwrap().qss_score);
    }
    
    // Risk assessment
    let high_confidence_count = results.iter().filter(|r| r.confidence_score > 0.8).count();
    let low_confidence_count = results.iter().filter(|r| r.confidence_score < 0.5).count();
    
    println!("\n⚠️  Risk Assessment:");
    println!("  High Confidence (>80%): {} stocks", high_confidence_count);
    println!("  Low Confidence (<50%): {} stocks", low_confidence_count);
    
    if low_confidence_count > high_confidence_count {
        println!("  ⚠️  WARNING: Many low-confidence signals. Consider additional analysis.");
    }
    
    println!();
}

/// Example of how to use the sentiment system programmatically
#[allow(dead_code)]
async fn programmatic_example() -> Result<(), Box<dyn std::error::Error>> {
    let calculator = QSSCalculator::new();
    
    // Calculate QSS for a single stock
    let result = calculator.calculate_qss("AAPL").await?;
    
    // Check if we should buy
    if matches!(result.trading_signal, TradingSignal::StrongBuy | TradingSignal::WeakBuy) {
        let position_size = result.trading_signal.position_size();
        println!("Buy signal detected! Recommended position size: {:.1}%", position_size * 100.0);
        
        // Only proceed if confidence is high enough
        if result.confidence_score > 0.7 {
            println!("High confidence signal - proceeding with trade");
        } else {
            println!("Low confidence signal - additional analysis recommended");
        }
    }
    
    // Check for warning flags
    if result.flags.contains(&"earnings_window".to_string()) {
        println!("⚠️  Earnings announcement approaching - be cautious");
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sentiment_example() {
        let calculator = QSSCalculator::new();
        let result = calculator.calculate_qss("AAPL").await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_portfolio_analysis() {
        // Mock results for testing
        let results = vec![
            QSSResult {
                symbol: "AAPL".to_string(),
                qss_score: 0.5,
                trading_signal: TradingSignal::WeakBuy,
                components: sentiment_backend::sentiment::models::QSSComponents {
                    earnings_revisions: 0.5,
                    relative_strength: 0.3,
                    short_interest: 0.2,
                    options_flow: 0.1,
                },
                flags: vec![],
                confidence_score: 0.8,
                timestamp: chrono::Utc::now(),
                meta: sentiment_backend::sentiment::models::QSSMeta {
                    computation_time_ms: 100,
                    data_points_count: 300,
                    trend_direction: 0.5,
                    data_freshness: 0.9,
                },
            }
        ];
        
        // This should not panic
        generate_portfolio_analysis(&results);
    }
}
