// Settings types
export interface SettingsPage {
  id: string
  label: string
  icon?: string
}

export const SETTINGS_PAGES: SettingsPage[] = [
  { id: 'general', label: 'General' },
  { id: 'features', label: 'Features' },
  { id: 'appearance', label: 'Appearance' },
  { id: 'clipboard', label: 'Clipboard' },
  { id: 'screenshot', label: 'Screenshot' },
  { id: 'ai', label: 'AI' },
  { id: 'web-search', label: 'Web Search' },
  { id: 'shortcuts', label: 'Shortcuts' },
  { id: 'advanced', label: 'Advanced' },
  { id: 'about', label: 'About' },
]
