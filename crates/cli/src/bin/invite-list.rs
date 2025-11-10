// Invite list CLI binary

use clap::Parser;
use buenotea_invite_list::{SP500Fetcher, InviteListCalculator};
use tracing::info;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Fetch and analyze invite list
    #[arg(short, long, default_value_t = false)]
    analyze: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Load environment variables
    dotenv::dotenv().ok();

    let _args = Args::parse();
    
    info!("Fetching S&P 500 list");
    
    let fetcher = SP500Fetcher::new();
    let stocks = fetcher.fetch_sp500_list().await?;
    
    println!("\n=== S&P 500 Stocks ===");
    println!("Total stocks: {}", stocks.len());
    
    let calculator = InviteListCalculator::new();
    let results = calculator.analyze_list(&stocks).await?;
    
    println!("Tradeable stocks: {}", results.len());
    
    Ok(())
}

