// Sentiment analysis CLI binary

use clap::Parser;
use buenotea_sentiment::QSSCalculator;
use tracing::info;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Stock symbol to analyze
    #[arg(short, long)]
    symbol: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Load environment variables
    dotenv::dotenv().ok();

    let args = Args::parse();
    
    info!("Running sentiment analysis for {}", args.symbol);
    
    let calculator = QSSCalculator::new();
    let result = calculator.calculate_qss(&args.symbol).await?;
    
    println!("\n=== Sentiment Analysis for {} ===", args.symbol);
    println!("QSS Score: {:.1}", result.final_qss_score);
    println!("Trading Signal: {:?}", result.trading_signal);
    println!("Confidence: {:.1}%", result.confidence_score * 100.0);
    
    Ok(())
}

