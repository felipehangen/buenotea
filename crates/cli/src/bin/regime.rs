// Regime analysis CLI binary

use clap::Parser;
use buenotea_regime::MarketRegimeCalculator;
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
    
    info!("Running regime analysis for {}", args.symbol);
    
    let calculator = MarketRegimeCalculator::new();
    let result = calculator.analyze_regime(&args.symbol).await?;
    
    println!("\n=== Regime Analysis for {} ===", args.symbol);
    println!("Market Regime: {:?}", result.market_regime);
    println!("TTS Score: {:.3}", result.tts_score);
    println!("Trading Signal: {:?}", result.trading_signal);
    
    Ok(())
}

