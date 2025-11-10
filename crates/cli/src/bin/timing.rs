// Timing analysis CLI binary

use clap::Parser;
use buenotea_timing::{TTSCalculator, TimingStorage, create_timing_record_with_tracking};
use tracing::info;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Stock symbol to analyze
    #[arg(short, long)]
    symbol: String,

    /// Save to database
    #[arg(short, long, default_value_t = false)]
    save: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Load environment variables
    dotenv::dotenv().ok();

    let args = Args::parse();
    
    info!("Running timing analysis for {}", args.symbol);
    
    let mut calculator = TTSCalculator::new();
    let (result, tracking) = calculator.calculate_tts_with_tracking(&args.symbol).await?;
    
    println!("\n=== Timing Analysis for {} ===", args.symbol);
    println!("TTS Score: {:.2}", result.tts_score);
    println!("Trading Signal: {:?}", result.trading_signal);
    println!("Confidence: {:.1}%", result.confidence_score * 100.0);
    println!("Risk Level: {}", format!("{:?}", result.risk_assessment.risk_level));
    println!("\nTrend Analysis:");
    println!("  Short-term: {:?}", result.trend_analysis.short_term);
    println!("  Medium-term: {:?}", result.trend_analysis.medium_term);
    println!("  Long-term: {:?}", result.trend_analysis.long_term);
    
    if args.save {
        let storage = TimingStorage::from_env()?;
        let record = create_timing_record_with_tracking(result, tracking);
        let id = storage.store_timing_record(&record).await?;
        info!("Saved timing analysis to database with ID: {}", id);
        println!("\n✅ Saved to database (ID: {})", id);
    }
    
    Ok(())
}

