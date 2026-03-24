import { describe, it, expect, beforeEach } from 'vitest'
import { reconstructPath } from './pathUtils'
import { useNetworkStore } from './networkStore'
import type { Network } from '../types/network'

beforeEach(() => {
  useNetworkStore.setState({ network: { stations: {}, edges: {}, lines: {} }, past: [], future: [] })
})

function buildPath(n: number): { network: Network; edgeIds: string[]; stationIds: string[] } {
  const store = useNetworkStore.getState()
  const stationIds: string[] = []
  for (let i = 0; i < n; i++) {
    stationIds.push(store.addStation(`S${i}`, i, 0))
  }
  const edgeIds: string[] = []
  for (let i = 0; i < n - 1; i++) {
    edgeIds.push(store.addEdge(stationIds[i], stationIds[i + 1])!)
  }
  return { network: useNetworkStore.getState().network, edgeIds, stationIds }
}

describe('reconstructPath', () => {
  it('returns correct ordered sequence for a path', () => {
    const { network, edgeIds, stationIds } = buildPath(4)
    const result = reconstructPath(edgeIds, network)
    expect(result instanceof Error).toBe(false)
    if (result instanceof Error) return
    expect(result.station_ids.length).toBe(4)
    expect(result.is_loop).toBe(false)
    expect(result.station_ids[0]).toBe(stationIds[0])
    expect(result.station_ids[3]).toBe(stationIds[3])
  })

  it('returns is_loop true for a cycle', () => {
    const { network: net1, edgeIds, stationIds } = buildPath(4)
    useNetworkStore.setState({ network: net1 })
    const closingEdge = useNetworkStore.getState().addEdge(stationIds[3], stationIds[0])!
    const network = useNetworkStore.getState().network
    const result = reconstructPath([...edgeIds, closingEdge], network)
    expect(result instanceof Error).toBe(false)
    if (result instanceof Error) return
    expect(result.is_loop).toBe(true)
  })

  it('returns error for branching edge set', () => {
    const a = useNetworkStore.getState().addStation('A', 0, 0)
    const b = useNetworkStore.getState().addStation('B', 0, 1)
    const c = useNetworkStore.getState().addStation('C', 0, 2)
    const d = useNetworkStore.getState().addStation('D', 0, 3)
    const e1 = useNetworkStore.getState().addEdge(a, b)!
    const e2 = useNetworkStore.getState().addEdge(b, c)!
    const e3 = useNetworkStore.getState().addEdge(b, d)!
    const network = useNetworkStore.getState().network
    const result = reconstructPath([e1, e2, e3], network)
    expect(result instanceof Error).toBe(true)
  })
})
