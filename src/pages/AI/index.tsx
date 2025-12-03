import { Component, For, Show, createSignal, onMount, onCleanup, createEffect } from 'solid-js'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { MessageSquare, Plus, Trash2, Send, Image, Sparkles } from 'lucide-solid'
import ChatMessage from './ChatMessage'
import PresetSelector from './PresetSelector'

export interface AIMessage {
  id: string
  role: 'user' | 'assistant' | 'system'
  content: string
  timestamp: number
  attachments?: AIAttachment[]
}

export interface AIAttachment {
  type: 'image' | 'file'
  name: string
  data: string
  mime_type?: string
}

export interface AIConversation {
  id: string
  title: string
  messages: AIMessage[]
  created_at: number
  updated_at: number
  system_prompt?: string
}

export interface PresetPrompt {
  id: string
  name: string
  prompt: string
  description?: string
  category?: string
}

const AIPage: Component = () => {
  const [conversations, setConversations] = createSignal<AIConversation[]>([])
  const [currentConversation, setCurrentConversation] = createSignal<AIConversation | null>(null)
  const [inputMessage, setInputMessage] = createSignal('')
  const [isStreaming, setIsStreaming] = createSignal(false)
  const [streamingContent, setStreamingContent] = createSignal('')
  const [showPresets, setShowPresets] = createSignal(false)
  const [presets, setPresets] = createSignal<PresetPrompt[]>([])
  const [attachments, setAttachments] = createSignal<AIAttachment[]>([])
  
  let inputRef: HTMLTextAreaElement | undefined
  let messagesEndRef: HTMLDivElement | undefined
  let fileInputRef: HTMLInputElement | undefined

  onMount(async () => {
    await loadConversations()
    await loadPresets()
    
    // Listen for streaming events
    const unlistenStart = await listen('ai-stream-start', () => {
      setIsStreaming(true)
      setStreamingContent('')
    })
    
    const unlistenChunk = await listen<{ id: string; chunk: string }>('ai-stream-chunk', (event) => {
      setStreamingContent((prev) => prev + event.payload.chunk)
    })
    
    const unlistenEnd = await listen<{ id: string; content: string }>('ai-stream-end', async (event) => {
      setIsStreaming(false)
      const conv = currentConversation()
      if (conv) {
        // Save the response
        await invoke('ai_save_response', {
          conversationId: conv.id,
          messageId: event.payload.id,
          content: event.payload.content,
        })
        // Reload the conversation
        await loadConversation(conv.id)
      }
      setStreamingContent('')
    })
    
    const unlistenError = await listen<string>('ai-stream-error', (event) => {
      setIsStreaming(false)
      setStreamingContent('')
      console.error('Streaming error:', event.payload)
    })
    
    onCleanup(() => {
      unlistenStart()
      unlistenChunk()
      unlistenEnd()
      unlistenError()
    })
  })

  const loadConversations = async () => {
    try {
      const convs = await invoke<AIConversation[]>('ai_get_conversations')
      setConversations(convs)
    } catch (error) {
      console.error('Failed to load conversations:', error)
    }
  }

  const loadConversation = async (id: string) => {
    try {
      const conv = await invoke<AIConversation | null>('ai_get_conversation', { id })
      if (conv) {
        setCurrentConversation(conv)
      }
    } catch (error) {
      console.error('Failed to load conversation:', error)
    }
  }

  const loadPresets = async () => {
    try {
      const p = await invoke<PresetPrompt[]>('ai_get_presets')
      setPresets(p)
    } catch (error) {
      console.error('Failed to load presets:', error)
    }
  }

  const createNewConversation = async (systemPrompt?: string) => {
    try {
      const conv = await invoke<AIConversation>('ai_create_conversation', {
        title: null,
        systemPrompt,
      })
      setCurrentConversation(conv)
      await loadConversations()
      setShowPresets(false)
    } catch (error) {
      console.error('Failed to create conversation:', error)
    }
  }

  const deleteConversation = async (id: string) => {
    try {
      await invoke('ai_delete_conversation', { id })
      if (currentConversation()?.id === id) {
        setCurrentConversation(null)
      }
      await loadConversations()
    } catch (error) {
      console.error('Failed to delete conversation:', error)
    }
  }

  const sendMessage = async () => {
    const message = inputMessage().trim()
    const conv = currentConversation()
    
    if (!message || !conv || isStreaming()) return
    
    setInputMessage('')
    const currentAttachments = attachments()
    setAttachments([])
    
    try {
      // Use streaming API
      await invoke('ai_chat_stream', {
        conversationId: conv.id,
        message,
        attachments: currentAttachments.length > 0 ? currentAttachments : null,
      })
      
      // Immediately reload to show user message
      await loadConversation(conv.id)
    } catch (error) {
      console.error('Failed to send message:', error)
    }
  }

  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault()
      sendMessage()
    }
  }

  const handleImageUpload = (e: Event) => {
    const target = e.target as HTMLInputElement
    const files = target.files
    if (!files) return

    Array.from(files).forEach((file) => {
      const reader = new FileReader()
      reader.onload = () => {
        const base64 = (reader.result as string).split(',')[1]
        setAttachments((prev) => [
          ...prev,
          {
            type: 'image',
            name: file.name,
            data: base64,
            mime_type: file.type,
          },
        ])
      }
      reader.readAsDataURL(file)
    })
    
    target.value = ''
  }

  const removeAttachment = (index: number) => {
    setAttachments((prev) => prev.filter((_, i) => i !== index))
  }

  const scrollToBottom = () => {
    messagesEndRef?.scrollIntoView({ behavior: 'smooth' })
  }

  createEffect(() => {
    // Scroll to bottom when messages change or streaming
    currentConversation()
    streamingContent()
    scrollToBottom()
  })

  return (
    <div class="flex h-screen bg-white dark:bg-gray-900">
      {/* Sidebar - Conversations List */}
      <div class="w-64 border-r border-gray-200 bg-gray-50 flex flex-col dark:border-gray-700 dark:bg-gray-800">
        <div class="p-4 border-b border-gray-200 dark:border-gray-700">
          <button
            onClick={() => setShowPresets(true)}
            class="flex w-full items-center justify-center gap-2 rounded-lg bg-blue-500 px-4 py-2 text-white hover:bg-blue-600"
          >
            <Plus size={18} />
            New Chat
          </button>
        </div>
        
        <div class="flex-1 overflow-y-auto">
          <For each={conversations()}>
            {(conv) => (
              <div
                class={`flex items-center gap-2 px-4 py-3 cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-700 ${
                  currentConversation()?.id === conv.id ? 'bg-gray-100 dark:bg-gray-700' : ''
                }`}
                onClick={() => setCurrentConversation(conv)}
              >
                <MessageSquare size={16} class="text-gray-500 shrink-0" />
                <span class="flex-1 truncate text-sm text-gray-700 dark:text-gray-300">
                  {conv.title}
                </span>
                <button
                  onClick={(e) => {
                    e.stopPropagation()
                    deleteConversation(conv.id)
                  }}
                  class="p-1 text-gray-400 hover:text-red-500 opacity-0 group-hover:opacity-100"
                >
                  <Trash2 size={14} />
                </button>
              </div>
            )}
          </For>
        </div>
      </div>

      {/* Main Chat Area */}
      <div class="flex-1 flex flex-col">
        <Show
          when={currentConversation()}
          fallback={
            <div class="flex-1 flex items-center justify-center">
              <div class="text-center">
                <Sparkles size={48} class="mx-auto mb-4 text-gray-300" />
                <h2 class="text-xl font-semibold text-gray-700 dark:text-gray-300">
                  Start a New Conversation
                </h2>
                <p class="mt-2 text-gray-500">
                  Click "New Chat" to begin
                </p>
              </div>
            </div>
          }
        >
          {(conv) => (
            <>
              {/* Messages */}
              <div class="flex-1 overflow-y-auto p-4 space-y-4">
                <For each={conv().messages}>
                  {(message) => <ChatMessage message={message} />}
                </For>
                
                <Show when={isStreaming() && streamingContent()}>
                  <ChatMessage
                    message={{
                      id: 'streaming',
                      role: 'assistant',
                      content: streamingContent(),
                      timestamp: Date.now(),
                    }}
                    isStreaming={true}
                  />
                </Show>
                
                <div ref={messagesEndRef} />
              </div>

              {/* Input Area */}
              <div class="border-t border-gray-200 p-4 dark:border-gray-700">
                {/* Attachments Preview */}
                <Show when={attachments().length > 0}>
                  <div class="flex gap-2 mb-2 flex-wrap">
                    <For each={attachments()}>
                      {(attachment, index) => (
                        <div class="relative">
                          <img
                            src={`data:${attachment.mime_type};base64,${attachment.data}`}
                            alt={attachment.name}
                            class="h-16 w-16 object-cover rounded-lg"
                          />
                          <button
                            onClick={() => removeAttachment(index())}
                            class="absolute -top-2 -right-2 bg-red-500 text-white rounded-full p-1"
                          >
                            <Trash2 size={12} />
                          </button>
                        </div>
                      )}
                    </For>
                  </div>
                </Show>
                
                <div class="flex gap-2">
                  <input
                    ref={fileInputRef}
                    type="file"
                    accept="image/*"
                    multiple
                    class="hidden"
                    onChange={handleImageUpload}
                  />
                  <button
                    onClick={() => fileInputRef?.click()}
                    class="p-2 text-gray-500 hover:text-gray-700 dark:hover:text-gray-300"
                    title="Attach image"
                  >
                    <Image size={20} />
                  </button>
                  
                  <textarea
                    ref={inputRef}
                    value={inputMessage()}
                    onInput={(e) => setInputMessage(e.currentTarget.value)}
                    onKeyDown={handleKeyDown}
                    placeholder="Type a message... (Shift+Enter for new line)"
                    class="flex-1 resize-none rounded-lg border border-gray-300 px-4 py-2 focus:border-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white"
                    rows={1}
                    disabled={isStreaming()}
                  />
                  
                  <button
                    onClick={sendMessage}
                    disabled={!inputMessage().trim() || isStreaming()}
                    class="rounded-lg bg-blue-500 px-4 py-2 text-white hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed"
                  >
                    <Send size={20} />
                  </button>
                </div>
              </div>
            </>
          )}
        </Show>
      </div>

      {/* Preset Selector Modal */}
      <Show when={showPresets()}>
        <PresetSelector
          presets={presets()}
          onSelect={(prompt) => createNewConversation(prompt)}
          onClose={() => setShowPresets(false)}
          onNewChat={() => createNewConversation()}
        />
      </Show>
    </div>
  )
}

export default AIPage
