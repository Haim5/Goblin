import { create } from 'zustand'
import type { LayoutResult } from '../types/network'

interface LayoutState {
  result: LayoutResult | null
  loading: boolean
  error: string | null
  schematicEditMode: boolean
}

interface LayoutActions {
  setResult: (result: LayoutResult) => void
  setLoading: (loading: boolean) => void
  setError: (error: string | null) => void
  setSchematicEditMode: (enabled: boolean) => void
}

export const useLayoutStore = create<LayoutState & LayoutActions>((set) => ({
  result: null,
  loading: false,
  error: null,
  schematicEditMode: false,
  setResult: (result) => set({ result, error: null }),
  setLoading: (loading) => set({ loading }),
  setError: (error) => set({ error, loading: false }),
  setSchematicEditMode: (schematicEditMode) => set({ schematicEditMode }),
}))
