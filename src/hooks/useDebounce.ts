import { createSignal, onCleanup, createEffect } from 'solid-js'

/**
 * Debounce hook - delays updating a value until after a specified delay
 * @param value The value to debounce
 * @param delay Delay in milliseconds (default: 150ms)
 * @returns Debounced value
 */
export function useDebounce<T>(value: () => T, delay: number = 150): () => T {
  const [debouncedValue, setDebouncedValue] = createSignal<T>(value())

  // Use createEffect to reactively watch for value changes
  createEffect(() => {
    const currentValue = value()
    const timeoutId = setTimeout(() => {
      setDebouncedValue(() => currentValue)
    }, delay)

    onCleanup(() => {
      clearTimeout(timeoutId)
    })
  })

  return debouncedValue
}
