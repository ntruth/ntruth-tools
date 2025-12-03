// AI client module - Multi-provider support for OpenAI, Anthropic, Ollama

use crate::app::error::{AppError, AppResult};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

mod openai;
mod anthropic;
mod ollama;

pub use openai::OpenAIClient;
pub use anthropic::AnthropicClient;
pub use ollama::OllamaClient;

/// AI Message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMessage {
    pub id: String,
    pub role: String, // "user", "assistant", "system"
    pub content: String,
    pub timestamp: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AIAttachment>>,
}

/// AI Attachment (image, file, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIAttachment {
    #[serde(rename = "type")]
    pub attachment_type: String, // "image", "file"
    pub name: String,
    pub data: String, // base64 encoded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

/// AI Conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConversation {
    pub id: String,
    pub title: String,
    pub messages: Vec<AIMessage>,
    pub created_at: i64,
    pub updated_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
}

/// Preset Prompt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetPrompt {
    pub id: String,
    pub name: String,
    pub prompt: String,
    pub description: Option<String>,
    pub category: Option<String>,
}

/// AI Provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIProviderConfig {
    pub provider: String,
    pub api_key: String,
    pub api_url: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

/// Streaming chunk callback type
pub type StreamCallback = Box<dyn Fn(String) + Send + Sync>;

/// AI Provider trait - implement for each provider
#[async_trait::async_trait]
pub trait AIProvider: Send + Sync {
    /// Send a chat message and get a response
    async fn chat(
        &self,
        messages: Vec<AIMessage>,
        config: &AIProviderConfig,
    ) -> AppResult<String>;

    /// Send a chat message with streaming response
    async fn chat_stream(
        &self,
        messages: Vec<AIMessage>,
        config: &AIProviderConfig,
        on_chunk: tokio::sync::mpsc::Sender<String>,
    ) -> AppResult<()>;

    /// Get available models
    async fn list_models(&self, config: &AIProviderConfig) -> AppResult<Vec<String>>;
}

/// AI Client - manages conversations and provider interactions
pub struct AIClient {
    http_client: Client,
    conversations: Arc<RwLock<HashMap<String, AIConversation>>>,
    preset_prompts: Arc<RwLock<Vec<PresetPrompt>>>,
}

impl AIClient {
    pub fn new() -> Self {
        Self {
            http_client: Client::new(),
            conversations: Arc::new(RwLock::new(HashMap::new())),
            preset_prompts: Arc::new(RwLock::new(Self::default_prompts())),
        }
    }

    /// Get the appropriate provider client
    pub fn get_provider(&self, provider_name: &str) -> Box<dyn AIProvider> {
        match provider_name.to_lowercase().as_str() {
            "openai" => Box::new(OpenAIClient::new(self.http_client.clone())),
            "anthropic" => Box::new(AnthropicClient::new(self.http_client.clone())),
            "ollama" => Box::new(OllamaClient::new(self.http_client.clone())),
            _ => Box::new(OpenAIClient::new(self.http_client.clone())), // Default to OpenAI
        }
    }

    /// Create a new conversation
    pub async fn create_conversation(&self, title: Option<String>, system_prompt: Option<String>) -> AIConversation {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();
        
        let conversation = AIConversation {
            id: id.clone(),
            title: title.unwrap_or_else(|| "New Conversation".to_string()),
            messages: Vec::new(),
            created_at: now,
            updated_at: now,
            system_prompt,
        };

        let mut conversations = self.conversations.write().await;
        conversations.insert(id, conversation.clone());
        
        conversation
    }

    /// Get a conversation by ID
    pub async fn get_conversation(&self, id: &str) -> Option<AIConversation> {
        let conversations = self.conversations.read().await;
        conversations.get(id).cloned()
    }

    /// Get all conversations
    pub async fn get_all_conversations(&self) -> Vec<AIConversation> {
        let conversations = self.conversations.read().await;
        let mut list: Vec<_> = conversations.values().cloned().collect();
        list.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        list
    }

    /// Add a message to a conversation
    pub async fn add_message(&self, conversation_id: &str, message: AIMessage) -> AppResult<()> {
        let mut conversations = self.conversations.write().await;
        
        if let Some(conv) = conversations.get_mut(conversation_id) {
            conv.messages.push(message);
            conv.updated_at = chrono::Utc::now().timestamp();
            
            // Auto-generate title from first user message
            if conv.title == "New Conversation" {
                if let Some(first_user_msg) = conv.messages.iter().find(|m| m.role == "user") {
                    let title = first_user_msg.content.chars().take(30).collect::<String>();
                    conv.title = if first_user_msg.content.len() > 30 {
                        format!("{}...", title)
                    } else {
                        title
                    };
                }
            }
            
            Ok(())
        } else {
            Err(AppError::NotFound("Conversation not found".to_string()))
        }
    }

    /// Delete a conversation
    pub async fn delete_conversation(&self, id: &str) -> AppResult<()> {
        let mut conversations = self.conversations.write().await;
        conversations.remove(id);
        Ok(())
    }

    /// Clear all conversations
    pub async fn clear_conversations(&self) {
        let mut conversations = self.conversations.write().await;
        conversations.clear();
    }

    /// Send a chat message
    pub async fn chat(
        &self,
        conversation_id: &str,
        user_message: String,
        attachments: Option<Vec<AIAttachment>>,
        config: &AIProviderConfig,
    ) -> AppResult<AIMessage> {
        // Create user message
        let user_msg = AIMessage {
            id: uuid::Uuid::new_v4().to_string(),
            role: "user".to_string(),
            content: user_message,
            timestamp: chrono::Utc::now().timestamp(),
            attachments,
        };

        // Add user message to conversation
        self.add_message(conversation_id, user_msg.clone()).await?;

        // Get conversation messages
        let conversation = self.get_conversation(conversation_id).await
            .ok_or_else(|| AppError::NotFound("Conversation not found".to_string()))?;

        // Build messages array with system prompt
        let mut messages = Vec::new();
        if let Some(system_prompt) = &conversation.system_prompt {
            messages.push(AIMessage {
                id: "system".to_string(),
                role: "system".to_string(),
                content: system_prompt.clone(),
                timestamp: 0,
                attachments: None,
            });
        }
        messages.extend(conversation.messages);

        // Get provider and send request
        let provider = self.get_provider(&config.provider);
        let response_content = provider.chat(messages, config).await?;

        // Create assistant message
        let assistant_msg = AIMessage {
            id: uuid::Uuid::new_v4().to_string(),
            role: "assistant".to_string(),
            content: response_content,
            timestamp: chrono::Utc::now().timestamp(),
            attachments: None,
        };

        // Add assistant message to conversation
        self.add_message(conversation_id, assistant_msg.clone()).await?;

        Ok(assistant_msg)
    }

    /// Get preset prompts
    pub async fn get_preset_prompts(&self) -> Vec<PresetPrompt> {
        let prompts = self.preset_prompts.read().await;
        prompts.clone()
    }

    /// Add a preset prompt
    pub async fn add_preset_prompt(&self, prompt: PresetPrompt) {
        let mut prompts = self.preset_prompts.write().await;
        prompts.push(prompt);
    }

    /// Delete a preset prompt
    pub async fn delete_preset_prompt(&self, id: &str) {
        let mut prompts = self.preset_prompts.write().await;
        prompts.retain(|p| p.id != id);
    }

    /// Default preset prompts
    fn default_prompts() -> Vec<PresetPrompt> {
        vec![
            PresetPrompt {
                id: "translate".to_string(),
                name: "Translator".to_string(),
                prompt: "You are a professional translator. Translate the following text accurately while maintaining the original tone and style.".to_string(),
                description: Some("Translate text between languages".to_string()),
                category: Some("Writing".to_string()),
            },
            PresetPrompt {
                id: "code-review".to_string(),
                name: "Code Reviewer".to_string(),
                prompt: "You are an expert code reviewer. Analyze the following code for bugs, security issues, performance problems, and suggest improvements.".to_string(),
                description: Some("Review and improve code".to_string()),
                category: Some("Programming".to_string()),
            },
            PresetPrompt {
                id: "summarize".to_string(),
                name: "Summarizer".to_string(),
                prompt: "Summarize the following text concisely, highlighting the key points and main ideas.".to_string(),
                description: Some("Summarize long texts".to_string()),
                category: Some("Writing".to_string()),
            },
            PresetPrompt {
                id: "explain".to_string(),
                name: "Explainer".to_string(),
                prompt: "Explain the following concept in simple terms that anyone can understand. Use examples if helpful.".to_string(),
                description: Some("Explain complex topics simply".to_string()),
                category: Some("Learning".to_string()),
            },
            PresetPrompt {
                id: "brainstorm".to_string(),
                name: "Brainstormer".to_string(),
                prompt: "Help me brainstorm ideas about the following topic. Be creative and think outside the box.".to_string(),
                description: Some("Generate creative ideas".to_string()),
                category: Some("Creativity".to_string()),
            },
        ]
    }
}

impl Default for AIClient {
    fn default() -> Self {
        Self::new()
    }
}
