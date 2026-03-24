use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub type StationId = Uuid;
pub type EdgeId = Uuid;
pub type LineId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoPoint {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Station {
    pub id: StationId,
    pub name: String,
    pub geo: GeoPoint,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub id: EdgeId,
    pub source: StationId,
    pub target: StationId,
    pub line_ids: Vec<LineId>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Line {
    pub id: LineId,
    pub name: String,
    pub code: Option<String>,
    pub color: String,
    pub station_ids: Vec<StationId>,
    pub is_loop: bool,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    pub stations: HashMap<StationId, Station>,
    pub edges: HashMap<EdgeId, Edge>,
    pub lines: HashMap<LineId, Line>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManualAnchor {
    pub station_id: StationId,
    pub position: Point2D,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyWeights {
    pub octilinear: f64,
    pub length: f64,
    pub bends: f64,
    pub separation: f64,
    pub crossings: f64,
    pub topology: f64,
    pub balance: f64,
}

impl Default for EnergyWeights {
    fn default() -> Self {
        Self {
            octilinear: 3.0,
            length: 1.0,
            bends: 2.0,
            separation: 4.0,
            crossings: 2.0,
            topology: 1.5,
            balance: 0.5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutOptions {
    pub time_budget_ms: u64,
    pub min_station_spacing: f64,
    pub canvas_size: f64,
    pub weights: EnergyWeights,
}

impl Default for LayoutOptions {
    fn default() -> Self {
        Self {
            time_budget_ms: 5000,
            min_station_spacing: 60.0,
            canvas_size: 1000.0,
            weights: EnergyWeights::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutRequest {
    pub network: Network,
    pub manual_anchors: Vec<ManualAnchor>,
    pub options: LayoutOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LabelDirection {
    N,
    #[serde(rename = "NE")]
    Ne,
    E,
    #[serde(rename = "SE")]
    Se,
    S,
    #[serde(rename = "SW")]
    Sw,
    W,
    #[serde(rename = "NW")]
    Nw,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelPlacement {
    pub position: Point2D,
    pub direction: LabelDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeRenderData {
    pub line_offsets: HashMap<LineId, f64>,
    pub bundle_order: Vec<LineId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutDiagnostics {
    pub elapsed_ms: u64,
    pub final_energy: f64,
    pub octilinearity_score: f64,
    pub crossing_count: usize,
    pub circle_lines_detected: Vec<LineId>,
    pub label_conflicts: Vec<StationId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutResult {
    pub station_positions: HashMap<StationId, Point2D>,
    pub label_positions: HashMap<StationId, LabelPlacement>,
    pub edge_render_data: HashMap<EdgeId, EdgeRenderData>,
    pub diagnostics: LayoutDiagnostics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub code: String,
    pub message: String,
    pub entity_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResponse {
    pub valid: bool,
    pub errors: Vec<ValidationError>,
}
