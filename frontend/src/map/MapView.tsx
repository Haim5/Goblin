import { useEffect, useRef, useCallback, useState } from 'react'
import L from 'leaflet'
import { useUiStore } from '../store/uiStore'
import { useNetworkStore } from '../store/networkStore'
import { NetworkOverlay } from './NetworkOverlay'
import type { EdgeId, LineId } from '../types/network'

export function MapView() {
  const containerRef = useRef<HTMLDivElement>(null)
  const mapRef = useRef<L.Map | null>(null)
  const [leafletMap, setLeafletMap] = useState<L.Map | null>(null)
  const pendingSourceRef = useRef<string | null>(null)
  const activeTool = useUiStore((s) => s.activeTool)
  const activeLineId = useUiStore((s) => s.activeLineId)
  const setSelectedId = useUiStore((s) => s.setSelectedId)
  const addStation = useNetworkStore((s) => s.addStation)
  const addEdge = useNetworkStore((s) => s.addEdge)
  const assignEdgeToLine = useNetworkStore((s) => s.assignEdgeToLine)

  useEffect(() => {
    if (!containerRef.current || mapRef.current) return
    const map = L.map(containerRef.current).setView([51.505, -0.09], 13)
    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
      attribution: '© OpenStreetMap contributors',
    }).addTo(map)
    mapRef.current = map
    setLeafletMap(map)
    return () => {
      map.remove()
      mapRef.current = null
      setLeafletMap(null)
    }
  }, [])

  const handleMapClick = useCallback(
    (e: L.LeafletMouseEvent) => {
      if (activeTool === 'add-station') {
        const name = window.prompt('Station name:', 'New Station')
        if (name === null) return
        addStation(name || 'New Station', e.latlng.lat, e.latlng.lng)
      } else if (activeTool === 'add-edge') {
        pendingSourceRef.current = null
      }
    },
    [activeTool, addStation]
  )

  const handleStationClick = useCallback(
    (stationId: string) => {
      if (activeTool === 'select') {
        setSelectedId(stationId)
      } else if (activeTool === 'add-edge') {
        if (!pendingSourceRef.current) {
          pendingSourceRef.current = stationId
        } else if (pendingSourceRef.current !== stationId) {
          const edgeId: EdgeId | null = addEdge(pendingSourceRef.current, stationId)
          if (edgeId && activeLineId) {
            assignEdgeToLine(edgeId, activeLineId as LineId)
          }
          pendingSourceRef.current = null
        }
      }
    },
    [activeTool, activeLineId, setSelectedId, addEdge, assignEdgeToLine]
  )

  useEffect(() => {
    const map = mapRef.current
    if (!map) return
    map.on('click', handleMapClick)
    return () => { map.off('click', handleMapClick) }
  }, [handleMapClick])

  return (
    <div style={{ width: '100%', height: '100%', position: 'relative' }}>
      <div ref={containerRef} style={{ width: '100%', height: '100%' }} />
      {leafletMap && (
        <NetworkOverlay map={leafletMap} onStationClick={handleStationClick} />
      )}
    </div>
  )
}
