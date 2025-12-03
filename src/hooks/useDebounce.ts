import { createSignal, onCleanup } from 'solid-js'

/**
 * Debounce hook - delays updating a value until after a specified delay
 * @param value The value to debounce
 * @param delay Delay in milliseconds (default: 150ms)
 * @returns Debounced value
 */
export function useDebounce<T>(value: () => T, delay: number = 150): () => T {
  const [debouncedValue, setDebouncedValue] = createSignal<T>(value())

  // Create effect to update debounced value after delay
  let timeoutId: NodeJS.Timeout

  const updateValue = () => {
    clearTimeout(timeoutId)
    timeoutId = setTimeout(() => {
      setDebouncedValue(() => value())
    }, delay)
  }

  // Watch for value changes
  const interval = setInterval(() => {
    if (value() !== debouncedValue()) {
      updateValue()
    }
  }, 50)

  onCleanup(() => {
    clearTimeout(timeoutId)
    clearInterval(interval)
  })

  return debouncedValue
}
