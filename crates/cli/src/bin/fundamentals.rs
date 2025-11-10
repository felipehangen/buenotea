// Fundamentals analysis CLI binary

use clap::Parser;
use buenotea_fundamentals::FundamentalsCalculator;
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
    
    info!("Running fundamentals analysis for {}", args.symbol);
    
    let calculator = FundamentalsCalculator::new();
    let result = calculator.analyze_fundamentals(&args.symbol).await?;
    
    println!("\n=== Fundamentals Analysis for {} ===", args.symbol);
    println!("Overall Score: {:.1}", result.overall_score);
    println!("Profitability: {:.1}", result.category_scores.profitability);
    println!("Growth: {:.1}", result.category_scores.growth);
    println!("Valuation: {:.1}", result.category_scores.valuation);
    
    Ok(())
}

