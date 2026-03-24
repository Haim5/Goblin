import { create } from 'zustand'

export type ToolMode = 'select' | 'add-station' | 'add-edge' | 'delete'

interface UiState {
  activeTool: ToolMode
  selectedId: string | null
}

interface UiActions {
  setActiveTool: (tool: ToolMode) => void
  setSelectedId: (id: string | null) => void
}

export const useUiStore = create<UiState & UiActions>((set) => ({
  activeTool: 'select',
  selectedId: null,
  setActiveTool: (tool) => set({ activeTool: tool }),
  setSelectedId: (id) => set({ selectedId: id }),
}))
