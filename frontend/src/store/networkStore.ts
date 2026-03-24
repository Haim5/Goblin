import { create } from 'zustand'
import { v4 as uuidv4 } from 'uuid'
import type { Network, Station, Edge, Line, StationId, EdgeId, LineId } from '../types/network'

interface NetworkState {
  network: Network
  past: Network[]
  future: Network[]
}

interface NetworkActions {
  addStation: (name: string, lat: number, lon: number) => StationId
  updateStationPosition: (id: StationId, lat: number, lon: number) => void
  updateStationName: (id: StationId, name: string) => void
  deleteStation: (id: StationId) => void
  addEdge: (sourceId: StationId, targetId: StationId) => EdgeId | null
  deleteEdge: (id: EdgeId) => void
  assignEdgeToLine: (edgeId: EdgeId, lineId: LineId) => void
  removeEdgeFromLine: (edgeId: EdgeId, lineId: LineId) => void
  addLine: (name: string, color: string) => LineId
  updateLine: (id: LineId, patch: Partial<Pick<Line, 'name' | 'color' | 'is_loop' | 'code'>>) => void
  setLineStations: (lineId: LineId, stationIds: StationId[]) => void
  deleteLine: (id: LineId) => void
  clearAll: () => void
  undo: () => void
  redo: () => void
  canUndo: () => boolean
  canRedo: () => boolean
}

const MAX_HISTORY = 50

function snapshot(network: Network): Network {
  return JSON.parse(JSON.stringify(network)) as Network
}

function pushHistory(past: Network[], current: Network): Network[] {
  const next = [...past, snapshot(current)]
  return next.length > MAX_HISTORY ? next.slice(next.length - MAX_HISTORY) : next
}

export const useNetworkStore = create<NetworkState & NetworkActions>((set, get) => ({
  network: { stations: {}, edges: {}, lines: {} },
  past: [],
  future: [],

  addStation: (name, lat, lon) => {
    const id = uuidv4()
    set((s) => {
      const station: Station = { id, name, geo: { lat, lon }, metadata: {} }
      return {
        past: pushHistory(s.past, s.network),
        future: [],
        network: { ...s.network, stations: { ...s.network.stations, [id]: station } },
      }
    })
    return id
  },

  updateStationPosition: (id, lat, lon) => {
    set((s) => {
      const station = s.network.stations[id]
      if (!station) return s
      return {
        past: pushHistory(s.past, s.network),
        future: [],
        network: {
          ...s.network,
          stations: { ...s.network.stations, [id]: { ...station, geo: { lat, lon } } },
        },
      }
    })
  },

  updateStationName: (id, name) => {
    set((s) => {
      const station = s.network.stations[id]
      if (!station) return s
      return {
        past: pushHistory(s.past, s.network),
        future: [],
        network: {
          ...s.network,
          stations: { ...s.network.stations, [id]: { ...station, name } },
        },
      }
    })
  },

  deleteStation: (id) => {
    set((s) => {
      const stations = { ...s.network.stations }
      delete stations[id]

      const edges: Record<EdgeId, Edge> = {}
      for (const [eid, edge] of Object.entries(s.network.edges)) {
        if (edge.source !== id && edge.target !== id) {
          edges[eid] = edge
        }
      }

      const lines: Record<LineId, Line> = {}
      for (const [lid, line] of Object.entries(s.network.lines)) {
        lines[lid] = {
          ...line,
          station_ids: line.station_ids.filter((sid) => sid !== id),
        }
      }

      return {
        past: pushHistory(s.past, s.network),
        future: [],
        network: { stations, edges, lines },
      }
    })
  },

  addEdge: (sourceId, targetId) => {
    const { network } = get()
    if (!network.stations[sourceId] || !network.stations[targetId]) return null
    if (sourceId === targetId) return null
    const duplicate = Object.values(network.edges).find(
      (e) =>
        (e.source === sourceId && e.target === targetId) ||
        (e.source === targetId && e.target === sourceId)
    )
    if (duplicate) return duplicate.id as EdgeId

    const id = uuidv4()
    set((s) => {
      const edge: Edge = { id, source: sourceId, target: targetId, line_ids: [], metadata: {} }
      return {
        past: pushHistory(s.past, s.network),
        future: [],
        network: { ...s.network, edges: { ...s.network.edges, [id]: edge } },
      }
    })
    return id
  },

  deleteEdge: (id) => {
    set((s) => {
      const edges = { ...s.network.edges }
      delete edges[id]
      return {
        past: pushHistory(s.past, s.network),
        future: [],
        network: { ...s.network, edges },
      }
    })
  },

  assignEdgeToLine: (edgeId, lineId) => {
    set((s) => {
      const edge = s.network.edges[edgeId]
      const line = s.network.lines[lineId]
      if (!edge || !line) return s
      if (edge.line_ids.includes(lineId)) return s
      const updatedEdge = { ...edge, line_ids: [...edge.line_ids, lineId] }
      const stationsToAdd = [edge.source, edge.target].filter(
        (sid) => !line.station_ids.includes(sid)
      )
      const updatedLine = { ...line, station_ids: [...line.station_ids, ...stationsToAdd] }
      return {
        past: pushHistory(s.past, s.network),
        future: [],
        network: {
          ...s.network,
          edges: { ...s.network.edges, [edgeId]: updatedEdge },
          lines: { ...s.network.lines, [lineId]: updatedLine },
        },
      }
    })
  },

  removeEdgeFromLine: (edgeId, lineId) => {
    set((s) => {
      const edge = s.network.edges[edgeId]
      if (!edge) return s
      const updatedEdge = { ...edge, line_ids: edge.line_ids.filter((lid) => lid !== lineId) }
      return {
        past: pushHistory(s.past, s.network),
        future: [],
        network: {
          ...s.network,
          edges: { ...s.network.edges, [edgeId]: updatedEdge },
        },
      }
    })
  },

  addLine: (name, color) => {
    const id = uuidv4()
    set((s) => {
      const line: Line = { id, name, code: null, color, station_ids: [], is_loop: false, metadata: {} }
      return {
        past: pushHistory(s.past, s.network),
        future: [],
        network: { ...s.network, lines: { ...s.network.lines, [id]: line } },
      }
    })
    return id
  },

  updateLine: (id, patch) => {
    set((s) => {
      const line = s.network.lines[id]
      if (!line) return s
      return {
        past: pushHistory(s.past, s.network),
        future: [],
        network: { ...s.network, lines: { ...s.network.lines, [id]: { ...line, ...patch } } },
      }
    })
  },

  setLineStations: (lineId, stationIds) => {
    set((s) => {
      const line = s.network.lines[lineId]
      if (!line) return s
      return {
        past: pushHistory(s.past, s.network),
        future: [],
        network: {
          ...s.network,
          lines: { ...s.network.lines, [lineId]: { ...line, station_ids: stationIds } },
        },
      }
    })
  },

  deleteLine: (id) => {
    set((s) => {
      const lines = { ...s.network.lines }
      delete lines[id]
      const edges: Record<EdgeId, Edge> = {}
      for (const [eid, edge] of Object.entries(s.network.edges)) {
        const remaining = edge.line_ids.filter((lid) => lid !== id)
        if (remaining.length > 0) {
          edges[eid] = { ...edge, line_ids: remaining }
        }
      }
      return {
        past: pushHistory(s.past, s.network),
        future: [],
        network: { ...s.network, edges, lines },
      }
    })
  },

  clearAll: () => {
    set((s) => ({
      past: pushHistory(s.past, s.network),
      future: [],
      network: { stations: {}, edges: {}, lines: {} },
    }))
  },

  undo: () => {
    set((s) => {
      if (s.past.length === 0) return s
      const previous = s.past[s.past.length - 1]
      return {
        past: s.past.slice(0, -1),
        future: [snapshot(s.network), ...s.future].slice(0, MAX_HISTORY),
        network: previous,
      }
    })
  },

  redo: () => {
    set((s) => {
      if (s.future.length === 0) return s
      const next = s.future[0]
      return {
        past: pushHistory(s.past, s.network),
        future: s.future.slice(1),
        network: next,
      }
    })
  },

  canUndo: () => get().past.length > 0,
  canRedo: () => get().future.length > 0,
}))
