import { Component, Match, Switch, onMount } from 'solid-js'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { MainPage } from './pages/Main'
import ClipboardPage from './pages/Clipboard'
import AlfredSettings from './pages/Settings/AlfredSettings'
import AIPage from './pages/AI'

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
          <ClipboardPage />
        </Match>
        <Match when={currentWindow.label === 'settings'}>
          <AlfredSettings />
        </Match>
        <Match when={currentWindow.label === 'ai'}>
          <AIPage />
        </Match>
        <Match when={currentWindow.label === 'ai-chat'}>
          <AIPage />
        </Match>
        <Match when={currentWindow.label.startsWith('pin-')}>
          <PinWindow />
        </Match>
      </Switch>
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
