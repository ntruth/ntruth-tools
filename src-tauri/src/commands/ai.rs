use crate::app::{error::AppResult, state::AppState};
use crate::core::ai::{AIAttachment, AIClient, AIConversation, AIMessage, AIProviderConfig, PresetPrompt};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, State};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Shared AI client state
pub struct AIState {
    pub client: Arc<RwLock<AIClient>>,
}

impl AIState {
    pub fn new() -> Self {
        Self {
            client: Arc::new(RwLock::new(AIClient::new())),
        }
    }
}

impl Default for AIState {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a new conversation
#[tauri::command]
pub async fn ai_create_conversation(
    title: Option<String>,
    system_prompt: Option<String>,
    ai_state: State<'_, AIState>,
) -> AppResult<AIConversation> {
    let client = ai_state.client.read().await;
    Ok(client.create_conversation(title, system_prompt).await)
}

/// Get a conversation by ID
#[tauri::command]
pub async fn ai_get_conversation(
    id: String,
    ai_state: State<'_, AIState>,
) -> AppResult<Option<AIConversation>> {
    let client = ai_state.client.read().await;
    Ok(client.get_conversation(&id).await)
}

/// Get all conversations
#[tauri::command]
pub async fn ai_get_conversations(
    ai_state: State<'_, AIState>,
) -> AppResult<Vec<AIConversation>> {
    let client = ai_state.client.read().await;
    Ok(client.get_all_conversations().await)
}

/// Delete a conversation
#[tauri::command]
pub async fn ai_delete_conversation(
    id: String,
    ai_state: State<'_, AIState>,
) -> AppResult<()> {
    let client = ai_state.client.read().await;
    client.delete_conversation(&id).await
}

/// Clear all conversations
#[tauri::command]
pub async fn ai_clear_conversations(
    ai_state: State<'_, AIState>,
) -> AppResult<()> {
    let client = ai_state.client.read().await;
    client.clear_conversations().await;
    Ok(())
}

/// Send a chat message (non-streaming)
#[tauri::command]
pub async fn ai_chat(
    conversation_id: String,
    message: String,
    attachments: Option<Vec<AIAttachment>>,
    state: State<'_, AppState>,
    ai_state: State<'_, AIState>,
) -> AppResult<AIMessage> {
    let config = state.config.read().await;
    let provider_config = AIProviderConfig {
        provider: config.ai.provider.clone(),
        api_key: config.ai.api_key.clone(),
        api_url: config.ai.api_url.clone(),
        model: config.ai.model.clone(),
        temperature: config.ai.temperature,
        max_tokens: config.ai.max_tokens,
    };
    drop(config);

    let client = ai_state.client.read().await;
    client.chat(&conversation_id, message, attachments, &provider_config).await
}

/// Send a chat message with streaming response
#[tauri::command]
pub async fn ai_chat_stream(
    conversation_id: String,
    message: String,
    attachments: Option<Vec<AIAttachment>>,
    app: AppHandle,
    state: State<'_, AppState>,
    ai_state: State<'_, AIState>,
) -> AppResult<String> {
    let config = state.config.read().await;
    let provider_config = AIProviderConfig {
        provider: config.ai.provider.clone(),
        api_key: config.ai.api_key.clone(),
        api_url: config.ai.api_url.clone(),
        model: config.ai.model.clone(),
        temperature: config.ai.temperature,
        max_tokens: config.ai.max_tokens,
    };
    drop(config);

    let client = ai_state.client.read().await;
    
    // Create user message
    let user_msg = AIMessage {
        id: uuid::Uuid::new_v4().to_string(),
        role: "user".to_string(),
        content: message.clone(),
        timestamp: chrono::Utc::now().timestamp(),
        attachments: attachments.clone(),
    };
    client.add_message(&conversation_id, user_msg).await?;

    // Get conversation and build messages
    let conversation = client.get_conversation(&conversation_id).await
        .ok_or_else(|| crate::app::error::AppError::NotFound("Conversation not found".to_string()))?;

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

    // Create channel for streaming
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(100);
    let assistant_msg_id = uuid::Uuid::new_v4().to_string();

    // Spawn task to handle streaming
    let provider = client.get_provider(&provider_config.provider);
    let stream_config = provider_config.clone();
    let stream_app = app.clone();
    let stream_msg_id = assistant_msg_id.clone();

    tokio::spawn(async move {
        let mut full_response = String::new();
        
        // Start streaming
        let _ = stream_app.emit("ai-stream-start", &stream_msg_id);
        
        if let Err(e) = provider.chat_stream(messages, &stream_config, tx).await {
            let _ = stream_app.emit("ai-stream-error", e.to_string());
            return;
        }

        while let Some(chunk) = rx.recv().await {
            full_response.push_str(&chunk);
            let _ = stream_app.emit("ai-stream-chunk", serde_json::json!({
                "id": stream_msg_id,
                "chunk": chunk,
            }));
        }

        let _ = stream_app.emit("ai-stream-end", serde_json::json!({
            "id": stream_msg_id,
            "content": full_response,
        }));
    });

    Ok(assistant_msg_id)
}

/// Save the assistant response after streaming completes
#[tauri::command]
pub async fn ai_save_response(
    conversation_id: String,
    message_id: String,
    content: String,
    ai_state: State<'_, AIState>,
) -> AppResult<AIMessage> {
    let client = ai_state.client.read().await;
    
    let assistant_msg = AIMessage {
        id: message_id,
        role: "assistant".to_string(),
        content,
        timestamp: chrono::Utc::now().timestamp(),
        attachments: None,
    };

    client.add_message(&conversation_id, assistant_msg.clone()).await?;
    Ok(assistant_msg)
}

/// Get preset prompts
#[tauri::command]
pub async fn ai_get_presets(
    ai_state: State<'_, AIState>,
) -> AppResult<Vec<PresetPrompt>> {
    let client = ai_state.client.read().await;
    Ok(client.get_preset_prompts().await)
}

/// Add a preset prompt
#[tauri::command]
pub async fn ai_add_preset(
    name: String,
    prompt: String,
    description: Option<String>,
    category: Option<String>,
    ai_state: State<'_, AIState>,
) -> AppResult<PresetPrompt> {
    let preset = PresetPrompt {
        id: uuid::Uuid::new_v4().to_string(),
        name,
        prompt,
        description,
        category,
    };

    let client = ai_state.client.read().await;
    client.add_preset_prompt(preset.clone()).await;
    Ok(preset)
}

/// Delete a preset prompt
#[tauri::command]
pub async fn ai_delete_preset(
    id: String,
    ai_state: State<'_, AIState>,
) -> AppResult<()> {
    let client = ai_state.client.read().await;
    client.delete_preset_prompt(&id).await;
    Ok(())
}

/// Get available models for a provider
#[tauri::command]
pub async fn ai_get_models(
    provider: Option<String>,
    state: State<'_, AppState>,
    ai_state: State<'_, AIState>,
) -> AppResult<Vec<String>> {
    let config = state.config.read().await;
    let provider_name = provider.unwrap_or_else(|| config.ai.provider.clone());
    let provider_config = AIProviderConfig {
        provider: provider_name.clone(),
        api_key: config.ai.api_key.clone(),
        api_url: config.ai.api_url.clone(),
        model: config.ai.model.clone(),
        temperature: config.ai.temperature,
        max_tokens: config.ai.max_tokens,
    };
    drop(config);

    let client = ai_state.client.read().await;
    let provider = client.get_provider(&provider_name);
    provider.list_models(&provider_config).await
}

// Legacy commands for backward compatibility
#[tauri::command]
pub async fn get_ai_conversations(
    ai_state: State<'_, AIState>,
) -> AppResult<Vec<AIConversation>> {
    ai_get_conversations(ai_state).await
}
