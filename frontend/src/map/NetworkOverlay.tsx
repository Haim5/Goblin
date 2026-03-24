import { useEffect, useRef, useState, useCallback } from 'react'
import L from 'leaflet'
import { useNetworkStore } from '../store/networkStore'
import { useUiStore } from '../store/uiStore'
import type { StationId } from '../types/network'

interface Props {
  map: L.Map
  onStationClick: (id: StationId) => void
}

interface PixelPos {
  x: number
  y: number
}

function geoToPixel(map: L.Map, lat: number, lon: number): PixelPos {
  const point = map.latLngToContainerPoint([lat, lon])
  return { x: point.x, y: point.y }
}

export function NetworkOverlay({ map, onStationClick }: Props) {
  const overlayRef = useRef<SVGSVGElement>(null)
  const [, forceUpdate] = useState(0)
  const network = useNetworkStore((s) => s.network)
  const selectedId = useUiStore((s) => s.selectedId)
  const activeTool = useUiStore((s) => s.activeTool)
  const updateStationPosition = useNetworkStore((s) => s.updateStationPosition)
  const deleteStation = useNetworkStore((s) => s.deleteStation)
  const deleteEdge = useNetworkStore((s) => s.deleteEdge)

  useEffect(() => {
    const update = () => forceUpdate((n) => n + 1)
    map.on('move zoom', update)
    return () => { map.off('move zoom', update) }
  }, [map])

  const syncSize = useCallback(() => {
    const container = map.getContainer()
    const svg = overlayRef.current
    if (!svg) return
    svg.setAttribute('width', String(container.clientWidth))
    svg.setAttribute('height', String(container.clientHeight))
  }, [map])

  useEffect(() => {
    syncSize()
    const observer = new ResizeObserver(syncSize)
    observer.observe(map.getContainer())
    return () => observer.disconnect()
  }, [map, syncSize])

  const handleStationMouseDown = useCallback(
    (e: React.MouseEvent, stationId: StationId) => {
      e.stopPropagation()
      if (activeTool === 'delete') {
        deleteStation(stationId)
        return
      }
      if (activeTool === 'add-edge') {
        onStationClick(stationId)
        return
      }
      if (activeTool !== 'select') return

      const startX = e.clientX
      const startY = e.clientY
      const station = network.stations[stationId]
      const startLatLng = L.latLng(station.geo.lat, station.geo.lon)
      let moved = false

      const onMouseMove = (ev: MouseEvent) => {
        const dx = ev.clientX - startX
        const dy = ev.clientY - startY
        if (Math.abs(dx) + Math.abs(dy) > 3) moved = true
        if (!moved) return
        const startPx = map.latLngToContainerPoint(startLatLng)
        const newPx = L.point(startPx.x + dx, startPx.y + dy)
        const newLatLng = map.containerPointToLatLng(newPx)
        updateStationPosition(stationId, newLatLng.lat, newLatLng.lng)
      }

      const onMouseUp = () => {
        document.removeEventListener('mousemove', onMouseMove)
        document.removeEventListener('mouseup', onMouseUp)
        if (!moved) onStationClick(stationId)
      }

      document.addEventListener('mousemove', onMouseMove)
      document.addEventListener('mouseup', onMouseUp)
    },
    [activeTool, deleteStation, map, network.stations, onStationClick, updateStationPosition]
  )

  const handleEdgeClick = useCallback(
    (e: React.MouseEvent, edgeId: string) => {
      e.stopPropagation()
      if (activeTool === 'delete') {
        deleteEdge(edgeId)
      } else if (activeTool === 'select') {
        useUiStore.getState().setSelectedId(edgeId)
      }
    },
    [activeTool, deleteEdge]
  )

  const lines = Object.values(network.lines)

  const getStationColor = (stationId: StationId): string => {
    const lineIds = Object.values(network.edges)
      .filter((e) => e.source === stationId || e.target === stationId)
      .flatMap((e) => e.line_ids)
    if (lineIds.length === 0) return '#666'
    const line = network.lines[lineIds[0]]
    return line?.color ?? '#666'
  }

  const container = map.getContainer()
  const w = container.clientWidth
  const h = container.clientHeight

  return (
    <svg
      ref={overlayRef}
      width={w}
      height={h}
      style={{ position: 'absolute', top: 0, left: 0, pointerEvents: 'none', zIndex: 1000 }}
    >
      {Object.values(network.edges).map((edge) => {
        const src = network.stations[edge.source]
        const tgt = network.stations[edge.target]
        if (!src || !tgt) return null
        const sp = geoToPixel(map, src.geo.lat, src.geo.lon)
        const tp = geoToPixel(map, tgt.geo.lat, tgt.geo.lon)
        const color = edge.line_ids[0] ? (network.lines[edge.line_ids[0]]?.color ?? '#999') : '#999'
        return (
          <line
            key={edge.id}
            x1={sp.x} y1={sp.y} x2={tp.x} y2={tp.y}
            stroke={color}
            strokeWidth={selectedId === edge.id ? 5 : 3}
            strokeOpacity={0.85}
            style={{ pointerEvents: 'stroke', cursor: activeTool === 'delete' ? 'pointer' : 'default' }}
            onClick={(e) => handleEdgeClick(e, edge.id)}
          />
        )
      })}
      {Object.values(network.stations).map((station) => {
        const p = geoToPixel(map, station.geo.lat, station.geo.lon)
        const isTransfer = lines.filter((l) => l.station_ids.includes(station.id)).length >= 2
        const color = getStationColor(station.id)
        return (
          <g
            key={station.id}
            style={{ pointerEvents: 'all', cursor: activeTool === 'delete' ? 'pointer' : 'grab' }}
            onMouseDown={(e) => handleStationMouseDown(e, station.id)}
          >
            <circle cx={p.x} cy={p.y} r={isTransfer ? 12 : 9} fill={color} stroke="white" strokeWidth={2} />
            {selectedId === station.id && (
              <circle cx={p.x} cy={p.y} r={isTransfer ? 12 : 9} fill="none" stroke="#2196f3" strokeWidth={2} strokeDasharray="4 2" />
            )}
            <text x={p.x + 10} y={p.y + 4} fontSize={11} fill="#333" style={{ pointerEvents: 'none', userSelect: 'none' }}>
              {station.name}
            </text>
          </g>
        )
      })}
    </svg>
  )
}
