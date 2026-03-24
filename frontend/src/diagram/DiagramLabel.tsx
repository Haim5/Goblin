import type { Station, LabelPlacement } from '../types/network'

interface Props {
  station: Station
  placement: LabelPlacement
}

export function DiagramLabel({ station, placement }: Props) {
  const { position, direction } = placement

  let textAnchor: 'start' | 'middle' | 'end' = 'middle'
  if (direction === 'E' || direction === 'NE' || direction === 'SE') textAnchor = 'start'
  if (direction === 'W' || direction === 'NW' || direction === 'SW') textAnchor = 'end'

  let dominantBaseline: 'auto' | 'middle' | 'hanging' = 'middle'
  if (direction === 'N' || direction === 'NE' || direction === 'NW') dominantBaseline = 'auto'
  if (direction === 'S' || direction === 'SE' || direction === 'SW') dominantBaseline = 'hanging'

  return (
    <text
      x={position.x}
      y={position.y}
      fontSize={10}
      fill="#333"
      textAnchor={textAnchor}
      dominantBaseline={dominantBaseline}
      fontFamily="sans-serif"
      style={{ userSelect: 'none', pointerEvents: 'none' }}
    >
      {station.name}
    </text>
  )
}
