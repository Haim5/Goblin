import { useState } from 'react'
import { useNetworkStore } from '../store/networkStore'
import { useUiStore } from '../store/uiStore'
import { LineEditor } from './LineEditor'
import type { LineId } from '../types/network'

export function LinePanel() {
  const lines = useNetworkStore((s) => s.network.lines)
  const addLine = useNetworkStore((s) => s.addLine)
  const [editingId, setEditingId] = useState<LineId | null>(null)
  const selectedId = useUiStore((s) => s.selectedId)

  void selectedId

  const handleAdd = () => {
    const colors = ['#E32017', '#003688', '#FFD300', '#00782A', '#F3A9BB', '#A65022', '#9B0056', '#000000']
    const existingCount = Object.keys(lines).length
    const color = colors[existingCount % colors.length]
    const id = addLine(`Line ${existingCount + 1}`, color)
    setEditingId(id)
  }

  return (
    <div style={{ display: 'flex', flexDirection: 'column', height: '100%' }}>
      <div style={{ padding: '8px 12px', fontWeight: 600, fontSize: 13, borderBottom: '1px solid #eee', color: '#555' }}>Lines</div>
      <div style={{ flex: 1, overflowY: 'auto' }}>
        {Object.values(lines).map((line) => (
          <div
            key={line.id}
            onClick={() => setEditingId(editingId === line.id ? null : line.id)}
            style={{
              padding: '8px 12px',
              cursor: 'pointer',
              background: editingId === line.id ? '#e3f2fd' : 'transparent',
              borderBottom: '1px solid #f0f0f0',
              display: 'flex',
              alignItems: 'center',
              gap: 8,
            }}
          >
            <div style={{ width: 12, height: 12, borderRadius: '50%', background: line.color, flexShrink: 0 }} />
            <span style={{ fontSize: 13, flex: 1 }}>{line.name}</span>
            {line.is_loop && <span style={{ fontSize: 11, color: '#888' }}>↺</span>}
          </div>
        ))}
        {editingId && lines[editingId] && (
          <LineEditor lineId={editingId} onClose={() => setEditingId(null)} />
        )}
      </div>
      <div style={{ padding: 8, borderTop: '1px solid #eee' }}>
        <button
          onClick={handleAdd}
          style={{ width: '100%', padding: '6px 0', background: '#1976d2', color: '#fff', border: 'none', borderRadius: 4, cursor: 'pointer', fontWeight: 600 }}
        >
          + New Line
        </button>
      </div>
    </div>
  )
}
