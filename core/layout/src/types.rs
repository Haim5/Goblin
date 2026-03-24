use common::types::{StationId, EdgeId, LineId, Point2D, LabelPlacement, EdgeRenderData};
use std::collections::{HashMap, HashSet};

pub type PositionMap = HashMap<StationId, Point2D>;
pub type AnchorSet = HashSet<StationId>;

#[derive(Debug)]
pub struct PreprocessedNetwork {
    pub degree: HashMap<StationId, usize>,
    pub line_membership: HashMap<StationId, Vec<LineId>>,
    pub transfer_stations: HashSet<StationId>,
    pub circle_line_ids: HashSet<LineId>,
    pub geo_normalized: PositionMap,
    pub anchor_set: AnchorSet,
}

#[derive(Debug)]
pub struct LayoutOutput {
    pub positions: PositionMap,
    pub label_positions: HashMap<StationId, LabelPlacement>,
    pub edge_render_data: HashMap<EdgeId, EdgeRenderData>,
}
