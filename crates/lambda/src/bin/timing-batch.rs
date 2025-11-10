// Lambda handler for batch timing analysis

use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use buenotea_timing::TTSCalculator;
use buenotea_infrastructure::DatabaseClient;
use tracing::info;

#[derive(Deserialize)]
struct Request {
    symbols: Vec<String>,
}

#[derive(Serialize)]
struct Response {
    message: String,
    processed: usize,
    errors: usize,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    info!("Processing timing analysis for {} symbols", event.payload.symbols.len());
    
    let mut calculator = TTSCalculator::new();
    let db_client = DatabaseClient::from_env()?;
    let storage = buenotea_timing::TimingStorage::new(db_client);
    
    let mut processed = 0;
    let mut errors = 0;
    
    for symbol in &event.payload.symbols {
        match calculator.calculate_tts_with_tracking(symbol).await {
            Ok((result, _tracking)) => {
                if let Err(e) = storage.upsert_timing_analysis(&result).await {
                    tracing::error!("Failed to save {}: {}", symbol, e);
                    errors += 1;
                } else {
                    processed += 1;
                }
            }
            Err(e) => {
                tracing::error!("Failed to analyze {}: {}", symbol, e);
                errors += 1;
            }
        }
    }
    
    Ok(Response {
        message: format!("Processed {} symbols", processed),
        processed,
        errors,
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}

