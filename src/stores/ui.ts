import { createStore } from 'solid-js/store'

interface UIState {
  mainWindowVisible: boolean
  clipboardWindowVisible: boolean
  settingsWindowVisible: boolean
  aiChatWindowVisible: boolean
}

const [uiStore, setUIStore] = createStore<UIState>({
  mainWindowVisible: false,
  clipboardWindowVisible: false,
  settingsWindowVisible: false,
  aiChatWindowVisible: false,
})

export const useUIStore = () => {
  const setMainWindowVisible = (visible: boolean) => {
    setUIStore('mainWindowVisible', visible)
  }

  const setClipboardWindowVisible = (visible: boolean) => {
    setUIStore('clipboardWindowVisible', visible)
  }

  const setSettingsWindowVisible = (visible: boolean) => {
    setUIStore('settingsWindowVisible', visible)
  }

  const setAIChatWindowVisible = (visible: boolean) => {
    setUIStore('aiChatWindowVisible', visible)
  }

  return {
    store: uiStore,
    setMainWindowVisible,
    setClipboardWindowVisible,
    setSettingsWindowVisible,
    setAIChatWindowVisible,
  }
}

export { uiStore }
