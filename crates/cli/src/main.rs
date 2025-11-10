// BuenoTea CLI main entry point
// Use the individual binary targets instead: timing, sentiment, regime, fundamentals, invite-list

fn main() {
    println!("BuenoTea - Stock Analysis System");
    println!("\nAvailable commands:");
    println!("  timing         - Run timing analysis");
    println!("  sentiment      - Run sentiment analysis");
    println!("  regime         - Run regime analysis");
    println!("  fundamentals   - Run fundamentals analysis");
    println!("  invite-list    - Analyze invite list");
    println!("\nExample: cargo run --bin timing -- --symbol AAPL --save");
}

