import { Component, Match, Switch, onMount, onCleanup, createSignal } from 'solid-js'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/core'
import { listen, UnlistenFn } from '@tauri-apps/api/event'
import { MainPage } from './pages/Main'
import ClipboardPage from './pages/Clipboard'
import AlfredSettings from './pages/Settings/AlfredSettings'
import AIPage from './pages/AI'

const App: Component = () => {
  const currentWindow = getCurrentWindow()
  // Clipboard mode state - used to toggle clipboard UI in main window
  const [_clipboardMode, setClipboardMode] = createSignal(false)
  let unlistenClipboard: UnlistenFn | null = null
  
  onMount(async () => {
    // Get window label to determine which component to render
    const label = currentWindow.label
    console.log('Window label:', label)
    
    // Listen for clipboard toggle event from Rust (global shortcut)
    unlistenClipboard = await listen('toggle-clipboard-history', () => {
      console.log('Clipboard toggle event received')
      setClipboardMode(prev => !prev)
    })
    
    // Signal to Rust that frontend is ready
    // Only the main window will be shown on startup
    // Other windows (settings, clipboard, ai) stay hidden until user action
    requestAnimationFrame(() => {
      requestAnimationFrame(async () => {
        try {
          await invoke('app_ready')
          console.log(`App ready signal sent for window: ${label}`)
          
          // Only the main window is shown automatically
          if (label === 'main') {
            console.log('Main window should now be visible')
          } else {
            console.log(`Window '${label}' ready but staying hidden (will show on user action)`)
          }
        } catch (e) {
          console.error('Failed to signal app ready:', e)
          // Only try fallback show for main window
          if (label === 'main') {
            setTimeout(async () => {
              try {
                await currentWindow.show()
                await currentWindow.setFocus()
              } catch (err) {
                console.error('Fallback show failed:', err)
              }
            }, 100)
          }
        }
      })
    })
  })
  
  onCleanup(() => {
    if (unlistenClipboard) {
      unlistenClipboard()
    }
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
