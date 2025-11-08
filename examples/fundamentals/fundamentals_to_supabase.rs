// Example: Fundamentals analysis with Supabase storage
// This example demonstrates how to calculate fundamentals scores and save them to Supabase

use sentiment_backend::fundamentals::{FundamentalsCalculator, FundamentalsResult};
use sentiment_backend::database::{FundamentalsStorage, FundamentalsApiUrls};
use sentiment_backend::ai::ChatGPTService;
use sentiment_backend::error::Result;
use dotenv::dotenv;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load environment variables
    dotenv().ok();

    println!("=== FUNDAMENTALS ANALYSIS WITH SUPABASE STORAGE ===\n");

    // Initialize fundamentals calculator
    let calculator = FundamentalsCalculator::new();
    
    // Initialize database storage
    let storage = FundamentalsStorage::from_env()?;
    
    // Initialize ChatGPT service for AI analysis
    let chatgpt_api_key = std::env::var("CHATGPT_API_KEY").unwrap_or_else(|_| "demo-key".to_string());
    let chatgpt_service = ChatGPTService::new(chatgpt_api_key);
    
    // Test database connection
    println!("🔗 Testing Supabase connection...");
    storage.test_connection().await?;
    println!("✅ Connected to Supabase successfully!\n");

    // Analyze multiple stocks and save to database
    let symbols = vec!["AAPL", "MSFT", "GOOGL", "TSLA", "NVDA"];
    
    for symbol in symbols {
        println!("📊 Analyzing {} fundamentals and saving to database...", symbol);
        
        // Calculate fundamentals
        let fundamentals_result = calculator.calculate_fundamentals(symbol).await?;
        
        // Create API URLs (populated with mock data for demonstration)
        let api_urls = FundamentalsApiUrls {
            profitability_api_url: Some(format!("https://financialmodelingprep.com/api/v3/ratios/{}", symbol)),
            profitability_api_source: Some("FMP".to_string()),
            profitability_data_available: false, // Using mock data
            profitability_raw_data: Some(serde_json::json!({"source": "mock_data", "symbol": symbol, "timestamp": chrono::Utc::now()})),
            
            growth_api_url: Some(format!("https://financialmodelingprep.com/api/v3/financial-growth/{}", symbol)),
            growth_api_source: Some("FMP".to_string()),
            growth_data_available: false, // Using mock data
            growth_raw_data: Some(serde_json::json!({"source": "mock_data", "symbol": symbol, "timestamp": chrono::Utc::now()})),
            
            valuation_api_url: Some(format!("https://financialmodelingprep.com/api/v3/key-metrics/{}", symbol)),
            valuation_api_source: Some("FMP".to_string()),
            valuation_data_available: false, // Using mock data
            valuation_raw_data: Some(serde_json::json!({"source": "mock_data", "symbol": symbol, "timestamp": chrono::Utc::now()})),
            
            financial_strength_api_url: Some(format!("https://financialmodelingprep.com/api/v3/ratios/{}", symbol)),
            financial_strength_api_source: Some("FMP".to_string()),
            financial_strength_data_available: false, // Using mock data
            financial_strength_raw_data: Some(serde_json::json!({"source": "mock_data", "symbol": symbol, "timestamp": chrono::Utc::now()})),
            
            efficiency_api_url: Some(format!("https://financialmodelingprep.com/api/v3/ratios/{}", symbol)),
            efficiency_api_source: Some("FMP".to_string()),
            efficiency_data_available: false, // Using mock data
            efficiency_raw_data: Some(serde_json::json!({"source": "mock_data", "symbol": symbol, "timestamp": chrono::Utc::now()})),
        };
        
        // Generate ChatGPT explanation
        let gpt_explanation = generate_chatgpt_explanation(&chatgpt_service, symbol, &fundamentals_result).await;
        let gpt_trading_suggestion = generate_chatgpt_trading_suggestion(&chatgpt_service, symbol, &fundamentals_result).await;
        
        // Store in database
        match storage.store_fundamentals_result(symbol, &fundamentals_result, &api_urls, gpt_explanation, gpt_trading_suggestion).await {
            Ok(record) => {
                println!("✅ {} saved to database with ID: {}", symbol, record.id.unwrap_or(0));
                println!("   Fundamentals Score: {:.2} | Signal: {} {}", 
                    fundamentals_result.fundamentals_score, 
                    fundamentals_result.trading_signal.emoji(),
                    fundamentals_result.trading_signal
                );
                println!("   Profitability: {:.2} | Growth: {:.2} | Valuation: {:.2} | Financial Strength: {:.2} | Efficiency: {:.2}", 
                    fundamentals_result.components.profitability,
                    fundamentals_result.components.growth,
                    fundamentals_result.components.valuation,
                    fundamentals_result.components.financial_strength,
                    fundamentals_result.components.efficiency
                );
                println!("   Confidence: {:.1}% | Data Points: {}", 
                    fundamentals_result.confidence_score * 100.0,
                    fundamentals_result.meta.data_points_count
                );
            }
            Err(e) => {
                println!("❌ Failed to save {}: {}", symbol, e);
            }
        }
        
        println!();
    }

    // Retrieve and display latest results
    println!("📋 Retrieving latest fundamentals data from database...\n");
    
    let latest_records = storage.get_all_latest_fundamentals().await?;
    
    if latest_records.is_empty() {
        println!("No fundamentals records found in database.");
    } else {
        println!("Found {} fundamentals records:\n", latest_records.len());
        
        for record in latest_records {
            println!("📈 {} (ID: {})", record.symbol, record.id.unwrap_or(0));
            println!("   Date: {}", record.analysis_date.format("%Y-%m-%d %H:%M:%S UTC"));
            println!("   Fundamentals Score: {:.2} | Signal: {}", 
                record.fundamentals_score, record.trading_signal);
            println!("   Components: P:{:.2} G:{:.2} V:{:.2} F:{:.2} E:{:.2}", 
                record.profitability_score, record.growth_score, record.valuation_score,
                record.financial_strength_score, record.efficiency_score);
            println!("   Confidence: {:.1}%", record.confidence_score * 100.0);
            if let Some(explanation) = &record.gpt_explanation {
                println!("   GPT Analysis: {}", explanation);
            }
            if let Some(suggestion) = &record.gpt_trading_suggestion {
                println!("   GPT Suggestion: {}", suggestion);
            }
            println!();
        }
    }

    // Display statistics
    println!("📊 Fundamentals Analysis Statistics:\n");
    let stats = storage.get_fundamentals_stats().await?;
    println!("{}", serde_json::to_string_pretty(&stats)?);

    println!("🎉 Fundamentals analysis and storage completed successfully!");
    
    Ok(())
}

/// Generate ChatGPT explanation for fundamentals analysis
async fn generate_chatgpt_explanation(
    chatgpt_service: &ChatGPTService,
    symbol: &str,
    result: &FundamentalsResult,
) -> Option<String> {
    let prompt = format!(
        "Analyze the fundamentals score for {} stock and provide a brief explanation (2-3 sentences) of what this score reveals about the company's financial health. 

Fundamentals Score: {:.2} (range: -1.0 to +1.0)
Trading Signal: {}
Component Scores:
- Profitability: {:.2}
- Growth: {:.2}  
- Valuation: {:.2}
- Financial Strength: {:.2}
- Efficiency: {:.2}

Key Metrics:
- ROE: {:.1}%
- P/E Ratio: {:.1}
- Revenue Growth: {:.1}%
- Market Cap: ${}B

Focus on the most significant strengths and weaknesses revealed by this analysis.",
        symbol,
        result.fundamentals_score,
        result.trading_signal,
        result.components.profitability,
        result.components.growth,
        result.components.valuation,
        result.components.financial_strength,
        result.components.efficiency,
        result.metrics.profitability.roe.unwrap_or(0.0) * 100.0,
        result.metrics.valuation.pe_ratio.unwrap_or(0.0),
        result.metrics.growth.revenue_growth_yoy.unwrap_or(0.0) * 100.0,
        result.meta.market_cap.unwrap_or(0) / 1_000_000_000
    );

    match chatgpt_service.generate_response(&prompt).await {
        Ok(response) => {
            info!("✅ Generated ChatGPT explanation for {}", symbol);
            Some(response)
        }
        Err(e) => {
            error!("❌ Failed to generate ChatGPT explanation for {}: {}", symbol, e);
            None
        }
    }
}

/// Generate ChatGPT trading suggestion
async fn generate_chatgpt_trading_suggestion(
    chatgpt_service: &ChatGPTService,
    symbol: &str,
    result: &FundamentalsResult,
) -> Option<String> {
    let prompt = format!(
        "Based on the fundamentals analysis for {} stock, provide a brief trading suggestion (2-3 sentences) for an investor. Consider the overall score, trading signal, and key metrics.

Fundamentals Score: {:.2} (range: -1.0 to +1.0)
Trading Signal: {}
Confidence: {:.1}%

Key Considerations:
- Profitability Score: {:.2}
- Growth Score: {:.2}
- Valuation Score: {:.2} (negative = expensive, positive = cheap)
- Financial Strength: {:.2}
- Efficiency: {:.2}

Provide practical trading advice considering position sizing, timing, and risk management.",
        symbol,
        result.fundamentals_score,
        result.trading_signal,
        result.confidence_score * 100.0,
        result.components.profitability,
        result.components.growth,
        result.components.valuation,
        result.components.financial_strength,
        result.components.efficiency
    );

    match chatgpt_service.generate_response(&prompt).await {
        Ok(response) => {
            info!("✅ Generated ChatGPT trading suggestion for {}", symbol);
            Some(response)
        }
        Err(e) => {
            error!("❌ Failed to generate ChatGPT trading suggestion for {}: {}", symbol, e);
            None
        }
    }
}
