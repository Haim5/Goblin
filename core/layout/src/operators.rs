use common::types::{StationId, Point2D};
use crate::types::{PositionMap, AnchorSet};

pub fn op_translate(
    positions: &mut PositionMap,
    station_id: StationId,
    delta_x: f64,
    delta_y: f64,
    canvas_size: f64,
) -> Option<Point2D> {
    let pos = positions.get(&station_id)?.clone();
    let new_x = (pos.x + delta_x).clamp(0.0, canvas_size);
    let new_y = (pos.y + delta_y).clamp(0.0, canvas_size);
    positions.insert(station_id, Point2D { x: new_x, y: new_y });
    Some(pos)
}

pub fn op_swap(positions: &mut PositionMap, id_a: StationId, id_b: StationId) -> bool {
    let pa = match positions.get(&id_a) { Some(p) => p.clone(), None => return false };
    let pb = match positions.get(&id_b) { Some(p) => p.clone(), None => return false };
    positions.insert(id_a, pb);
    positions.insert(id_b, pa);
    true
}

pub fn op_segment_shift(
    positions: &mut PositionMap,
    station_ids: &[StationId],
    anchors: &AnchorSet,
    delta_x: f64,
    delta_y: f64,
    canvas_size: f64,
) -> Vec<(StationId, Point2D)> {
    let mut undos = Vec::new();
    for &sid in station_ids {
        if anchors.contains(&sid) {
            continue;
        }
        if let Some(pos) = positions.get(&sid).cloned() {
            undos.push((sid, pos.clone()));
            let new_x = (pos.x + delta_x).clamp(0.0, canvas_size);
            let new_y = (pos.y + delta_y).clamp(0.0, canvas_size);
            positions.insert(sid, Point2D { x: new_x, y: new_y });
        }
    }
    undos
}

pub fn op_reflect(
    positions: &mut PositionMap,
    station_id: StationId,
    neighbor_a: StationId,
    neighbor_b: StationId,
    canvas_size: f64,
) -> Option<Point2D> {
    let p = positions.get(&station_id)?.clone();
    let a = positions.get(&neighbor_a)?.clone();
    let b = positions.get(&neighbor_b)?.clone();
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let len2 = dx * dx + dy * dy;
    if len2 < 1e-10 {
        return None;
    }
    let t = ((p.x - a.x) * dx + (p.y - a.y) * dy) / len2;
    let foot_x = a.x + t * dx;
    let foot_y = a.y + t * dy;
    let new_x = (2.0 * foot_x - p.x).clamp(0.0, canvas_size);
    let new_y = (2.0 * foot_y - p.y).clamp(0.0, canvas_size);
    positions.insert(station_id, Point2D { x: new_x, y: new_y });
    Some(p)
}
