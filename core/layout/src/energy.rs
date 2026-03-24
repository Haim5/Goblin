use common::types::{Network, Point2D};
use crate::types::PositionMap;
use uuid::Uuid;

fn edge_angle(p1: &Point2D, p2: &Point2D) -> f64 {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    dy.atan2(dx).to_degrees()
}

fn nearest_octilinear(angle_deg: f64) -> f64 {
    let a = ((angle_deg % 360.0) + 360.0) % 360.0;
    (a / 45.0).round() * 45.0
}

fn angle_deviation(angle_deg: f64) -> f64 {
    let a = ((angle_deg % 360.0) + 360.0) % 360.0;
    let rem = a % 45.0;
    rem.min(45.0 - rem)
}

pub fn e_oct(positions: &PositionMap, network: &Network) -> f64 {
    let mut total = 0.0;
    for edge in network.edges.values() {
        let p1 = match positions.get(&edge.source) { Some(p) => p, None => continue };
        let p2 = match positions.get(&edge.target) { Some(p) => p, None => continue };
        let angle = edge_angle(p1, p2);
        let dev = angle_deviation(angle);
        total += (dev / 22.5).powi(2);
    }
    total
}

pub fn e_len(positions: &PositionMap, network: &Network, canvas_size: f64) -> f64 {
    let scale = canvas_size * 0.1;
    let mut total = 0.0;
    for edge in network.edges.values() {
        let p1 = match positions.get(&edge.source) { Some(p) => p, None => continue };
        let p2 = match positions.get(&edge.target) { Some(p) => p, None => continue };
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        total += (dx * dx + dy * dy).sqrt() / scale;
    }
    total
}

pub fn e_bends(positions: &PositionMap, network: &Network) -> f64 {
    let mut total = 0.0;
    for line in network.lines.values() {
        if line.station_ids.len() < 3 {
            continue;
        }
        for window in line.station_ids.windows(3) {
            let a = match positions.get(&window[0]) { Some(p) => p, None => continue };
            let b = match positions.get(&window[1]) { Some(p) => p, None => continue };
            let c = match positions.get(&window[2]) { Some(p) => p, None => continue };
            let angle1 = nearest_octilinear(edge_angle(a, b));
            let angle2 = nearest_octilinear(edge_angle(b, c));
            if (angle1 - angle2).abs() > 1.0 && (angle1 - angle2 - 180.0).abs() > 1.0 && (angle1 - angle2 + 180.0).abs() > 1.0 {
                total += 1.0;
            }
        }
    }
    total
}

pub fn e_sep(positions: &PositionMap, network: &Network, d_min: f64) -> f64 {
    let ids: Vec<Uuid> = positions.keys().copied().collect();
    let mut total = 0.0;
    let _ = network;
    for i in 0..ids.len() {
        for j in (i + 1)..ids.len() {
            let p1 = &positions[&ids[i]];
            let p2 = &positions[&ids[j]];
            let dx = p2.x - p1.x;
            let dy = p2.y - p1.y;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist < d_min {
                total += (d_min - dist).powi(2);
            }
        }
    }
    total
}

pub fn e_cross(positions: &PositionMap, network: &Network) -> f64 {
    let edges: Vec<_> = network.edges.values().collect();
    let mut count = 0.0;
    for i in 0..edges.len() {
        for j in (i + 1)..edges.len() {
            let ea = edges[i];
            let eb = edges[j];
            if ea.source == eb.source || ea.source == eb.target || ea.target == eb.source || ea.target == eb.target {
                continue;
            }
            let p1 = match positions.get(&ea.source) { Some(p) => p, None => continue };
            let p2 = match positions.get(&ea.target) { Some(p) => p, None => continue };
            let p3 = match positions.get(&eb.source) { Some(p) => p, None => continue };
            let p4 = match positions.get(&eb.target) { Some(p) => p, None => continue };
            if segments_intersect(p1, p2, p3, p4) {
                count += 1.0;
            }
        }
    }
    count
}

fn segments_intersect(p1: &Point2D, p2: &Point2D, p3: &Point2D, p4: &Point2D) -> bool {
    let d1x = p2.x - p1.x;
    let d1y = p2.y - p1.y;
    let d2x = p4.x - p3.x;
    let d2y = p4.y - p3.y;
    let denom = d1x * d2y - d1y * d2x;
    if denom.abs() < 1e-10 {
        return false;
    }
    let t = ((p3.x - p1.x) * d2y - (p3.y - p1.y) * d2x) / denom;
    let u = ((p3.x - p1.x) * d1y - (p3.y - p1.y) * d1x) / denom;
    t > 0.0 && t < 1.0 && u > 0.0 && u < 1.0
}

pub fn e_topo(positions: &PositionMap, geo_positions: &PositionMap, network: &Network) -> f64 {
    let mut violations = 0.0;
    for edge in network.edges.values() {
        let gs = match geo_positions.get(&edge.source) { Some(p) => p, None => continue };
        let gt = match geo_positions.get(&edge.target) { Some(p) => p, None => continue };
        let ss = match positions.get(&edge.source) { Some(p) => p, None => continue };
        let st = match positions.get(&edge.target) { Some(p) => p, None => continue };
        let geo_goes_right = (gt.x - gs.x).abs() > 1.0;
        let sch_goes_right = (st.x - ss.x).abs() > 1.0;
        if geo_goes_right && sch_goes_right {
            if (gt.x > gs.x) != (st.x > ss.x) {
                violations += 1.0;
            }
        }
        let geo_goes_down = (gt.y - gs.y).abs() > 1.0;
        let sch_goes_down = (st.y - ss.y).abs() > 1.0;
        if geo_goes_down && sch_goes_down {
            if (gt.y > gs.y) != (st.y > ss.y) {
                violations += 1.0;
            }
        }
    }
    violations
}

pub fn e_bal(positions: &PositionMap) -> f64 {
    if positions.len() < 2 {
        return 0.0;
    }
    let n = positions.len() as f64;
    let mean_x = positions.values().map(|p| p.x).sum::<f64>() / n;
    let mean_y = positions.values().map(|p| p.y).sum::<f64>() / n;
    let var_x = positions.values().map(|p| (p.x - mean_x).powi(2)).sum::<f64>() / n;
    let var_y = positions.values().map(|p| (p.y - mean_y).powi(2)).sum::<f64>() / n;
    let spread = (var_x + var_y).sqrt();
    if spread < 50.0 { (50.0 - spread).powi(2) / 2500.0 } else { 0.0 }
}

pub struct Weights {
    pub octilinear: f64,
    pub length: f64,
    pub bends: f64,
    pub separation: f64,
    pub crossings: f64,
    pub topology: f64,
    pub balance: f64,
}

pub fn total_energy(
    positions: &PositionMap,
    geo_positions: &PositionMap,
    network: &Network,
    weights: &Weights,
    d_min: f64,
    canvas_size: f64,
) -> f64 {
    weights.octilinear * e_oct(positions, network)
        + weights.length * e_len(positions, network, canvas_size)
        + weights.bends * e_bends(positions, network)
        + weights.separation * e_sep(positions, network, d_min)
        + weights.crossings * e_cross(positions, network)
        + weights.topology * e_topo(positions, geo_positions, network)
        + weights.balance * e_bal(positions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::types::{Network, Point2D};
    use std::collections::HashMap;
    use uuid::Uuid;

    fn two_station_net(angle_deg: f64) -> (Network, PositionMap) {
        let a = Uuid::new_v4();
        let b = Uuid::new_v4();
        let eid = Uuid::new_v4();
        let lid = Uuid::new_v4();

        let rad = angle_deg.to_radians();
        let mut positions = HashMap::new();
        positions.insert(a, Point2D { x: 0.0, y: 0.0 });
        positions.insert(b, Point2D { x: rad.cos() * 100.0, y: rad.sin() * 100.0 });

        let mut stations = HashMap::new();
        stations.insert(a, common::types::Station {
            id: a, name: "A".to_string(),
            geo: common::types::GeoPoint { lat: 0.0, lon: 0.0 },
            metadata: HashMap::new(),
        });
        stations.insert(b, common::types::Station {
            id: b, name: "B".to_string(),
            geo: common::types::GeoPoint { lat: 0.0, lon: 1.0 },
            metadata: HashMap::new(),
        });

        let edge = common::types::Edge {
            id: eid, source: a, target: b, line_ids: vec![lid], metadata: HashMap::new(),
        };
        let line = common::types::Line {
            id: lid, name: "L".to_string(), code: None, color: "#f00".to_string(),
            station_ids: vec![a, b], is_loop: false, metadata: HashMap::new(),
        };

        let net = Network {
            stations,
            edges: HashMap::from([(eid, edge)]),
            lines: HashMap::from([(lid, line)]),
        };
        (net, positions)
    }

    #[test]
    fn e_oct_zero_for_horizontal_edge() {
        let (net, pos) = two_station_net(0.0);
        let score = e_oct(&pos, &net);
        assert!(score < 0.001, "Horizontal edge should have ~0 octilinear penalty, got {}", score);
    }

    #[test]
    fn e_oct_zero_for_45_degree_edge() {
        let (net, pos) = two_station_net(45.0);
        let score = e_oct(&pos, &net);
        assert!(score < 0.001, "45° edge should have ~0 octilinear penalty, got {}", score);
    }

    #[test]
    fn e_oct_positive_for_22_5_degree_edge() {
        let (net, pos) = two_station_net(22.5);
        let score = e_oct(&pos, &net);
        assert!(score > 0.9, "22.5° edge should have max octilinear penalty, got {}", score);
    }

    #[test]
    fn e_sep_zero_when_stations_far_enough() {
        let (net, pos) = two_station_net(0.0);
        let score = e_sep(&pos, &net, 50.0);
        assert_eq!(score, 0.0, "Stations 100 units apart with d_min=50 should have 0 separation penalty");
    }

    #[test]
    fn e_sep_positive_when_too_close() {
        let (net, pos) = two_station_net(0.0);
        let score = e_sep(&pos, &net, 200.0);
        assert!(score > 0.0, "Stations closer than d_min should have positive separation penalty");
    }

    #[test]
    fn e_bends_zero_for_straight_line() {
        let (net, pos) = two_station_net(0.0);
        let score = e_bends(&pos, &net);
        assert_eq!(score, 0.0, "Two-station line has no bends");
    }
}
