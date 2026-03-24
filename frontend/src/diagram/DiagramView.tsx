import { useRef, useState, useCallback } from 'react'
import { useLayoutStore } from '../store/layoutStore'
import { useNetworkStore } from '../store/networkStore'
import { DiagramStation } from './DiagramStation'
import { DiagramEdge } from './DiagramEdge'
import { DiagramLabel } from './DiagramLabel'
import type { StationId, Point2D } from '../types/network'

const CANVAS_SIZE = 1000

interface Props {
  anchors: Record<StationId, Point2D>
  onAnchorChange: (id: StationId, position: Point2D) => void
  onAnchorClear: (id: StationId) => void
  onAnchorClearAll: () => void
}

export function DiagramView({ anchors, onAnchorChange, onAnchorClear, onAnchorClearAll }: Props) {
  const result = useLayoutStore((s) => s.result)
  const loading = useLayoutStore((s) => s.loading)
  const error = useLayoutStore((s) => s.error)
  const editMode = useLayoutStore((s) => s.schematicEditMode)
  const setEditMode = useLayoutStore((s) => s.setSchematicEditMode)
  const network = useNetworkStore((s) => s.network)

  const containerRef = useRef<HTMLDivElement>(null)
  const [viewBox, setViewBox] = useState({ x: 0, y: 0, w: CANVAS_SIZE, h: CANVAS_SIZE })
  const panStart = useRef<{ mx: number; my: number; vx: number; vy: number } | null>(null)
  const isPanning = useRef(false)

  const onWheel = useCallback((e: React.WheelEvent) => {
    e.preventDefault()
    const factor = e.deltaY > 0 ? 1.15 : 0.87
    setViewBox((vb) => {
      const rect = containerRef.current?.getBoundingClientRect()
      if (!rect) return vb
      const mx = ((e.clientX - rect.left) / rect.width) * vb.w + vb.x
      const my = ((e.clientY - rect.top) / rect.height) * vb.h + vb.y
      const nw = Math.min(Math.max(vb.w * factor, 100), 3000)
      const nh = Math.min(Math.max(vb.h * factor, 100), 3000)
      return {
        x: mx - (mx - vb.x) * (nw / vb.w),
        y: my - (my - vb.y) * (nh / vb.h),
        w: nw,
        h: nh,
      }
    })
  }, [])

  const onBackgroundMouseDown = useCallback((e: React.MouseEvent) => {
    if (e.button !== 0) return
    isPanning.current = false
    panStart.current = { mx: e.clientX, my: e.clientY, vx: viewBox.x, vy: viewBox.y }
    const startX = e.clientX
    const startY = e.clientY
    const onMove = (ev: MouseEvent) => {
      if (Math.abs(ev.clientX - startX) + Math.abs(ev.clientY - startY) > 3) {
        isPanning.current = true
      }
      if (!panStart.current) return
      const rect = containerRef.current?.getBoundingClientRect()
      if (!rect) return
      const dx = ((ev.clientX - panStart.current.mx) / rect.width) * viewBox.w
      const dy = ((ev.clientY - panStart.current.my) / rect.height) * viewBox.h
      setViewBox((vb) => ({ ...vb, x: panStart.current!.vx - dx, y: panStart.current!.vy - dy }))
    }
    const onUp = () => {
      panStart.current = null
      document.removeEventListener('mousemove', onMove)
      document.removeEventListener('mouseup', onUp)
    }
    document.addEventListener('mousemove', onMove)
    document.addEventListener('mouseup', onUp)
  }, [viewBox.w, viewBox.h, viewBox.x, viewBox.y])

  const handleStationDragStart = useCallback((e: React.MouseEvent, stationId: StationId) => {
    if (!editMode) return
    e.stopPropagation()

    const currentPos = result?.station_positions[stationId]
    if (!currentPos) return

    const startClientX = e.clientX
    const startClientY = e.clientY
    const startPos = { ...currentPos }

    const onMove = (ev: MouseEvent) => {
      const rect = containerRef.current?.getBoundingClientRect()
      if (!rect) return
      const dx = ((ev.clientX - startClientX) / rect.width) * viewBox.w
      const dy = ((ev.clientY - startClientY) / rect.height) * viewBox.h
      onAnchorChange(stationId, {
        x: Math.max(0, Math.min(CANVAS_SIZE, startPos.x + dx)),
        y: Math.max(0, Math.min(CANVAS_SIZE, startPos.y + dy)),
      })
    }
    const onUp = () => {
      document.removeEventListener('mousemove', onMove)
      document.removeEventListener('mouseup', onUp)
    }
    document.addEventListener('mousemove', onMove)
    document.addEventListener('mouseup', onUp)
  }, [editMode, result, viewBox.w, viewBox.h, onAnchorChange])

  if (loading) {
    return <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', height: '100%', color: '#888', fontSize: 14 }}>Generating schematic...</div>
  }

  if (error) {
    return <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', height: '100%', color: '#c62828', fontSize: 13, padding: 20, textAlign: 'center' }}>Error: {error}</div>
  }

  if (!result) {
    return <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', height: '100%', color: '#bbb', fontSize: 13 }}>Click "Generate Schematic" to render the diagram.</div>
  }

  const displayPositions: Record<StationId, Point2D> = { ...result.station_positions, ...anchors }
  const vbStr = `${viewBox.x} ${viewBox.y} ${viewBox.w} ${viewBox.h}`

  return (
    <div style={{ width: '100%', height: '100%', display: 'flex', flexDirection: 'column' }}>
      <div style={{ display: 'flex', alignItems: 'center', gap: 8, padding: '4px 8px', background: editMode ? '#fff8e1' : '#fff', borderBottom: '1px solid #eee', flexShrink: 0 }}>
        <button
          onClick={() => setEditMode(!editMode)}
          style={{ padding: '3px 10px', fontSize: 12, border: '1px solid #ccc', borderRadius: 3, cursor: 'pointer', background: editMode ? '#f57f17' : '#f0f0f0', color: editMode ? '#fff' : '#333', fontWeight: editMode ? 600 : 400 }}
        >
          {editMode ? 'Exit Edit Mode' : 'Edit Schematic'}
        </button>
        {editMode && Object.keys(anchors).length > 0 && (
          <button
            onClick={onAnchorClearAll}
            style={{ padding: '3px 10px', fontSize: 12, border: '1px solid #ccc', borderRadius: 3, cursor: 'pointer', background: '#fff', color: '#c62828' }}
          >
            Reset All Anchors
          </button>
        )}
        {editMode && (
          <span style={{ fontSize: 11, color: '#888' }}>
            Drag stations to refine. {Object.keys(anchors).length > 0 ? `${Object.keys(anchors).length} anchored.` : ''}
          </span>
        )}
        <div style={{ flex: 1 }} />
        <span style={{ fontSize: 11, color: '#aaa' }}>
          oct: {(result.diagnostics.octilinearity_score * 100).toFixed(0)}% · crossings: {result.diagnostics.crossing_count} · {result.diagnostics.elapsed_ms}ms
        </span>
      </div>
      <div
        ref={containerRef}
        style={{ flex: 1, background: '#fff', overflow: 'hidden', cursor: editMode ? 'default' : 'grab' }}
        onWheel={onWheel}
        onMouseDown={onBackgroundMouseDown}
      >
        <svg width="100%" height="100%" viewBox={vbStr}>
          <rect x={0} y={0} width={CANVAS_SIZE} height={CANVAS_SIZE} fill="#fafafa" />
          {Object.values(network.edges).map((edge) => {
            const srcPos = displayPositions[edge.source]
            const tgtPos = displayPositions[edge.target]
            const renderData = result.edge_render_data[edge.id]
            if (!srcPos || !tgtPos) return null
            return (
              <DiagramEdge
                key={edge.id}
                edge={edge}
                srcPos={srcPos}
                tgtPos={tgtPos}
                network={network}
                renderData={renderData ?? null}
              />
            )
          })}
          {Object.values(network.stations).map((station) => {
            const pos = displayPositions[station.id]
            if (!pos) return null
            const lineIds = Object.values(network.edges)
              .filter((e) => e.source === station.id || e.target === station.id)
              .flatMap((e) => e.line_ids)
            const isTransfer = [...new Set(lineIds)].length >= 2
            const isTerminus = Object.values(network.lines).some(
              (l) => l.station_ids[0] === station.id || l.station_ids[l.station_ids.length - 1] === station.id
            )
            const isAnchored = station.id in anchors
            return (
              <DiagramStation
                key={station.id}
                station={station}
                pos={pos}
                network={network}
                isTransfer={isTransfer}
                isTerminus={isTerminus}
                isAnchored={isAnchored}
                editMode={editMode}
                onDragStart={(e) => handleStationDragStart(e, station.id)}
                onClearAnchor={() => onAnchorClear(station.id)}
              />
            )
          })}
          {Object.values(network.stations).map((station) => {
            const placement = result.label_positions[station.id]
            const pos = displayPositions[station.id]
            if (!placement || !pos) return null
            return <DiagramLabel key={station.id} station={station} placement={placement} />
          })}
        </svg>
      </div>
    </div>
  )
}
