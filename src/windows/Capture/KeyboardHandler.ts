export interface KeyboardHandlerOptions {
  /** Current virtual cursor */
  getCursor: () => { x: number; y: number }
  /** Set virtual cursor (clamped by caller bounds) */
  setCursor: (p: { x: number; y: number }) => void
  /** Canvas bounds (window-size) for clamping */
  getBounds: () => { width: number; height: number }
  /** Set whether Alt is held (for magnifier) */
  setAltDown?: (down: boolean) => void
}

const clamp = (v: number, min: number, max: number) => Math.max(min, Math.min(max, v))

/**
 * WASD pixel-move + Ctrl+Arrow.
 *
 * - Default step: 1px
 * - With Shift: 10px
 * - WASD moves virtual cursor
 * - Ctrl+Arrow moves virtual cursor
 * - Alt down/up signals magnifier enable
 */
export function installCaptureKeyboardHandler(opts: KeyboardHandlerOptions) {
  const onKeyDown = (e: KeyboardEvent) => {
    // Alt toggles magnifier
    if (e.key === 'Alt') {
      opts.setAltDown?.(true)
      return
    }

    const step = e.shiftKey ? 10 : 1

    const isWASD = ['w', 'a', 's', 'd'].includes(e.key.toLowerCase())
    const isCtrlArrow = e.ctrlKey && ['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight'].includes(e.key)

    if (!isWASD && !isCtrlArrow) return

    e.preventDefault()
    e.stopPropagation()

    const { width, height } = opts.getBounds()
    const cur = opts.getCursor()

    let dx = 0
    let dy = 0

    if (isWASD) {
      switch (e.key.toLowerCase()) {
        case 'w': dy = -step; break
        case 's': dy = step; break
        case 'a': dx = -step; break
        case 'd': dx = step; break
      }
    } else {
      switch (e.key) {
        case 'ArrowUp': dy = -step; break
        case 'ArrowDown': dy = step; break
        case 'ArrowLeft': dx = -step; break
        case 'ArrowRight': dx = step; break
      }
    }

    const x = clamp(cur.x + dx, 0, Math.max(0, width - 1))
    const y = clamp(cur.y + dy, 0, Math.max(0, height - 1))
    opts.setCursor({ x, y })
  }

  const onKeyUp = (e: KeyboardEvent) => {
    if (e.key === 'Alt') {
      opts.setAltDown?.(false)
    }
  }

  window.addEventListener('keydown', onKeyDown, { capture: true })
  window.addEventListener('keyup', onKeyUp, { capture: true })

  return () => {
    window.removeEventListener('keydown', onKeyDown, { capture: true } as any)
    window.removeEventListener('keyup', onKeyUp, { capture: true } as any)
  }
}
