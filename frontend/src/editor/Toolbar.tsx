import { useUiStore, type ToolMode } from '../store/uiStore'
import { useNetworkStore } from '../store/networkStore'
import { PersistenceBar } from './PersistenceBar'

interface Props {
  onGenerateSchematic: () => void
  generating: boolean
  validationValid: boolean
}

const TOOLS: { mode: ToolMode; label: string }[] = [
  { mode: 'select', label: 'Select' },
  { mode: 'add-station', label: 'Add Station' },
  { mode: 'delete', label: 'Delete' },
]

export function Toolbar({ onGenerateSchematic, generating, validationValid }: Props) {
  const activeTool = useUiStore((s) => s.activeTool)
  const setActiveTool = useUiStore((s) => s.setActiveTool)
  const undo = useNetworkStore((s) => s.undo)
  const redo = useNetworkStore((s) => s.redo)
  const canUndo = useNetworkStore((s) => s.canUndo)
  const canRedo = useNetworkStore((s) => s.canRedo)
  const clearAll = useNetworkStore((s) => s.clearAll)

  const handleClearAll = () => {
    if (window.confirm('Delete all stations, edges, and lines?')) clearAll()
  }

  return (
    <div style={{ display: 'flex', gap: 8, padding: '8px 12px', background: '#fff', borderBottom: '1px solid #ddd', alignItems: 'center', flexWrap: 'wrap' }}>
      {TOOLS.map((t) => (
        <button
          key={t.mode}
          onClick={() => setActiveTool(t.mode)}
          style={{
            padding: '5px 12px',
            background: activeTool === t.mode ? '#1976d2' : '#f0f0f0',
            color: activeTool === t.mode ? '#fff' : '#333',
            border: '1px solid #ccc',
            borderRadius: 4,
            cursor: 'pointer',
            fontWeight: activeTool === t.mode ? 600 : 400,
          }}
        >
          {t.label}
        </button>
      ))}
      <div style={{ width: 1, height: 24, background: '#ddd', margin: '0 4px' }} />
      <button
        onClick={undo}
        disabled={!canUndo()}
        style={{ padding: '5px 12px', border: '1px solid #ccc', borderRadius: 4, cursor: canUndo() ? 'pointer' : 'default', background: '#f0f0f0', opacity: canUndo() ? 1 : 0.4 }}
      >
        Undo
      </button>
      <button
        onClick={redo}
        disabled={!canRedo()}
        style={{ padding: '5px 12px', border: '1px solid #ccc', borderRadius: 4, cursor: canRedo() ? 'pointer' : 'default', background: '#f0f0f0', opacity: canRedo() ? 1 : 0.4 }}
      >
        Redo
      </button>
      <button
        onClick={handleClearAll}
        style={{ padding: '5px 12px', border: '1px solid #e57373', borderRadius: 4, cursor: 'pointer', background: '#fff', color: '#c62828' }}
      >
        Clear All
      </button>
      <div style={{ flex: 1 }} />
      <PersistenceBar />
      <div style={{ width: 1, height: 24, background: '#ddd', margin: '0 4px' }} />
      <button
        onClick={onGenerateSchematic}
        disabled={generating || !validationValid}
        style={{
          padding: '6px 16px',
          background: validationValid ? '#388e3c' : '#aaa',
          color: '#fff',
          border: 'none',
          borderRadius: 4,
          cursor: validationValid && !generating ? 'pointer' : 'default',
          fontWeight: 600,
        }}
      >
        {generating ? 'Generating...' : 'Generate Schematic'}
      </button>
    </div>
  )
}
