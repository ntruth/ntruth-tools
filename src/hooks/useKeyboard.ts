import { onMount, onCleanup } from 'solid-js'

export interface KeyboardHandlers {
  onArrowUp?: () => void
  onArrowDown?: () => void
  onEnter?: () => void
  onEscape?: () => void
  onTab?: () => void
  onBackspace?: () => void
  onCommand1?: () => void
  onCommand2?: () => void
  onCommand3?: () => void
  onCommand4?: () => void
  onCommand5?: () => void
  onCommand6?: () => void
  onCommand7?: () => void
  onCommand8?: () => void
  onCommand9?: () => void
}

/**
 * Keyboard event handling hook
 * Provides handlers for common keyboard shortcuts
 */
export function useKeyboard(handlers: KeyboardHandlers) {
  const handleKeyDown = (event: KeyboardEvent) => {
    const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0
    const cmdOrCtrl = isMac ? event.metaKey : event.ctrlKey

    // Arrow navigation
    if (event.key === 'ArrowUp') {
      event.preventDefault()
      handlers.onArrowUp?.()
    } else if (event.key === 'ArrowDown') {
      event.preventDefault()
      handlers.onArrowDown?.()
    }
    // Enter key
    else if (event.key === 'Enter') {
      event.preventDefault()
      handlers.onEnter?.()
    }
    // Escape key
    else if (event.key === 'Escape') {
      event.preventDefault()
      handlers.onEscape?.()
    }
    // Tab key
    else if (event.key === 'Tab') {
      event.preventDefault()
      handlers.onTab?.()
    }
    // Backspace
    else if (event.key === 'Backspace' && (event.target as HTMLElement).tagName !== 'INPUT') {
      handlers.onBackspace?.()
    }
    // Command/Ctrl + Number shortcuts
    else if (cmdOrCtrl) {
      switch (event.key) {
        case '1':
          event.preventDefault()
          handlers.onCommand1?.()
          break
        case '2':
          event.preventDefault()
          handlers.onCommand2?.()
          break
        case '3':
          event.preventDefault()
          handlers.onCommand3?.()
          break
        case '4':
          event.preventDefault()
          handlers.onCommand4?.()
          break
        case '5':
          event.preventDefault()
          handlers.onCommand5?.()
          break
        case '6':
          event.preventDefault()
          handlers.onCommand6?.()
          break
        case '7':
          event.preventDefault()
          handlers.onCommand7?.()
          break
        case '8':
          event.preventDefault()
          handlers.onCommand8?.()
          break
        case '9':
          event.preventDefault()
          handlers.onCommand9?.()
          break
      }
    }
  }

  onMount(() => {
    window.addEventListener('keydown', handleKeyDown)
  })

  onCleanup(() => {
    window.removeEventListener('keydown', handleKeyDown)
  })
}
