// Lambda handler for batch sentiment analysis
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request { symbols: Vec<String> }

#[derive(Serialize)]
struct Response { message: String, processed: usize }

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    Ok(Response { message: "Sentiment batch analysis".to_string(), processed: event.payload.symbols.len() })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}
