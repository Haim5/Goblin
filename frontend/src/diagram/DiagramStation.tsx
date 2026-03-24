import type { Station, Network, Point2D } from '../types/network'

interface Props {
  station: Station
  pos: Point2D
  network: Network
  isTransfer: boolean
  isTerminus: boolean
  isAnchored: boolean
  editMode: boolean
  onDragStart: (e: React.MouseEvent) => void
  onClearAnchor: () => void
}

export function DiagramStation({ station, pos, network, isTransfer, isTerminus, isAnchored, editMode, onDragStart, onClearAnchor }: Props) {
  const lineColors = [...new Set(
    Object.values(network.edges)
      .filter((e) => e.source === station.id || e.target === station.id)
      .flatMap((e) => e.line_ids)
      .map((lid) => network.lines[lid]?.color)
      .filter((c): c is string => !!c)
  )]

  const primaryColor = lineColors[0] ?? '#666'
  const r = isTransfer ? 8 : 5

  return (
    <g
      style={{ cursor: editMode ? 'grab' : 'default' }}
      onMouseDown={editMode ? onDragStart : undefined}
      onContextMenu={isAnchored ? (e) => { e.preventDefault(); onClearAnchor() } : undefined}
    >
      {isTerminus && (
        <circle cx={pos.x} cy={pos.y} r={r + 3} fill={primaryColor} opacity={0.3} />
      )}
      <circle cx={pos.x} cy={pos.y} r={r} fill="white" stroke={primaryColor} strokeWidth={isTransfer ? 2.5 : 2} />
      {isTransfer && lineColors.length > 1 && lineColors.slice(1).map((color, i) => (
        <circle
          key={i}
          cx={pos.x}
          cy={pos.y}
          r={r + (i + 1) * 3.5}
          fill="none"
          stroke={color}
          strokeWidth={2}
          opacity={0.7}
        />
      ))}
      {isAnchored && (
        <text x={pos.x + r + 2} y={pos.y - r} fontSize={9} fill="#f57f17" style={{ pointerEvents: 'none', userSelect: 'none' }}>⚓</text>
      )}
    </g>
  )
}
