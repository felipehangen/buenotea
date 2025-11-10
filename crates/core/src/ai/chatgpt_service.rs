// ChatGPT integration for generating AI explanations and insights
// Generic service that can be used by any study module

use crate::error::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{info, error};

/// ChatGPT API response structure
#[derive(Debug, Serialize, Deserialize)]
struct ChatGPTResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    content: String,
}

/// ChatGPT service for generating AI explanations
pub struct ChatGPTService {
    client: Client,
    api_key: String,
}

impl ChatGPTService {
    /// Create a new ChatGPT service
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    /// Create from environment variable
    pub fn from_env() -> Result<Self> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| crate::error::Error::MissingApiKey("OPENAI_API_KEY".to_string()))?;
        Ok(Self::new(api_key))
    }

    /// Generate a generic response from ChatGPT
    pub async fn generate_response(&self, prompt: &str) -> Result<String> {
        info!("Generating ChatGPT response");
        
        match self.call_chatgpt_api(prompt).await {
            Ok(response) => {
                info!("✅ Generated ChatGPT response successfully");
                Ok(response)
            }
            Err(e) => {
                error!("❌ Failed to generate ChatGPT response: {}", e);
                Err(e)
            }
        }
    }

    /// Call ChatGPT API with a prompt
    async fn call_chatgpt_api(&self, prompt: &str) -> Result<String> {
        let url = "https://api.openai.com/v1/chat/completions";
        
        let request_body = json!({
            "model": "gpt-3.5-turbo",
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "max_tokens": 500,
            "temperature": 0.7
        });

        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(crate::error::Error::ApiError(
                "ChatGPT".to_string(),
                format!("API call failed with status {}: {}", status, error_text)
            ));
        }

        let chatgpt_response: ChatGPTResponse = response.json().await?;
        
        if let Some(choice) = chatgpt_response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(crate::error::Error::ApiError(
                "ChatGPT".to_string(),
                "No response choices returned from ChatGPT API".to_string()
            ))
        }
    }
}

impl Default for ChatGPTService {
    fn default() -> Self {
        let api_key = std::env::var("OPENAI_API_KEY").unwrap_or_default();
        Self::new(api_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_service() {
        let service = ChatGPTService::new("test_key".to_string());
        assert_eq!(service.api_key, "test_key");
    }

    #[test]
    fn test_default() {
        let _service = ChatGPTService::default();
        // Should not panic
    }
}
