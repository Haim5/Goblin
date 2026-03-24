import type { EdgeId, StationId, Network } from '../types/network'

export interface PathResult {
  station_ids: StationId[]
  is_loop: boolean
}

export function reconstructPath(edgeIds: EdgeId[], network: Network): PathResult | Error {
  if (edgeIds.length === 0) return { station_ids: [], is_loop: false }

  const adj = new Map<StationId, StationId[]>()
  for (const eid of edgeIds) {
    const edge = network.edges[eid]
    if (!edge) return new Error(`Edge ${eid} not found`)
    if (!adj.has(edge.source)) adj.set(edge.source, [])
    if (!adj.has(edge.target)) adj.set(edge.target, [])
    adj.get(edge.source)!.push(edge.target)
    adj.get(edge.target)!.push(edge.source)
  }

  const degrees = new Map<StationId, number>()
  for (const [node, neighbors] of adj) {
    degrees.set(node, neighbors.length)
  }

  const endpoints = [...degrees.entries()].filter(([, d]) => d === 1).map(([n]) => n)
  const isLoop = endpoints.length === 0

  if (!isLoop && endpoints.length !== 2) {
    return new Error('Edges do not form a simple path or cycle')
  }

  const start = isLoop ? [...adj.keys()][0] : endpoints[0]
  const visited = new Set<StationId>()
  const path: StationId[] = []
  let current = start
  let prev: StationId | null = null

  while (true) {
    path.push(current)
    visited.add(current)
    const neighbors = adj.get(current) ?? []
    const next = neighbors.find((n) => n !== prev && !visited.has(n))
    if (next === undefined) break
    prev = current
    current = next
  }

  if (path.length !== adj.size) {
    return new Error('Edges do not form a single connected path')
  }

  return { station_ids: path, is_loop: isLoop }
}
