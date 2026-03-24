use common::types::{Network, LineId, StationId, Point2D};
use std::collections::HashSet;
use crate::types::{PositionMap, AnchorSet};

#[derive(Debug, Clone, Copy)]
enum PolygonShape {
    Rectangle,
    Octagon,
    ElongatedOctagon,
}

fn select_shape(k: usize) -> PolygonShape {
    if k <= 8 { PolygonShape::Rectangle }
    else if k <= 16 { PolygonShape::Octagon }
    else { PolygonShape::ElongatedOctagon }
}

fn polygon_vertices(shape: PolygonShape, k: usize, center: (f64, f64), radius: f64, rotation_deg: f64) -> Vec<(f64, f64)> {
    let n = match shape {
        PolygonShape::Rectangle => 4,
        PolygonShape::Octagon => 8,
        PolygonShape::ElongatedOctagon => 8,
    };
    let rot_rad = rotation_deg.to_radians();
    let mut verts = Vec::new();

    for i in 0..n {
        let angle = rot_rad + (i as f64) * std::f64::consts::TAU / n as f64;
        let (sx, sy) = match shape {
            PolygonShape::ElongatedOctagon => (1.6, 1.0),
            _ => (1.0, 1.0),
        };
        let x = center.0 + angle.cos() * radius * sx;
        let y = center.1 + angle.sin() * radius * sy;
        verts.push((x, y));
    }

    let mut positions = Vec::new();
    for i in 0..k {
        let t = i as f64 / k as f64;
        let total_len = n as f64;
        let segment_pos = t * total_len;
        let seg = (segment_pos as usize).min(n - 1);
        let frac = segment_pos - seg as f64;
        let next_seg = (seg + 1) % n;
        let (x1, y1) = verts[seg];
        let (x2, y2) = verts[next_seg];
        positions.push((x1 + frac * (x2 - x1), y1 + frac * (y2 - y1)));
    }
    positions
}

fn compute_radius(k: usize, d_min: f64, shape: PolygonShape) -> f64 {
    let n = match shape {
        PolygonShape::Rectangle => 4,
        PolygonShape::Octagon => 8,
        PolygonShape::ElongatedOctagon => 8,
    };
    let perimeter_needed = k as f64 * d_min * 1.1;
    let approx_radius = perimeter_needed / (2.0 * std::f64::consts::PI);
    approx_radius.max(d_min * n as f64 / std::f64::consts::TAU)
}

pub fn pre_place_circle_lines(
    network: &Network,
    circle_line_ids: &HashSet<LineId>,
    stage3_positions: &PositionMap,
    d_min: f64,
    positions: &mut PositionMap,
    anchors: &mut AnchorSet,
) {
    for &line_id in circle_line_ids {
        let line = match network.lines.get(&line_id) {
            Some(l) => l,
            None => continue,
        };
        let station_ids = &line.station_ids;
        let k = station_ids.len();
        if k < 3 { continue; }

        let valid_positions: Vec<(f64, f64)> = station_ids.iter()
            .filter_map(|sid| stage3_positions.get(sid).map(|p| (p.x, p.y)))
            .collect();
        if valid_positions.is_empty() { continue; }

        let cx = valid_positions.iter().map(|(x, _)| x).sum::<f64>() / valid_positions.len() as f64;
        let cy = valid_positions.iter().map(|(_, y)| y).sum::<f64>() / valid_positions.len() as f64;

        let shape = select_shape(k);
        let radius = compute_radius(k, d_min, shape);

        let _transfer_on_circle: HashSet<StationId> = station_ids.iter()
            .filter(|&&sid| {
                network.lines.values()
                    .filter(|l| l.id != line_id)
                    .any(|l| l.station_ids.contains(&sid))
            })
            .copied()
            .collect();

        let best_rotation = (0..8)
            .map(|i| i as f64 * 45.0)
            .min_by(|&rot_a, &rot_b| {
                let pa = polygon_vertices(shape, k, (cx, cy), radius, rot_a);
                let pb = polygon_vertices(shape, k, (cx, cy), radius, rot_b);
                let da: f64 = station_ids.iter().enumerate()
                    .filter_map(|(i, sid)| stage3_positions.get(sid).map(|p| {
                        let (px, py) = pa[i];
                        (p.x - px).powi(2) + (p.y - py).powi(2)
                    }))
                    .sum();
                let db: f64 = station_ids.iter().enumerate()
                    .filter_map(|(i, sid)| stage3_positions.get(sid).map(|p| {
                        let (px, py) = pb[i];
                        (p.x - px).powi(2) + (p.y - py).powi(2)
                    }))
                    .sum();
                da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(0.0);

        let final_positions = polygon_vertices(shape, k, (cx, cy), radius, best_rotation);

        for (i, &sid) in station_ids.iter().enumerate() {
            let (x, y) = final_positions[i];
            positions.insert(sid, Point2D { x, y });
            anchors.insert(sid);
        }
    }
}
