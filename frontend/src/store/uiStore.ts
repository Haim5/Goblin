import { create } from 'zustand'

export type ToolMode = 'select' | 'add-station' | 'add-edge' | 'delete'

interface UiState {
  activeTool: ToolMode
  selectedId: string | null
  activeLineId: string | null
}

interface UiActions {
  setActiveTool: (tool: ToolMode) => void
  setSelectedId: (id: string | null) => void
  startDrawingLine: (lineId: string) => void
  stopDrawing: () => void
}

export const useUiStore = create<UiState & UiActions>((set) => ({
  activeTool: 'select',
  selectedId: null,
  activeLineId: null,
  setActiveTool: (tool) => set({ activeTool: tool, activeLineId: null }),
  setSelectedId: (id) => set({ selectedId: id }),
  startDrawingLine: (lineId) => set({ activeTool: 'add-edge', activeLineId: lineId }),
  stopDrawing: () => set({ activeTool: 'select', activeLineId: null }),
}))
