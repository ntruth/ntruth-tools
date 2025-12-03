import { Component, Show } from 'solid-js'
import { User, Bot } from 'lucide-solid'

interface AIMessage {
  id: string
  role: 'user' | 'assistant' | 'system'
  content: string
  timestamp: number
  attachments?: Array<{
    type: 'image' | 'file'
    name: string
    data: string
    mime_type?: string
  }>
}

interface ChatMessageProps {
  message: AIMessage
  isStreaming?: boolean
}

const ChatMessage: Component<ChatMessageProps> = (props) => {
  const isUser = () => props.message.role === 'user'

  const formatTime = (timestamp: number) => {
    return new Date(timestamp).toLocaleTimeString([], {
      hour: '2-digit',
      minute: '2-digit',
    })
  }

  // Simple markdown-like rendering
  const renderContent = (content: string) => {
    // Handle code blocks
    const parts = content.split(/(```[\s\S]*?```)/g)
    
    return parts.map((part) => {
      if (part.startsWith('```') && part.endsWith('```')) {
        const code = part.slice(3, -3)
        const lines = code.split('\n')
        const codeContent = lines.slice(1).join('\n') || code
        
        return (
          <pre class="my-2 rounded-lg bg-gray-800 p-3 overflow-x-auto">
            <code class="text-sm text-gray-100">{codeContent}</code>
          </pre>
        )
      }
      
      // Handle inline code
      const inlineCodeParts = part.split(/(`[^`]+`)/g)
      return inlineCodeParts.map((p) => {
        if (p.startsWith('`') && p.endsWith('`')) {
          return (
            <code class="rounded bg-gray-200 px-1 py-0.5 text-sm dark:bg-gray-700">
              {p.slice(1, -1)}
            </code>
          )
        }
        return <span>{p}</span>
      })
    })
  }

  return (
    <div
      class={`flex gap-3 ${isUser() ? 'flex-row-reverse' : ''}`}
    >
      {/* Avatar */}
      <div
        class={`flex h-8 w-8 shrink-0 items-center justify-center rounded-full ${
          isUser()
            ? 'bg-blue-500 text-white'
            : 'bg-gray-200 text-gray-600 dark:bg-gray-700 dark:text-gray-300'
        }`}
      >
        {isUser() ? <User size={16} /> : <Bot size={16} />}
      </div>

      {/* Message Content */}
      <div
        class={`max-w-[70%] rounded-lg px-4 py-2 ${
          isUser()
            ? 'bg-blue-500 text-white'
            : 'bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-200'
        }`}
      >
        {/* Attachments */}
        <Show when={props.message.attachments && props.message.attachments.length > 0}>
          <div class="flex flex-wrap gap-2 mb-2">
            {props.message.attachments?.map((attachment) => (
              <Show when={attachment.type === 'image'}>
                <img
                  src={`data:${attachment.mime_type || 'image/png'};base64,${attachment.data}`}
                  alt={attachment.name}
                  class="max-w-[200px] rounded-lg"
                />
              </Show>
            ))}
          </div>
        </Show>

        {/* Message Text */}
        <div class="whitespace-pre-wrap break-words">
          {renderContent(props.message.content)}
        </div>

        {/* Streaming indicator */}
        <Show when={props.isStreaming}>
          <span class="inline-block w-2 h-4 ml-1 bg-current animate-pulse" />
        </Show>

        {/* Timestamp */}
        <div
          class={`mt-1 text-xs ${
            isUser() ? 'text-blue-100' : 'text-gray-500 dark:text-gray-400'
          }`}
        >
          {formatTime(props.message.timestamp)}
        </div>
      </div>
    </div>
  )
}

export default ChatMessage
