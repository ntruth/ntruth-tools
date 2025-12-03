import { invoke } from '@tauri-apps/api/core'

// Tauri API wrapper service
export const tauriService = {
  // Invoke a Tauri command
  async invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
    return invoke<T>(cmd, args)
  },
}

// Search commands
export const searchCommands = {
  async search(query: string) {
    return tauriService.invoke('search', { query })
  },
  
  async calculate(expression: string) {
    return tauriService.invoke('calculate', { expression })
  },
}

// Clipboard commands
export const clipboardCommands = {
  async getHistory() {
    return tauriService.invoke('get_clipboard_history')
  },
  
  async pasteItem(id: string) {
    return tauriService.invoke('paste_clipboard_item', { id })
  },
}

// AI commands
export const aiCommands = {
  async chat(message: string, conversationId?: string) {
    return tauriService.invoke('ai_chat', { message, conversationId })
  },
  
  async getConversations() {
    return tauriService.invoke('get_ai_conversations')
  },
}

// Settings commands
export const settingsCommands = {
  async getConfig() {
    return tauriService.invoke('get_config')
  },
  
  async updateConfig(config: Record<string, unknown>) {
    return tauriService.invoke('update_config', { config })
  },
}

// System commands
export const systemCommands = {
  async openPath(path: string) {
    return tauriService.invoke('open_path', { path })
  },
  
  async openUrl(url: string) {
    return tauriService.invoke('open_url', { url })
  },
  
  async showWindow(label: string) {
    return tauriService.invoke('show_window', { label })
  },
  
  async hideWindow(label: string) {
    return tauriService.invoke('hide_window', { label })
  },
  
  async toggleMainWindow() {
    return tauriService.invoke('toggle_main_window')
  },
}

// Clipboard commands
export const clipboardWindowCommands = {
  async show() {
    return tauriService.invoke('show_clipboard_window')
  },
  
  async hide() {
    return tauriService.invoke('hide_clipboard_window')
  },
  
  async toggleFavorite(id: string) {
    return tauriService.invoke('toggle_clipboard_favorite', { id })
  },
  
  async deleteItem(id: string) {
    return tauriService.invoke('delete_clipboard_item', { id })
  },
}
