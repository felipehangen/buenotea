// Example: Fundamentals analysis for AAPL
// This example demonstrates how to use the fundamentals calculator to analyze Apple Inc.

use buenotea_fundamentals::{FundamentalsCalculator, FundamentalsResult};
use buenotea_error::Result;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("Starting AAPL fundamentals analysis example");

    // Load environment variables
    dotenv::dotenv().ok();

    // Create fundamentals calculator
    let calculator = FundamentalsCalculator::new();

    // Analyze AAPL fundamentals
    let symbol = "AAPL";
    info!("Analyzing fundamentals for {}", symbol);

    match calculator.calculate_fundamentals(symbol).await {
        Ok(result) => {
            // Display the results
            display_fundamentals_result(&result);
            
            // Save to JSON file
            save_to_json(&result).await?;
            
            info!("Fundamentals analysis completed successfully for {}", symbol);
        }
        Err(e) => {
            error!("Failed to calculate fundamentals for {}: {}", symbol, e);
            return Err(e);
        }
    }

    Ok(())
}

fn display_fundamentals_result(result: &FundamentalsResult) {
    println!("\n=== FUNDAMENTALS ANALYSIS FOR {} ===", result.symbol);
    println!("Overall Score: {:.2} (range: -1.0 to +1.0)", result.fundamentals_score);
    println!("Trading Signal: {} {}", result.trading_signal.emoji(), result.trading_signal.description());
    println!("Confidence: {:.1}%", result.confidence_score * 100.0);
    
    println!("\n--- Component Scores ---");
    println!("Profitability: {:.2} (25% weight)", result.components.profitability);
    println!("Growth: {:.2} (25% weight)", result.components.growth);
    println!("Valuation: {:.2} (25% weight)", result.components.valuation);
    println!("Financial Strength: {:.2} (15% weight)", result.components.financial_strength);
    println!("Efficiency: {:.2} (10% weight)", result.components.efficiency);
    
    println!("\n--- Financial Metrics ---");
    display_profitability_metrics(&result.metrics.profitability);
    display_growth_metrics(&result.metrics.growth);
    display_valuation_metrics(&result.metrics.valuation);
    display_financial_strength_metrics(&result.metrics.financial_strength);
    display_efficiency_metrics(&result.metrics.efficiency);
    
    if !result.flags.is_empty() {
        println!("\n--- Flags ---");
        for flag in &result.flags {
            println!("⚠️  {}", flag);
        }
    }
    
    println!("\n--- Metadata ---");
    println!("Analysis Time: {}ms", result.meta.computation_time_ms);
    println!("Data Points: {}", result.meta.data_points_count);
    println!("Data Freshness: {:.1}%", result.meta.data_freshness * 100.0);
    
    if let Some(sector) = &result.meta.sector {
        println!("Sector: {}", sector);
    }
    if let Some(industry) = &result.meta.industry {
        println!("Industry: {}", industry);
    }
    if let Some(market_cap) = result.meta.market_cap {
        println!("Market Cap: ${}", market_cap);
    }
    if let Some(beta) = result.meta.beta {
        println!("Beta: {:.2}", beta);
    }
    if let Some(dividend_yield) = result.meta.dividend_yield {
        println!("Dividend Yield: {:.2}%", dividend_yield * 100.0);
    }
    
    println!("\nTimestamp: {}", result.timestamp.format("%Y-%m-%d %H:%M:%S UTC"));
}

fn display_profitability_metrics(metrics: &sentiment_backend::fundamentals::ProfitabilityMetrics) {
    println!("Profitability Metrics:");
    if let Some(roe) = metrics.roe {
        println!("  ROE: {:.2}%", roe * 100.0);
    }
    if let Some(roa) = metrics.roa {
        println!("  ROA: {:.2}%", roa * 100.0);
    }
    if let Some(roic) = metrics.roic {
        println!("  ROIC: {:.2}%", roic * 100.0);
    }
    if let Some(net_margin) = metrics.net_profit_margin {
        println!("  Net Profit Margin: {:.2}%", net_margin * 100.0);
    }
    if let Some(gross_margin) = metrics.gross_profit_margin {
        println!("  Gross Profit Margin: {:.2}%", gross_margin * 100.0);
    }
}

fn display_growth_metrics(metrics: &sentiment_backend::fundamentals::GrowthMetrics) {
    println!("Growth Metrics:");
    if let Some(revenue_growth) = metrics.revenue_growth_yoy {
        println!("  Revenue Growth (YoY): {:.2}%", revenue_growth * 100.0);
    }
    if let Some(eps_growth) = metrics.eps_growth_yoy {
        println!("  EPS Growth (YoY): {:.2}%", eps_growth * 100.0);
    }
    if let Some(net_income_growth) = metrics.net_income_growth_yoy {
        println!("  Net Income Growth (YoY): {:.2}%", net_income_growth * 100.0);
    }
    if let Some(book_value_growth) = metrics.book_value_growth_yoy {
        println!("  Book Value Growth (YoY): {:.2}%", book_value_growth * 100.0);
    }
}

fn display_valuation_metrics(metrics: &sentiment_backend::fundamentals::ValuationMetrics) {
    println!("Valuation Metrics:");
    if let Some(pe) = metrics.pe_ratio {
        println!("  P/E Ratio: {:.2}", pe);
    }
    if let Some(peg) = metrics.peg_ratio {
        println!("  PEG Ratio: {:.2}", peg);
    }
    if let Some(ps) = metrics.ps_ratio {
        println!("  P/S Ratio: {:.2}", ps);
    }
    if let Some(pb) = metrics.pb_ratio {
        println!("  P/B Ratio: {:.2}", pb);
    }
    if let Some(ev_ebitda) = metrics.ev_ebitda {
        println!("  EV/EBITDA: {:.2}", ev_ebitda);
    }
}

fn display_financial_strength_metrics(metrics: &sentiment_backend::fundamentals::FinancialStrengthMetrics) {
    println!("Financial Strength Metrics:");
    if let Some(debt_equity) = metrics.debt_to_equity {
        println!("  Debt-to-Equity: {:.2}", debt_equity);
    }
    if let Some(debt_assets) = metrics.debt_to_assets {
        println!("  Debt-to-Assets: {:.2}", debt_assets);
    }
    if let Some(current_ratio) = metrics.current_ratio {
        println!("  Current Ratio: {:.2}", current_ratio);
    }
    if let Some(quick_ratio) = metrics.quick_ratio {
        println!("  Quick Ratio: {:.2}", quick_ratio);
    }
    if let Some(interest_coverage) = metrics.interest_coverage {
        println!("  Interest Coverage: {:.2}", interest_coverage);
    }
}

fn display_efficiency_metrics(metrics: &sentiment_backend::fundamentals::EfficiencyMetrics) {
    println!("Efficiency Metrics:");
    if let Some(asset_turnover) = metrics.asset_turnover {
        println!("  Asset Turnover: {:.2}", asset_turnover);
    }
    if let Some(inventory_turnover) = metrics.inventory_turnover {
        println!("  Inventory Turnover: {:.2}", inventory_turnover);
    }
    if let Some(receivables_turnover) = metrics.receivables_turnover {
        println!("  Receivables Turnover: {:.2}", receivables_turnover);
    }
    if let Some(dso) = metrics.days_sales_outstanding {
        println!("  Days Sales Outstanding: {:.0}", dso);
    }
}

async fn save_to_json(result: &FundamentalsResult) -> Result<()> {
    use std::fs::File;
    use std::io::Write;
    
    let json = serde_json::to_string_pretty(result)
        .map_err(|e| sentiment_backend::error::Error::Json(e))?;
    
    let filename = format!("aapl_fundamentals_{}.json", 
        result.timestamp.format("%Y%m%d_%H%M%S"));
    
    let mut file = File::create(&filename)
        .map_err(|e| sentiment_backend::error::Error::ValidationError { message: format!("Failed to create file: {}", e) })?;
    
    file.write_all(json.as_bytes())
        .map_err(|e| sentiment_backend::error::Error::ValidationError { message: format!("Failed to write file: {}", e) })?;
    
    info!("Fundamentals analysis saved to {}", filename);
    Ok(())
}
