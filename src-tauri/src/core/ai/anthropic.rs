// Anthropic Claude API client implementation

use super::{AIMessage, AIProvider, AIProviderConfig};
use crate::app::error::{AppError, AppResult};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use futures_util::StreamExt;

pub struct AnthropicClient {
    http_client: Client,
}

impl AnthropicClient {
    pub fn new(http_client: Client) -> Self {
        Self { http_client }
    }
}

#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<AnthropicMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnthropicMessage {
    role: String,
    content: AnthropicContent,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum AnthropicContent {
    Text(String),
    Parts(Vec<AnthropicContentPart>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum AnthropicContentPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image")]
    Image { source: ImageSource },
}

#[derive(Debug, Serialize, Deserialize)]
struct ImageSource {
    #[serde(rename = "type")]
    source_type: String,
    media_type: String,
    data: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicResponseContent>,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponseContent {
    #[serde(rename = "type")]
    content_type: String,
    text: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AnthropicStreamEvent {
    #[serde(rename = "type")]
    event_type: String,
    delta: Option<AnthropicDelta>,
}

#[derive(Debug, Deserialize)]
struct AnthropicDelta {
    #[serde(rename = "type")]
    delta_type: Option<String>,
    text: Option<String>,
}

fn convert_messages(messages: Vec<AIMessage>) -> (Option<String>, Vec<AnthropicMessage>) {
    let mut system_prompt = None;
    let mut converted = Vec::new();

    for msg in messages {
        if msg.role == "system" {
            system_prompt = Some(msg.content);
            continue;
        }

        let content = if let Some(attachments) = msg.attachments {
            let mut parts = vec![AnthropicContentPart::Text { text: msg.content }];
            for attachment in attachments {
                if attachment.attachment_type == "image" {
                    let media_type = attachment.mime_type.unwrap_or_else(|| "image/png".to_string());
                    parts.push(AnthropicContentPart::Image {
                        source: ImageSource {
                            source_type: "base64".to_string(),
                            media_type,
                            data: attachment.data,
                        },
                    });
                }
            }
            AnthropicContent::Parts(parts)
        } else {
            AnthropicContent::Text(msg.content)
        };

        converted.push(AnthropicMessage {
            role: msg.role,
            content,
        });
    }

    (system_prompt, converted)
}

#[async_trait::async_trait]
impl AIProvider for AnthropicClient {
    async fn chat(
        &self,
        messages: Vec<AIMessage>,
        config: &AIProviderConfig,
    ) -> AppResult<String> {
        let api_url = if config.api_url.is_empty() {
            "https://api.anthropic.com/v1/messages".to_string()
        } else {
            format!("{}/messages", config.api_url.trim_end_matches('/'))
        };

        let (system_prompt, converted_messages) = convert_messages(messages);

        let request = AnthropicRequest {
            model: config.model.clone(),
            max_tokens: config.max_tokens,
            messages: converted_messages,
            system: system_prompt,
            stream: None,
        };

        let response = self
            .http_client
            .post(&api_url)
            .header("x-api-key", &config.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::Network(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Api(format!("Anthropic API error: {}", error_text)));
        }

        let result: AnthropicResponse = response
            .json()
            .await
            .map_err(|e| AppError::Parse(e.to_string()))?;

        result
            .content
            .first()
            .and_then(|c| c.text.clone())
            .ok_or_else(|| AppError::Api("No response from Anthropic".to_string()))
    }

    async fn chat_stream(
        &self,
        messages: Vec<AIMessage>,
        config: &AIProviderConfig,
        on_chunk: mpsc::Sender<String>,
    ) -> AppResult<()> {
        let api_url = if config.api_url.is_empty() {
            "https://api.anthropic.com/v1/messages".to_string()
        } else {
            format!("{}/messages", config.api_url.trim_end_matches('/'))
        };

        let (system_prompt, converted_messages) = convert_messages(messages);

        let request = AnthropicRequest {
            model: config.model.clone(),
            max_tokens: config.max_tokens,
            messages: converted_messages,
            system: system_prompt,
            stream: Some(true),
        };

        let response = self
            .http_client
            .post(&api_url)
            .header("x-api-key", &config.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::Network(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Api(format!("Anthropic API error: {}", error_text)));
        }

        let mut stream = response.bytes_stream();

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.map_err(|e| AppError::Network(e.to_string()))?;
            let text = String::from_utf8_lossy(&chunk);

            for line in text.lines() {
                if line.starts_with("data: ") {
                    let data = &line[6..];
                    if let Ok(event) = serde_json::from_str::<AnthropicStreamEvent>(data) {
                        if event.event_type == "content_block_delta" {
                            if let Some(delta) = event.delta {
                                if let Some(text) = delta.text {
                                    let _ = on_chunk.send(text).await;
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn list_models(&self, _config: &AIProviderConfig) -> AppResult<Vec<String>> {
        // Anthropic doesn't have a models endpoint, return known models
        Ok(vec![
            "claude-3-5-sonnet-20241022".to_string(),
            "claude-3-5-haiku-20241022".to_string(),
            "claude-3-opus-20240229".to_string(),
            "claude-3-sonnet-20240229".to_string(),
            "claude-3-haiku-20240307".to_string(),
        ])
    }
}
