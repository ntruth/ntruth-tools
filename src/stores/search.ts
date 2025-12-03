import { createStore } from 'solid-js/store'
import type { SearchResult, SearchState } from '@/types/search'

const [searchStore, setSearchStore] = createStore<SearchState>({
  query: '',
  results: [],
  selectedIndex: 0,
  loading: false,
})

export const useSearchStore = () => {
  const setQuery = (query: string) => {
    setSearchStore('query', query)
  }

  const setResults = (results: SearchResult[]) => {
    setSearchStore('results', results)
    setSearchStore('selectedIndex', 0)
  }

  const setSelectedIndex = (index: number) => {
    setSearchStore('selectedIndex', index)
  }

  const setLoading = (loading: boolean) => {
    setSearchStore('loading', loading)
  }

  const selectNext = () => {
    const maxIndex = searchStore.results.length - 1
    if (searchStore.selectedIndex < maxIndex) {
      setSearchStore('selectedIndex', searchStore.selectedIndex + 1)
    }
  }

  const selectPrevious = () => {
    if (searchStore.selectedIndex > 0) {
      setSearchStore('selectedIndex', searchStore.selectedIndex - 1)
    }
  }

  return {
    store: searchStore,
    setQuery,
    setResults,
    setSelectedIndex,
    setLoading,
    selectNext,
    selectPrevious,
  }
}

export { searchStore }
