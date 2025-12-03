// OpenAI API client implementation

use super::{AIMessage, AIProvider, AIProviderConfig};
use crate::app::error::{AppError, AppResult};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use futures_util::StreamExt;

pub struct OpenAIClient {
    http_client: Client,
}

impl OpenAIClient {
    pub fn new(http_client: Client) -> Self {
        Self { http_client }
    }
}

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    temperature: f32,
    max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIMessage {
    role: String,
    content: OpenAIContent,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum OpenAIContent {
    Text(String),
    Parts(Vec<OpenAIContentPart>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum OpenAIContentPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrl },
}

#[derive(Debug, Serialize, Deserialize)]
struct ImageUrl {
    url: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: OpenAIResponseMessage,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponseMessage {
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIStreamResponse {
    choices: Vec<OpenAIStreamChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAIStreamChoice {
    delta: OpenAIDelta,
}

#[derive(Debug, Deserialize)]
struct OpenAIDelta {
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAIModelsResponse {
    data: Vec<OpenAIModel>,
}

#[derive(Debug, Deserialize)]
struct OpenAIModel {
    id: String,
}

fn convert_messages(messages: Vec<AIMessage>) -> Vec<OpenAIMessage> {
    messages
        .into_iter()
        .map(|msg| {
            let content = if let Some(attachments) = msg.attachments {
                let mut parts = vec![OpenAIContentPart::Text { text: msg.content }];
                for attachment in attachments {
                    if attachment.attachment_type == "image" {
                        let mime = attachment.mime_type.unwrap_or_else(|| "image/png".to_string());
                        parts.push(OpenAIContentPart::ImageUrl {
                            image_url: ImageUrl {
                                url: format!("data:{};base64,{}", mime, attachment.data),
                            },
                        });
                    }
                }
                OpenAIContent::Parts(parts)
            } else {
                OpenAIContent::Text(msg.content)
            };

            OpenAIMessage {
                role: msg.role,
                content,
            }
        })
        .collect()
}

#[async_trait::async_trait]
impl AIProvider for OpenAIClient {
    async fn chat(
        &self,
        messages: Vec<AIMessage>,
        config: &AIProviderConfig,
    ) -> AppResult<String> {
        let api_url = if config.api_url.is_empty() {
            "https://api.openai.com/v1/chat/completions".to_string()
        } else {
            format!("{}/chat/completions", config.api_url.trim_end_matches('/'))
        };

        let request = OpenAIRequest {
            model: config.model.clone(),
            messages: convert_messages(messages),
            temperature: config.temperature,
            max_tokens: config.max_tokens,
            stream: None,
        };

        let response = self
            .http_client
            .post(&api_url)
            .header("Authorization", format!("Bearer {}", config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::Network(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Api(format!("OpenAI API error: {}", error_text)));
        }

        let result: OpenAIResponse = response
            .json()
            .await
            .map_err(|e| AppError::Parse(e.to_string()))?;

        result
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| AppError::Api("No response from OpenAI".to_string()))
    }

    async fn chat_stream(
        &self,
        messages: Vec<AIMessage>,
        config: &AIProviderConfig,
        on_chunk: mpsc::Sender<String>,
    ) -> AppResult<()> {
        let api_url = if config.api_url.is_empty() {
            "https://api.openai.com/v1/chat/completions".to_string()
        } else {
            format!("{}/chat/completions", config.api_url.trim_end_matches('/'))
        };

        let request = OpenAIRequest {
            model: config.model.clone(),
            messages: convert_messages(messages),
            temperature: config.temperature,
            max_tokens: config.max_tokens,
            stream: Some(true),
        };

        let response = self
            .http_client
            .post(&api_url)
            .header("Authorization", format!("Bearer {}", config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::Network(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Api(format!("OpenAI API error: {}", error_text)));
        }

        let mut stream = response.bytes_stream();

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.map_err(|e| AppError::Network(e.to_string()))?;
            let text = String::from_utf8_lossy(&chunk);

            for line in text.lines() {
                if line.starts_with("data: ") {
                    let data = &line[6..];
                    if data == "[DONE]" {
                        break;
                    }

                    if let Ok(response) = serde_json::from_str::<OpenAIStreamResponse>(data) {
                        if let Some(choice) = response.choices.first() {
                            if let Some(content) = &choice.delta.content {
                                let _ = on_chunk.send(content.clone()).await;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn list_models(&self, config: &AIProviderConfig) -> AppResult<Vec<String>> {
        let api_url = if config.api_url.is_empty() {
            "https://api.openai.com/v1/models".to_string()
        } else {
            format!("{}/models", config.api_url.trim_end_matches('/'))
        };

        let response = self
            .http_client
            .get(&api_url)
            .header("Authorization", format!("Bearer {}", config.api_key))
            .send()
            .await
            .map_err(|e| AppError::Network(e.to_string()))?;

        if !response.status().is_success() {
            // Return default models if API call fails
            return Ok(vec![
                "gpt-4o".to_string(),
                "gpt-4o-mini".to_string(),
                "gpt-4-turbo".to_string(),
                "gpt-4".to_string(),
                "gpt-3.5-turbo".to_string(),
            ]);
        }

        let result: OpenAIModelsResponse = response
            .json()
            .await
            .map_err(|e| AppError::Parse(e.to_string()))?;

        let models: Vec<String> = result
            .data
            .into_iter()
            .filter(|m| m.id.starts_with("gpt"))
            .map(|m| m.id)
            .collect();

        Ok(models)
    }
}
