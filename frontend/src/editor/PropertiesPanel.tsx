import { useUiStore } from '../store/uiStore'
import { useNetworkStore } from '../store/networkStore'
import { LineId } from '../types/network'

export function PropertiesPanel() {
  const selectedId = useUiStore((s) => s.selectedId)
  const network = useNetworkStore((s) => s.network)
  const updateStationName = useNetworkStore((s) => s.updateStationName)
  const assignEdgeToLine = useNetworkStore((s) => s.assignEdgeToLine)
  const removeEdgeFromLine = useNetworkStore((s) => s.removeEdgeFromLine)

  if (!selectedId) {
    return (
      <div style={{ padding: 12, color: '#999', fontSize: 13 }}>
        Select a station or edge to view properties.
      </div>
    )
  }

  const station = network.stations[selectedId]
  if (station) {
    return (
      <div style={{ padding: 12 }}>
        <div style={{ fontWeight: 600, fontSize: 13, marginBottom: 8 }}>Station</div>
        <div style={{ marginBottom: 8 }}>
          <label style={{ fontSize: 12, color: '#666', display: 'block', marginBottom: 2 }}>Name</label>
          <input
            value={station.name}
            onChange={(e) => updateStationName(selectedId, e.target.value)}
            style={{ width: '100%', padding: '4px 6px', border: '1px solid #ccc', borderRadius: 3, fontSize: 13 }}
          />
        </div>
        <div style={{ fontSize: 12, color: '#888' }}>
          {station.geo.lat.toFixed(5)}, {station.geo.lon.toFixed(5)}
        </div>
        <div style={{ fontSize: 11, color: '#aaa', marginTop: 4, wordBreak: 'break-all' }}>{station.id}</div>
      </div>
    )
  }

  const edge = network.edges[selectedId]
  if (edge) {
    return (
      <div style={{ padding: 12 }}>
        <div style={{ fontWeight: 600, fontSize: 13, marginBottom: 8 }}>Edge</div>
        <div style={{ fontSize: 12, color: '#666', marginBottom: 4 }}>
          {network.stations[edge.source]?.name ?? edge.source} → {network.stations[edge.target]?.name ?? edge.target}
        </div>
        <div style={{ fontSize: 12, color: '#666', marginBottom: 4 }}>Lines:</div>
        {Object.values(network.lines).length === 0 ? (
          <div style={{ fontSize: 12, color: '#aaa' }}>No lines defined</div>
        ) : (
          Object.values(network.lines).map((line) => {
            const assigned = edge.line_ids.includes(line.id as LineId)
            return (
              <div key={line.id} style={{ display: 'flex', alignItems: 'center', gap: 6, marginBottom: 4 }}>
                <div style={{ width: 10, height: 10, borderRadius: '50%', background: line.color, flexShrink: 0 }} />
                <span style={{ fontSize: 12, flex: 1 }}>{line.name}</span>
                <button
                  onClick={() => assigned ? removeEdgeFromLine(selectedId, line.id as LineId) : assignEdgeToLine(selectedId, line.id as LineId)}
                  style={{
                    fontSize: 11,
                    padding: '1px 6px',
                    border: '1px solid #ccc',
                    borderRadius: 3,
                    cursor: 'pointer',
                    background: assigned ? '#fee' : '#efe',
                    color: assigned ? '#c00' : '#060',
                  }}
                >
                  {assigned ? 'Remove' : 'Add'}
                </button>
              </div>
            )
          })
        )}
        <div style={{ fontSize: 11, color: '#aaa', marginTop: 4, wordBreak: 'break-all' }}>{edge.id}</div>
      </div>
    )
  }

  return null
}
