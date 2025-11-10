// Example: Run sentiment (QSS) analysis on a single stock and save to Supabase
// This analyzes sentiment data and saves to sentiment_history

use buenotea_sentiment::{QSSCalculator, SentimentStorage, create_sentiment_record_with_tracking};
use buenotea_infrastructure::sentiment_models::ApiUrls;
use buenotea_infrastructure::DatabaseClient;
use buenotea_core::Result;
use dotenv;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing for logging
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(tracing::Level::INFO)
            .finish()
    ).ok();

    println!("🚀 Starting Sentiment (QSS) Analysis (AAPL example)...\n");

    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Step 1: Run sentiment analysis for AAPL
    println!("📈 Analyzing AAPL sentiment (QSS)...");
    let calculator = QSSCalculator::new();
    let symbol = "AAPL";
    
    let result = calculator.calculate_qss(symbol).await?;
    println!("✅ Sentiment analysis completed!");
    println!("   Symbol: {}", result.symbol);
    println!("   QSS Score: {:.3}", result.qss_score);
    println!("   Signal: {:?}", result.trading_signal);
    println!("   Confidence: {:.1}%", result.confidence_score * 100.0);

    // Step 2: Prepare API URLs tracking (in a real scenario, you'd get this from the calculator)
    let api_urls = ApiUrls::default(); // Using default for this example

    // Step 3: Generate simple GPT explanation
    let gpt_explanation = format!(
        "{} shows {:?} sentiment with QSS score of {:.3}. Confidence: {:.1}%",
        result.symbol,
        result.trading_signal,
        result.qss_score,
        result.confidence_score * 100.0
    );

    // Step 4: Create database record
    let record_to_store = create_sentiment_record_with_tracking(
        result.clone(),
        api_urls,
        gpt_explanation,
    );

    // Step 5: Save to Supabase
    println!("\n💾 Saving to Supabase sentiment_history table...");
    let db_client = DatabaseClient::from_env()?;
    let sentiment_storage = SentimentStorage::new(db_client);

    let record_id = match sentiment_storage.store_sentiment_record(&record_to_store).await {
        Ok(id) => {
            println!("✅ Successfully saved to database with ID: {}", id);
            id
        }
        Err(e) => {
            eprintln!("❌ Failed to save to database: {}", e);
            return Err(e);
        }
    };

    // Step 6: Display component scores
    println!("\n📊 Component Scores:");
    println!("   Earnings Revisions: {:.3} (40% weight)", result.components.earnings_revisions);
    println!("   Relative Strength:  {:.3} (30% weight)", result.components.relative_strength);
    println!("   Short Interest:     {:.3} (20% weight)", result.components.short_interest);
    println!("   Options Flow:       {:.3} (10% weight)", result.components.options_flow);

    // Step 7: Display metadata
    println!("\n📈 Metadata:");
    println!("   Data Points: {}", result.meta.data_points_count);
    println!("   Computation Time: {}ms", result.meta.computation_time_ms);
    println!("   Trend Direction: {:.3}", result.meta.trend_direction);
    println!("   Data Freshness: {:.3}", result.meta.data_freshness);

    // Step 8: Summary
    println!("\n✨ Summary:");
    println!("   Symbol: {}", result.symbol);
    println!("   QSS Score: {:.3}", result.qss_score);
    println!("   Signal: {:?}", result.trading_signal);
    println!("   Confidence: {:.1}%", result.confidence_score * 100.0);
    println!("   Database ID: {}", record_id);

    // Step 9: Display flags (if any)
    if !result.flags.is_empty() {
        println!("\n⚠️  Flags:");
        for flag in &result.flags {
            println!("   - {}", flag);
        }
    }

    println!("\n🎉 Sentiment analysis successfully stored in Supabase!");

    Ok(())
}

