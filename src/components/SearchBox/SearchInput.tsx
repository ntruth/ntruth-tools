import { Component, createSignal, onMount, Show } from 'solid-js'
import { Search, X } from 'lucide-solid'

interface SearchInputProps {
  value: string
  onInput: (value: string) => void
  onClear: () => void
  placeholder?: string
  inputType?: string
  autofocus?: boolean
}

/**
 * Search input component with auto-focus and real-time input handling
 */
export const SearchInput: Component<SearchInputProps> = (props) => {
  let inputRef: HTMLInputElement | undefined

  const [isFocused, setIsFocused] = createSignal(false)

  onMount(() => {
    if (props.autofocus !== false && inputRef) {
      inputRef.focus()
    }
  })

  const handleInput = (e: Event) => {
    const target = e.target as HTMLInputElement
    props.onInput(target.value)
  }

  const handleClear = () => {
    props.onClear()
    inputRef?.focus()
  }

  const showClearButton = () => props.value.length > 0

  return (
    <div
      class="relative flex items-center gap-3 rounded-xl bg-white/90 px-4 py-3 shadow-lg backdrop-blur-sm transition-all duration-200 dark:bg-gray-800/90"
      classList={{
        'ring-2 ring-blue-500 dark:ring-blue-400': isFocused(),
      }}
    >
      {/* Search Icon */}
      <Search class="h-5 w-5 flex-shrink-0 text-gray-400 dark:text-gray-500" />

      {/* Input */}
      <input
        ref={inputRef}
        type="text"
        value={props.value}
        onInput={handleInput}
        onFocus={() => setIsFocused(true)}
        onBlur={() => setIsFocused(false)}
        placeholder={props.placeholder || 'Search...'}
        class="flex-1 bg-transparent text-base outline-none placeholder:text-gray-400 dark:text-white dark:placeholder:text-gray-500"
        autocomplete="off"
        spellcheck={false}
      />

      {/* Input Type Indicator */}
      <Show when={props.inputType}>
        <div class="flex-shrink-0 rounded-md bg-blue-100 px-2 py-1 text-xs font-medium text-blue-700 dark:bg-blue-900/50 dark:text-blue-300">
          {props.inputType}
        </div>
      </Show>

      {/* Clear Button */}
      <Show when={showClearButton()}>
        <button
          onClick={handleClear}
          class="flex-shrink-0 rounded-full p-1 text-gray-400 transition-colors hover:bg-gray-100 hover:text-gray-600 dark:hover:bg-gray-700 dark:hover:text-gray-300"
          aria-label="Clear search"
        >
          <X class="h-4 w-4" />
        </button>
      </Show>
    </div>
  )
}
