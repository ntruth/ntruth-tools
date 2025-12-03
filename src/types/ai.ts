// AI types
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
  data: string | ArrayBuffer
}

export interface AIConversation {
  id: string
  title: string
  messages: AIMessage[]
  createdAt: number
  updatedAt: number
}

export interface AIState {
  conversations: AIConversation[]
  currentConversationId?: string
  streaming: boolean
  currentResponse?: string
}

export interface AIProvider {
  name: string
  apiUrl: string
  apiKey: string
  models: string[]
}
