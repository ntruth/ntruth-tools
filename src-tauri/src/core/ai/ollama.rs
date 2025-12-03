// Ollama API client implementation (local LLM)

use super::{AIMessage, AIProvider, AIProviderConfig};
use crate::app::error::{AppError, AppResult};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use futures_util::StreamExt;

pub struct OllamaClient {
    http_client: Client,
}

impl OllamaClient {
    pub fn new(http_client: Client) -> Self {
        Self { http_client }
    }
}

#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<OllamaOptions>,
}

#[derive(Debug, Serialize)]
struct OllamaOptions {
    temperature: f32,
    num_predict: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaMessage {
    role: String,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    images: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    message: OllamaResponseMessage,
}

#[derive(Debug, Deserialize)]
struct OllamaResponseMessage {
    content: String,
}

#[derive(Debug, Deserialize)]
struct OllamaStreamResponse {
    message: Option<OllamaStreamMessage>,
    done: bool,
}

#[derive(Debug, Deserialize)]
struct OllamaStreamMessage {
    content: String,
}

#[derive(Debug, Deserialize)]
struct OllamaModelsResponse {
    models: Vec<OllamaModel>,
}

#[derive(Debug, Deserialize)]
struct OllamaModel {
    name: String,
}

fn convert_messages(messages: Vec<AIMessage>) -> Vec<OllamaMessage> {
    messages
        .into_iter()
        .map(|msg| {
            let images = msg.attachments.and_then(|attachments| {
                let image_data: Vec<String> = attachments
                    .into_iter()
                    .filter(|a| a.attachment_type == "image")
                    .map(|a| a.data)
                    .collect();
                if image_data.is_empty() {
                    None
                } else {
                    Some(image_data)
                }
            });

            OllamaMessage {
                role: msg.role,
                content: msg.content,
                images,
            }
        })
        .collect()
}

#[async_trait::async_trait]
impl AIProvider for OllamaClient {
    async fn chat(
        &self,
        messages: Vec<AIMessage>,
        config: &AIProviderConfig,
    ) -> AppResult<String> {
        let api_url = if config.api_url.is_empty() {
            "http://localhost:11434/api/chat".to_string()
        } else {
            format!("{}/api/chat", config.api_url.trim_end_matches('/'))
        };

        let request = OllamaRequest {
            model: config.model.clone(),
            messages: convert_messages(messages),
            stream: false,
            options: Some(OllamaOptions {
                temperature: config.temperature,
                num_predict: config.max_tokens,
            }),
        };

        let response = self
            .http_client
            .post(&api_url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::Network(format!("Failed to connect to Ollama: {}. Make sure Ollama is running.", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Api(format!("Ollama API error: {}", error_text)));
        }

        let result: OllamaResponse = response
            .json()
            .await
            .map_err(|e| AppError::Parse(e.to_string()))?;

        Ok(result.message.content)
    }

    async fn chat_stream(
        &self,
        messages: Vec<AIMessage>,
        config: &AIProviderConfig,
        on_chunk: mpsc::Sender<String>,
    ) -> AppResult<()> {
        let api_url = if config.api_url.is_empty() {
            "http://localhost:11434/api/chat".to_string()
        } else {
            format!("{}/api/chat", config.api_url.trim_end_matches('/'))
        };

        let request = OllamaRequest {
            model: config.model.clone(),
            messages: convert_messages(messages),
            stream: true,
            options: Some(OllamaOptions {
                temperature: config.temperature,
                num_predict: config.max_tokens,
            }),
        };

        let response = self
            .http_client
            .post(&api_url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::Network(format!("Failed to connect to Ollama: {}. Make sure Ollama is running.", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Api(format!("Ollama API error: {}", error_text)));
        }

        let mut stream = response.bytes_stream();

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.map_err(|e| AppError::Network(e.to_string()))?;
            let text = String::from_utf8_lossy(&chunk);

            for line in text.lines() {
                if !line.is_empty() {
                    if let Ok(response) = serde_json::from_str::<OllamaStreamResponse>(line) {
                        if let Some(message) = response.message {
                            if !message.content.is_empty() {
                                let _ = on_chunk.send(message.content).await;
                            }
                        }
                        if response.done {
                            break;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn list_models(&self, config: &AIProviderConfig) -> AppResult<Vec<String>> {
        let api_url = if config.api_url.is_empty() {
            "http://localhost:11434/api/tags".to_string()
        } else {
            format!("{}/api/tags", config.api_url.trim_end_matches('/'))
        };

        let response = self
            .http_client
            .get(&api_url)
            .send()
            .await
            .map_err(|e| AppError::Network(format!("Failed to connect to Ollama: {}. Make sure Ollama is running.", e)))?;

        if !response.status().is_success() {
            // Return common models if Ollama is not running
            return Ok(vec![
                "llama3.2".to_string(),
                "llama3.1".to_string(),
                "mistral".to_string(),
                "codellama".to_string(),
                "gemma2".to_string(),
            ]);
        }

        let result: OllamaModelsResponse = response
            .json()
            .await
            .map_err(|e| AppError::Parse(e.to_string()))?;

        let models: Vec<String> = result.models.into_iter().map(|m| m.name).collect();
        Ok(models)
    }
}
