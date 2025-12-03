import { Component, Match, Switch, onMount } from 'solid-js'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { MainPage } from './pages/Main'

const App: Component = () => {
  const currentWindow = getCurrentWindow()
  
  onMount(() => {
    // Get window label to determine which component to render
    const label = currentWindow.label
    console.log('Window label:', label)
  })

  return (
    <div class="h-screen w-full overflow-hidden">
      <Switch fallback={<MainPage />}>
        <Match when={currentWindow.label === 'main'}>
          <MainPage />
        </Match>
        <Match when={currentWindow.label === 'clipboard'}>
          <ClipboardWindow />
        </Match>
        <Match when={currentWindow.label === 'settings'}>
          <SettingsWindow />
        </Match>
        <Match when={currentWindow.label === 'ai-chat'}>
          <AIChatWindow />
        </Match>
        <Match when={currentWindow.label.startsWith('pin-')}>
          <PinWindow />
        </Match>
      </Switch>
    </div>
  )
}

// Clipboard history window
const ClipboardWindow: Component = () => {
  return (
    <div class="h-full w-full bg-white dark:bg-gray-900">
      <div class="p-4">
        <h2 class="text-lg font-semibold dark:text-white">Clipboard History</h2>
      </div>
    </div>
  )
}

// Settings window
const SettingsWindow: Component = () => {
  return (
    <div class="h-full w-full bg-white dark:bg-gray-900">
      <div class="p-4">
        <h2 class="text-lg font-semibold dark:text-white">Settings</h2>
      </div>
    </div>
  )
}

// AI chat window
const AIChatWindow: Component = () => {
  return (
    <div class="h-full w-full bg-white dark:bg-gray-900">
      <div class="p-4">
        <h2 class="text-lg font-semibold dark:text-white">AI Chat</h2>
      </div>
    </div>
  )
}

// Pin window for screenshots
const PinWindow: Component = () => {
  return (
    <div class="h-full w-full bg-transparent">
      <div class="p-4">
        <div class="rounded-lg bg-white/95 p-2 dark:bg-gray-900/95">
          Pinned Screenshot
        </div>
      </div>
    </div>
  )
}

export default App
