// Search types
export interface SearchResult {
  id: string
  type?: SearchResultType
  title: string
  subtitle?: string
  icon?: string
  path?: string
  category?: string
  score?: number
  action: SearchAction
  metadata?: Record<string, unknown>
}

export type SearchResultType = 
  | 'file'
  | 'app'
  | 'calculator'
  | 'web-search'
  | 'ai'
  | 'clipboard'
  | 'command'

export interface SearchAction {
  type: 'open' | 'copy' | 'execute' | 'web-search' | 'ai-query' | 'clipboard' | 'settings' | 'none'
  payload?: string
}

export interface SearchState {
  query: string
  results: SearchResult[]
  selectedIndex: number
  loading: boolean
}

// Calculator types
export interface CalculatorResult {
  expression: string
  result: number | string
  type: 'number' | 'unit-conversion' | 'error'
}
