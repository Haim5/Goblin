use common::types::{StationId, LabelPlacement, LabelDirection, Point2D, Network};
use crate::types::PositionMap;
use std::collections::HashMap;

#[derive(Clone)]
struct Rect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl Rect {
    fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.w
            && self.x + self.w > other.x
            && self.y < other.y + other.h
            && self.y + self.h > other.y
    }
}

const LABEL_OFFSET: f64 = 10.0;
const LABEL_HEIGHT: f64 = 12.0;
const CHARS_PER_UNIT: f64 = 6.5;

const DIRECTIONS: [LabelDirection; 8] = [
    LabelDirection::E,
    LabelDirection::N,
    LabelDirection::S,
    LabelDirection::W,
    LabelDirection::Ne,
    LabelDirection::Se,
    LabelDirection::Nw,
    LabelDirection::Sw,
];

fn candidate_position(pos: &Point2D, dir: &LabelDirection, label_width: f64) -> (Point2D, Rect) {
    let (dx, dy) = match dir {
        LabelDirection::N  => (0.0, -LABEL_OFFSET - LABEL_HEIGHT),
        LabelDirection::Ne => (LABEL_OFFSET, -LABEL_OFFSET - LABEL_HEIGHT),
        LabelDirection::E  => (LABEL_OFFSET, -LABEL_HEIGHT / 2.0),
        LabelDirection::Se => (LABEL_OFFSET, LABEL_OFFSET),
        LabelDirection::S  => (0.0, LABEL_OFFSET),
        LabelDirection::Sw => (-LABEL_OFFSET - label_width, LABEL_OFFSET),
        LabelDirection::W  => (-LABEL_OFFSET - label_width, -LABEL_HEIGHT / 2.0),
        LabelDirection::Nw => (-LABEL_OFFSET - label_width, -LABEL_OFFSET - LABEL_HEIGHT),
    };
    let lx = pos.x + dx;
    let ly = pos.y + dy;
    let rect = Rect { x: lx, y: ly, w: label_width, h: LABEL_HEIGHT };
    let label_pos = Point2D { x: lx, y: ly + LABEL_HEIGHT };
    (label_pos, rect)
}

fn score_placement(
    rect: &Rect,
    placed: &[Rect],
    network: &Network,
    positions: &PositionMap,
) -> f64 {
    let mut score = 0.0;

    for placed_rect in placed {
        if rect.intersects(placed_rect) {
            score -= 100.0;
        }
    }

    for edge in network.edges.values() {
        let p1 = match positions.get(&edge.source) { Some(p) => p, None => continue };
        let p2 = match positions.get(&edge.target) { Some(p) => p, None => continue };
        if segment_intersects_rect(p1, p2, rect) {
            score -= 50.0;
        }
    }

    score
}

fn segment_intersects_rect(p1: &Point2D, p2: &Point2D, rect: &Rect) -> bool {
    let rx1 = rect.x; let ry1 = rect.y;
    let rx2 = rect.x + rect.w; let ry2 = rect.y + rect.h;
    let min_x = p1.x.min(p2.x); let max_x = p1.x.max(p2.x);
    let min_y = p1.y.min(p2.y); let max_y = p1.y.max(p2.y);
    !(max_x < rx1 || min_x > rx2 || max_y < ry1 || min_y > ry2)
}

pub fn place_labels(
    positions: &PositionMap,
    network: &Network,
) -> (HashMap<StationId, LabelPlacement>, Vec<StationId>) {
    let mut result: HashMap<StationId, LabelPlacement> = HashMap::new();
    let mut placed_rects: Vec<Rect> = Vec::new();
    let mut conflicts: Vec<StationId> = Vec::new();

    let mut terminus_ids: Vec<StationId> = Vec::new();
    let mut transfer_ids: Vec<StationId> = Vec::new();
    let mut regular_ids: Vec<StationId> = Vec::new();

    for station in network.stations.values() {
        let line_count = network.lines.values()
            .filter(|l| l.station_ids.contains(&station.id))
            .count();
        let is_terminus = network.lines.values().any(|l| {
            l.station_ids.first() == Some(&station.id)
                || l.station_ids.last() == Some(&station.id)
        });
        if is_terminus {
            terminus_ids.push(station.id);
        } else if line_count >= 2 {
            transfer_ids.push(station.id);
        } else {
            regular_ids.push(station.id);
        }
    }

    let ordered: Vec<StationId> = terminus_ids.into_iter()
        .chain(transfer_ids)
        .chain(regular_ids)
        .collect();

    for sid in ordered {
        let station = match network.stations.get(&sid) { Some(s) => s, None => continue };
        let pos = match positions.get(&sid) { Some(p) => p, None => continue };
        let label_width = station.name.len() as f64 * CHARS_PER_UNIT;

        let mut best_score = f64::NEG_INFINITY;
        let mut best_placement: Option<(LabelPlacement, Rect)> = None;

        for dir in &DIRECTIONS {
            let (label_pos, rect) = candidate_position(pos, dir, label_width);
            let score = score_placement(&rect, &placed_rects, network, positions);
            if score > best_score {
                best_score = score;
                best_placement = Some((LabelPlacement { position: label_pos, direction: dir.clone() }, rect));
            }
        }

        if let Some((placement, rect)) = best_placement {
            if best_score < -50.0 {
                conflicts.push(sid);
            }
            placed_rects.push(rect);
            result.insert(sid, placement);
        }
    }

    (result, conflicts)
}

pub fn place_labels_stub(positions: &PositionMap) -> HashMap<StationId, LabelPlacement> {
    positions.iter().map(|(id, pos)| {
        (*id, LabelPlacement {
            position: Point2D { x: pos.x + 10.0, y: pos.y + 4.0 },
            direction: LabelDirection::E,
        })
    }).collect()
}
