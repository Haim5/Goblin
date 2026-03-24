export type StationId = string
export type EdgeId = string
export type LineId = string

export interface GeoPoint {
  lat: number
  lon: number
}

export interface Point2D {
  x: number
  y: number
}

export interface Station {
  id: StationId
  name: string
  geo: GeoPoint
  metadata: Record<string, unknown>
}

export interface Edge {
  id: EdgeId
  source: StationId
  target: StationId
  line_ids: LineId[]
  metadata: Record<string, unknown>
}

export interface Line {
  id: LineId
  name: string
  code: string | null
  color: string
  station_ids: StationId[]
  is_loop: boolean
  metadata: Record<string, unknown>
}

export interface Network {
  stations: Record<StationId, Station>
  edges: Record<EdgeId, Edge>
  lines: Record<LineId, Line>
}

export interface ManualAnchor {
  station_id: StationId
  position: Point2D
}

export interface EnergyWeights {
  octilinear: number
  length: number
  bends: number
  separation: number
  crossings: number
  topology: number
  balance: number
}

export interface LayoutOptions {
  time_budget_ms: number
  min_station_spacing: number
  canvas_size: number
  weights: EnergyWeights
}

export interface LayoutRequest {
  network: Network
  manual_anchors: ManualAnchor[]
  options: LayoutOptions
}

export type LabelDirection = 'N' | 'NE' | 'E' | 'SE' | 'S' | 'SW' | 'W' | 'NW'

export interface LabelPlacement {
  position: Point2D
  direction: LabelDirection
}

export interface EdgeRenderData {
  line_offsets: Record<LineId, number>
  bundle_order: LineId[]
}

export interface LayoutDiagnostics {
  elapsed_ms: number
  final_energy: number
  octilinearity_score: number
  crossing_count: number
  circle_lines_detected: LineId[]
  label_conflicts: StationId[]
}

export interface LayoutResult {
  station_positions: Record<StationId, Point2D>
  label_positions: Record<StationId, LabelPlacement>
  edge_render_data: Record<EdgeId, EdgeRenderData>
  diagnostics: LayoutDiagnostics
}

export interface ValidationError {
  code: string
  message: string
  entity_id: string | null
}

export interface ValidationResponse {
  valid: boolean
  errors: ValidationError[]
}
