import { useState } from 'react'
import { useNetworkStore } from '../store/networkStore'
import { useUiStore } from '../store/uiStore'
import { LineEditor } from './LineEditor'
import type { LineId } from '../types/network'

export function LinePanel() {
  const lines = useNetworkStore((s) => s.network.lines)
  const addLine = useNetworkStore((s) => s.addLine)
  const [editingId, setEditingId] = useState<LineId | null>(null)
  const activeLineId = useUiStore((s) => s.activeLineId)
  const startDrawingLine = useUiStore((s) => s.startDrawingLine)
  const stopDrawing = useUiStore((s) => s.stopDrawing)

  const handleAdd = () => {
    const colors = ['#E32017', '#003688', '#FFD300', '#00782A', '#F3A9BB', '#A65022', '#9B0056', '#000000']
    const existingCount = Object.keys(lines).length
    const color = colors[existingCount % colors.length]
    const id = addLine(`Line ${existingCount + 1}`, color)
    setEditingId(id)
  }

  const handleDrawToggle = (e: React.MouseEvent, lineId: LineId) => {
    e.stopPropagation()
    if (activeLineId === lineId) {
      stopDrawing()
    } else {
      startDrawingLine(lineId)
    }
  }

  return (
    <div style={{ display: 'flex', flexDirection: 'column', height: '100%' }}>
      <div style={{ padding: '8px 12px', fontWeight: 600, fontSize: 13, borderBottom: '1px solid #eee', color: '#555' }}>Lines</div>
      {activeLineId && lines[activeLineId] && (
        <div style={{ padding: '4px 12px', background: '#fff8e1', borderBottom: '1px solid #ffe082', fontSize: 12, color: '#5f4000', display: 'flex', alignItems: 'center', gap: 6 }}>
          <div style={{ width: 8, height: 8, borderRadius: '50%', background: lines[activeLineId].color }} />
          Drawing edges for {lines[activeLineId].name} — click stations on the map
          <button onClick={stopDrawing} style={{ marginLeft: 'auto', fontSize: 11, border: 'none', background: 'transparent', cursor: 'pointer', color: '#888' }}>✕</button>
        </div>
      )}
      <div style={{ flex: 1, overflowY: 'auto' }}>
        {Object.values(lines).map((line) => (
          <div key={line.id}>
            <div
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
              <button
                onClick={(e) => handleDrawToggle(e, line.id as LineId)}
                style={{
                  fontSize: 11,
                  padding: '2px 8px',
                  border: '1px solid',
                  borderColor: activeLineId === line.id ? line.color : '#ccc',
                  borderRadius: 3,
                  cursor: 'pointer',
                  background: activeLineId === line.id ? line.color : '#f5f5f5',
                  color: activeLineId === line.id ? '#fff' : '#555',
                  fontWeight: activeLineId === line.id ? 600 : 400,
                }}
              >
                {activeLineId === line.id ? 'Stop' : 'Draw'}
              </button>
            </div>
            {editingId === line.id && lines[editingId] && (
              <LineEditor lineId={editingId} onClose={() => setEditingId(null)} />
            )}
          </div>
        ))}
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
