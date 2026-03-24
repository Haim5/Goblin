import { useNetworkStore } from '../store/networkStore'
import type { LineId, StationId } from '../types/network'

interface Props {
  lineId: LineId
  onClose: () => void
}

export function LineEditor({ lineId, onClose }: Props) {
  const line = useNetworkStore((s) => s.network.lines[lineId])
  const stations = useNetworkStore((s) => s.network.stations)
  const updateLine = useNetworkStore((s) => s.updateLine)
  const setLineStations = useNetworkStore((s) => s.setLineStations)
  const deleteLine = useNetworkStore((s) => s.deleteLine)

  if (!line) return null

  const moveStation = (index: number, direction: -1 | 1) => {
    const newIds = [...line.station_ids]
    const swapIdx = index + direction
    if (swapIdx < 0 || swapIdx >= newIds.length) return
    ;[newIds[index], newIds[swapIdx]] = [newIds[swapIdx], newIds[index]]
    setLineStations(lineId, newIds)
  }

  const removeStation = (stationId: StationId) => {
    setLineStations(lineId, line.station_ids.filter((id) => id !== stationId))
  }

  return (
    <div style={{ padding: 12, background: '#f8f9fa', borderBottom: '1px solid #ddd' }}>
      <div style={{ marginBottom: 8 }}>
        <label style={{ fontSize: 12, color: '#666', display: 'block', marginBottom: 2 }}>Name</label>
        <input
          value={line.name}
          onChange={(e) => updateLine(lineId, { name: e.target.value })}
          style={{ width: '100%', padding: '4px 6px', border: '1px solid #ccc', borderRadius: 3, fontSize: 13 }}
        />
      </div>
      <div style={{ marginBottom: 8, display: 'flex', gap: 8, alignItems: 'center' }}>
        <div>
          <label style={{ fontSize: 12, color: '#666', display: 'block', marginBottom: 2 }}>Color</label>
          <input
            type="color"
            value={line.color}
            onChange={(e) => updateLine(lineId, { color: e.target.value })}
            style={{ width: 40, height: 28, border: '1px solid #ccc', borderRadius: 3, cursor: 'pointer', padding: 2 }}
          />
        </div>
        <div style={{ alignSelf: 'flex-end', paddingBottom: 2 }}>
          <label style={{ fontSize: 12, color: '#666', display: 'flex', alignItems: 'center', gap: 4, cursor: 'pointer' }}>
            <input
              type="checkbox"
              checked={line.is_loop}
              onChange={(e) => updateLine(lineId, { is_loop: e.target.checked })}
            />
            Circle Line ↺
          </label>
        </div>
      </div>
      {line.station_ids.length > 0 && (
        <div style={{ marginBottom: 8 }}>
          <label style={{ fontSize: 12, color: '#666', display: 'block', marginBottom: 4 }}>Station Order</label>
          {line.station_ids.map((sid, i) => (
            <div key={sid} style={{ display: 'flex', alignItems: 'center', gap: 4, marginBottom: 2 }}>
              <span style={{ fontSize: 12, flex: 1, color: '#333' }}>{stations[sid]?.name ?? sid}</span>
              <button onClick={() => moveStation(i, -1)} disabled={i === 0} style={{ padding: '1px 5px', fontSize: 11, border: '1px solid #ccc', borderRadius: 2, cursor: i === 0 ? 'default' : 'pointer', background: '#fff' }}>↑</button>
              <button onClick={() => moveStation(i, 1)} disabled={i === line.station_ids.length - 1} style={{ padding: '1px 5px', fontSize: 11, border: '1px solid #ccc', borderRadius: 2, cursor: i === line.station_ids.length - 1 ? 'default' : 'pointer', background: '#fff' }}>↓</button>
              <button onClick={() => removeStation(sid)} style={{ padding: '1px 5px', fontSize: 11, border: '1px solid #ccc', borderRadius: 2, cursor: 'pointer', background: '#fff', color: '#c00' }}>×</button>
            </div>
          ))}
        </div>
      )}
      <div style={{ display: 'flex', gap: 6 }}>
        <button
          onClick={() => { deleteLine(lineId); onClose() }}
          style={{ padding: '4px 10px', background: '#c62828', color: '#fff', border: 'none', borderRadius: 3, cursor: 'pointer', fontSize: 12 }}
        >
          Delete Line
        </button>
        <button
          onClick={onClose}
          style={{ padding: '4px 10px', background: '#eee', color: '#333', border: '1px solid #ccc', borderRadius: 3, cursor: 'pointer', fontSize: 12 }}
        >
          Close
        </button>
      </div>
    </div>
  )
}
