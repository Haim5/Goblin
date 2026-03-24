import { describe, it, expect, beforeEach } from 'vitest'
import { useNetworkStore } from './networkStore'

beforeEach(() => {
  useNetworkStore.setState({ network: { stations: {}, edges: {}, lines: {} }, past: [], future: [] })
})

describe('networkStore', () => {
  it('addStation adds a station with correct fields', () => {
    const id = useNetworkStore.getState().addStation('Test', 51.5, -0.1)
    const station = useNetworkStore.getState().network.stations[id]
    expect(station).toBeDefined()
    expect(station.name).toBe('Test')
    expect(station.geo.lat).toBe(51.5)
    expect(station.geo.lon).toBe(-0.1)
  })

  it('deleteStation removes station and cascades to edges and lines', () => {
    const sid = useNetworkStore.getState().addStation('A', 0, 0)
    const sid2 = useNetworkStore.getState().addStation('B', 0, 1)
    const eid = useNetworkStore.getState().addEdge(sid, sid2)!
    const lid = useNetworkStore.getState().addLine('L1', '#f00')
    useNetworkStore.getState().assignEdgeToLine(eid, lid)
    useNetworkStore.getState().deleteStation(sid)

    const state = useNetworkStore.getState().network
    expect(state.stations[sid]).toBeUndefined()
    expect(state.edges[eid]).toBeUndefined()
    const line = state.lines[lid]
    expect(line.station_ids).not.toContain(sid)
  })

  it('addEdge prevents duplicate edges', () => {
    const a = useNetworkStore.getState().addStation('A', 0, 0)
    const b = useNetworkStore.getState().addStation('B', 0, 1)
    useNetworkStore.getState().addEdge(a, b)
    const eid2 = useNetworkStore.getState().addEdge(a, b)
    expect(eid2).toBeNull()
  })

  it('undo restores previous state', () => {
    const id = useNetworkStore.getState().addStation('A', 0, 0)
    expect(useNetworkStore.getState().network.stations[id]).toBeDefined()
    useNetworkStore.getState().undo()
    expect(useNetworkStore.getState().network.stations[id]).toBeUndefined()
  })

  it('redo re-applies undone operation', () => {
    const id = useNetworkStore.getState().addStation('A', 0, 0)
    useNetworkStore.getState().undo()
    useNetworkStore.getState().redo()
    expect(useNetworkStore.getState().network.stations[id]).toBeDefined()
  })

  it('canUndo is false initially', () => {
    expect(useNetworkStore.getState().canUndo()).toBe(false)
  })

  it('canRedo is false initially', () => {
    expect(useNetworkStore.getState().canRedo()).toBe(false)
  })
})
