use common::types::{Network, Point2D};
use crate::types::{PositionMap, AnchorSet};

fn edge_angle_deg(p1: &Point2D, p2: &Point2D) -> f64 {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    dy.atan2(dx).to_degrees()
}

fn nearest_octilinear_angle(angle_deg: f64) -> f64 {
    let a = ((angle_deg % 360.0) + 360.0) % 360.0;
    (a / 45.0).round() * 45.0
}

fn deviation_from_octilinear(angle_deg: f64) -> f64 {
    let a = ((angle_deg % 360.0) + 360.0) % 360.0;
    let rem = a % 45.0;
    rem.min(45.0 - rem)
}

pub fn angle_snap_pass(
    positions: &mut PositionMap,
    network: &Network,
    anchors: &AnchorSet,
    epsilon_deg: f64,
    max_iterations: u32,
) {
    for _ in 0..max_iterations {
        let mut any_changed = false;
        for edge in network.edges.values() {
            let p1 = match positions.get(&edge.source) { Some(p) => p.clone(), None => continue };
            let p2 = match positions.get(&edge.target) { Some(p) => p.clone(), None => continue };
            let angle = edge_angle_deg(&p1, &p2);
            if deviation_from_octilinear(angle) <= epsilon_deg {
                continue;
            }
            let dist = ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt();
            if dist < 1.0 { continue; }
            let snapped = nearest_octilinear_angle(angle).to_radians();

            if !anchors.contains(&edge.target) {
                let new_x = p1.x + snapped.cos() * dist;
                let new_y = p1.y + snapped.sin() * dist;
                positions.insert(edge.target, Point2D { x: new_x, y: new_y });
                any_changed = true;
            } else if !anchors.contains(&edge.source) {
                let rev = (snapped + std::f64::consts::PI) % (2.0 * std::f64::consts::PI);
                let new_x = p2.x + rev.cos() * dist;
                let new_y = p2.y + rev.sin() * dist;
                positions.insert(edge.source, Point2D { x: new_x, y: new_y });
                any_changed = true;
            }
        }
        if !any_changed { break; }
    }
}

pub fn separation_pass(
    positions: &mut PositionMap,
    anchors: &AnchorSet,
    d_min: f64,
    max_iterations: u32,
) {
    let ids: Vec<_> = positions.keys().copied().collect();
    for _ in 0..max_iterations {
        let mut any_violation = false;
        for i in 0..ids.len() {
            for j in (i + 1)..ids.len() {
                let p1 = positions[&ids[i]].clone();
                let p2 = positions[&ids[j]].clone();
                let dx = p2.x - p1.x;
                let dy = p2.y - p1.y;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist >= d_min { continue; }
                any_violation = true;
                let push = (d_min - dist) / 2.0 + 0.5;
                let nx = if dist < 1e-6 { 1.0 } else { dx / dist };
                let ny = if dist < 1e-6 { 0.0 } else { dy / dist };
                let a_anchored = anchors.contains(&ids[i]);
                let b_anchored = anchors.contains(&ids[j]);
                if !a_anchored && !b_anchored {
                    let p1 = positions.get_mut(&ids[i]).unwrap();
                    p1.x -= nx * push;
                    p1.y -= ny * push;
                    let p2 = positions.get_mut(&ids[j]).unwrap();
                    p2.x += nx * push;
                    p2.y += ny * push;
                } else if !a_anchored {
                    let p1 = positions.get_mut(&ids[i]).unwrap();
                    p1.x -= nx * push * 2.0;
                    p1.y -= ny * push * 2.0;
                } else if !b_anchored {
                    let p2 = positions.get_mut(&ids[j]).unwrap();
                    p2.x += nx * push * 2.0;
                    p2.y += ny * push * 2.0;
                }
            }
        }
        if !any_violation { break; }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::types::{Network, Point2D, Station, Edge, Line, GeoPoint};
    use std::collections::{HashMap, HashSet};
    use uuid::Uuid;

    fn make_two_station_net(angle_deg: f64) -> (Network, PositionMap) {
        let a = Uuid::new_v4(); let b = Uuid::new_v4();
        let eid = Uuid::new_v4(); let lid = Uuid::new_v4();
        let rad = angle_deg.to_radians();
        let mut positions = HashMap::new();
        positions.insert(a, Point2D { x: 0.0, y: 0.0 });
        positions.insert(b, Point2D { x: rad.cos() * 100.0, y: rad.sin() * 100.0 });

        let edge = Edge { id: eid, source: a, target: b, line_ids: vec![lid], metadata: HashMap::new() };
        let line = Line { id: lid, name: "L".to_string(), code: None, color: "#f00".to_string(), station_ids: vec![a, b], is_loop: false, metadata: HashMap::new() };
        let net = Network {
            stations: HashMap::from([
                (a, Station { id: a, name: "A".to_string(), geo: GeoPoint { lat: 0.0, lon: 0.0 }, metadata: HashMap::new() }),
                (b, Station { id: b, name: "B".to_string(), geo: GeoPoint { lat: 0.0, lon: 1.0 }, metadata: HashMap::new() }),
            ]),
            edges: HashMap::from([(eid, edge)]),
            lines: HashMap::from([(lid, line)]),
        };
        (net, positions)
    }

    #[test]
    fn angle_snap_fixes_near_45_degree_edge() {
        let (net, mut positions) = make_two_station_net(44.0);
        let anchors: HashSet<Uuid> = HashSet::new();
        angle_snap_pass(&mut positions, &net, &anchors, 1.0, 10);
        let vals: Vec<Point2D> = positions.values().cloned().collect();
        let (p1, p2) = (&vals[0], &vals[1]);
        let dx = p2.x - p1.x; let dy = p2.y - p1.y;
        let angle = dy.atan2(dx).to_degrees();
        let a = ((angle % 360.0) + 360.0) % 360.0;
        let rem = a % 45.0;
        let dev = rem.min(45.0 - rem);
        assert!(dev < 1.5, "After snap, edge should be near octilinear, deviation was {}", dev);
    }

    #[test]
    fn angle_snap_does_not_move_anchored_node() {
        let (net, mut positions) = make_two_station_net(44.0);
        let ids: Vec<Uuid> = positions.keys().copied().collect();
        let anchored_id = ids[0];
        let anchored_pos = positions[&anchored_id].clone();
        let mut anchors = HashSet::new();
        anchors.insert(anchored_id);
        angle_snap_pass(&mut positions, &net, &anchors, 1.0, 10);
        let after = &positions[&anchored_id];
        assert!((after.x - anchored_pos.x).abs() < 0.01 && (after.y - anchored_pos.y).abs() < 0.01,
            "Anchored station should not move");
    }

    #[test]
    fn separation_pass_resolves_overlapping_stations() {
        let a = Uuid::new_v4(); let b = Uuid::new_v4();
        let mut positions = HashMap::new();
        positions.insert(a, Point2D { x: 100.0, y: 100.0 });
        positions.insert(b, Point2D { x: 105.0, y: 100.0 });
        let anchors: HashSet<Uuid> = HashSet::new();
        separation_pass(&mut positions, &anchors, 60.0, 20);
        let p1 = &positions[&a]; let p2 = &positions[&b];
        let dx = p2.x - p1.x; let dy = p2.y - p1.y;
        let dist = (dx*dx + dy*dy).sqrt();
        assert!(dist >= 59.9, "After separation pass, stations should be at least d_min apart, got {}", dist);
    }

    #[test]
    fn separation_pass_does_not_move_anchor() {
        let a = Uuid::new_v4(); let b = Uuid::new_v4();
        let mut positions = HashMap::new();
        positions.insert(a, Point2D { x: 100.0, y: 100.0 });
        positions.insert(b, Point2D { x: 105.0, y: 100.0 });
        let mut anchors = HashSet::new();
        anchors.insert(a);
        let a_pos_before = positions[&a].clone();
        separation_pass(&mut positions, &anchors, 60.0, 20);
        let a_pos_after = &positions[&a];
        assert!((a_pos_after.x - a_pos_before.x).abs() < 0.01, "Anchored station x should not change");
    }
}
