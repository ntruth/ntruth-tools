// Clipboard types
export interface ClipboardItem {
  id: string
  type: ClipboardItemType
  content: string | ArrayBuffer
  plainText?: string
  source?: string
  timestamp: number
  favorite: boolean
  metadata?: ClipboardMetadata
}

export type ClipboardItemType = 'text' | 'rich-text' | 'image' | 'file' | 'color'

export interface ClipboardMetadata {
  width?: number
  height?: number
  fileSize?: number
  fileName?: string
  color?: {
    hex: string
    rgb: string
    hsl: string
  }
}

export interface ClipboardState {
  items: ClipboardItem[]
  selectedId?: string
  filter: ClipboardFilter
}

export interface ClipboardFilter {
  type?: ClipboardItemType
  search?: string
  favoriteOnly?: boolean
}
