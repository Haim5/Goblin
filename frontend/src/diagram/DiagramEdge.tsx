import type { Edge, Network, Point2D, EdgeRenderData } from '../types/network'

interface Props {
  edge: Edge
  srcPos: Point2D
  tgtPos: Point2D
  network: Network
  renderData: EdgeRenderData | null
}

export function DiagramEdge({ edge, srcPos, tgtPos, network, renderData }: Props) {
  const dx = tgtPos.x - srcPos.x
  const dy = tgtPos.y - srcPos.y
  const len = Math.sqrt(dx * dx + dy * dy)
  if (len < 0.01) return null

  const perpX = -dy / len
  const perpY = dx / len

  const lineIds = renderData?.bundle_order ?? edge.line_ids

  if (lineIds.length === 0) {
    return (
      <line x1={srcPos.x} y1={srcPos.y} x2={tgtPos.x} y2={tgtPos.y} stroke="#ccc" strokeWidth={3} />
    )
  }

  return (
    <>
      {lineIds.map((lid) => {
        const offset = renderData?.line_offsets[lid] ?? 0
        const color = network.lines[lid]?.color ?? '#999'
        const ox = perpX * offset
        const oy = perpY * offset
        return (
          <line
            key={lid}
            x1={srcPos.x + ox}
            y1={srcPos.y + oy}
            x2={tgtPos.x + ox}
            y2={tgtPos.y + oy}
            stroke={color}
            strokeWidth={3}
            strokeLinecap="round"
          />
        )
      })}
    </>
  )
}
