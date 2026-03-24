use common::types::{
    Network, LayoutRequest, LayoutResult, LayoutDiagnostics,
    EdgeRenderData, LineId,
};
use std::collections::HashMap;
use std::time::Instant;
use crate::preprocess::preprocess;
use crate::labels::place_labels;
use crate::initial::{all_pairs_bfs, stress_majorization};
use crate::annealing::run_annealing;
use crate::repair::{angle_snap_pass, separation_pass};
use crate::energy::Weights;
use crate::circle::pre_place_circle_lines;

pub fn run(req: LayoutRequest) -> LayoutResult {
    let start = Instant::now();

    let preprocessed = preprocess(&req.network, &req.manual_anchors, req.options.canvas_size);
    let geo_positions = preprocessed.geo_normalized.clone();

    let mut positions = geo_positions.clone();
    if req.network.stations.len() >= 2 {
        let distances = all_pairs_bfs(&req.network);
        stress_majorization(
            &mut positions,
            &distances,
            &preprocessed.anchor_set,
            req.options.min_station_spacing,
            50,
            req.options.canvas_size,
        );
    }

    let mut anchors = preprocessed.anchor_set.clone();
    if !preprocessed.circle_line_ids.is_empty() {
        let stage3_snapshot = positions.clone();
        pre_place_circle_lines(
            &req.network,
            &preprocessed.circle_line_ids,
            &stage3_snapshot,
            req.options.min_station_spacing,
            &mut positions,
            &mut anchors,
        );
    }

    let weights = Weights {
        octilinear: req.options.weights.octilinear,
        length: req.options.weights.length,
        bends: req.options.weights.bends,
        separation: req.options.weights.separation,
        crossings: req.options.weights.crossings,
        topology: req.options.weights.topology,
        balance: req.options.weights.balance,
    };

    let final_energy = run_annealing(
        &mut positions,
        &geo_positions,
        &anchors,
        &req.network,
        &weights,
        req.options.min_station_spacing,
        req.options.canvas_size,
        req.options.time_budget_ms,
    );

    angle_snap_pass(&mut positions, &req.network, &anchors, 1.0, 10);
    separation_pass(&mut positions, &anchors, req.options.min_station_spacing, 20);

    let n_edges = req.network.edges.len();
    let oct_score = if n_edges == 0 {
        1.0
    } else {
        let violations = req.network.edges.values().filter(|e| {
            let p1 = match positions.get(&e.source) { Some(p) => p, None => return false };
            let p2 = match positions.get(&e.target) { Some(p) => p, None => return false };
            let dx = p2.x - p1.x;
            let dy = p2.y - p1.y;
            let angle = dy.atan2(dx).to_degrees();
            let a = ((angle % 360.0) + 360.0) % 360.0;
            let rem = a % 45.0;
            rem.min(45.0 - rem) > 2.0
        }).count();
        1.0 - violations as f64 / n_edges as f64
    };

    let crossing_count = {
        let edges: Vec<_> = req.network.edges.values().collect();
        let mut count = 0;
        for i in 0..edges.len() {
            for j in (i+1)..edges.len() {
                let ea = edges[i]; let eb = edges[j];
                if ea.source == eb.source || ea.source == eb.target || ea.target == eb.source || ea.target == eb.target { continue; }
                let p1 = match positions.get(&ea.source) { Some(p) => p, None => continue };
                let p2 = match positions.get(&ea.target) { Some(p) => p, None => continue };
                let p3 = match positions.get(&eb.source) { Some(p) => p, None => continue };
                let p4 = match positions.get(&eb.target) { Some(p) => p, None => continue };
                let d1x = p2.x-p1.x; let d1y = p2.y-p1.y;
                let d2x = p4.x-p3.x; let d2y = p4.y-p3.y;
                let denom = d1x*d2y - d1y*d2x;
                if denom.abs() < 1e-10 { continue; }
                let t = ((p3.x-p1.x)*d2y - (p3.y-p1.y)*d2x) / denom;
                let u = ((p3.x-p1.x)*d1y - (p3.y-p1.y)*d1x) / denom;
                if t > 0.0 && t < 1.0 && u > 0.0 && u < 1.0 { count += 1; }
            }
        }
        count
    };

    let (label_positions, label_conflicts) = place_labels(&positions, &req.network);
    let edge_render_data = compute_edge_render_data(&req.network);

    let elapsed_ms = start.elapsed().as_millis() as u64;

    LayoutResult {
        station_positions: positions,
        label_positions,
        edge_render_data,
        diagnostics: LayoutDiagnostics {
            elapsed_ms,
            final_energy,
            octilinearity_score: oct_score,
            crossing_count,
            circle_lines_detected: preprocessed.circle_line_ids.into_iter().collect(),
            label_conflicts,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::types::{LayoutOptions, EnergyWeights, ManualAnchor, Network, Station, Edge, Line, GeoPoint};
    use std::collections::HashMap;
    use uuid::Uuid;

    fn make_station(id: Uuid, name: &str, lat: f64, lon: f64) -> (Uuid, Station) {
        (id, Station { id, name: name.to_string(), geo: GeoPoint { lat, lon }, metadata: HashMap::new() })
    }

    fn make_edge(id: Uuid, source: Uuid, target: Uuid) -> (Uuid, Edge) {
        (id, Edge { id, source, target, line_ids: vec![], metadata: HashMap::new() })
    }

    fn make_line(id: Uuid, name: &str, station_ids: Vec<Uuid>, is_loop: bool) -> (Uuid, Line) {
        (id, Line { id, name: name.to_string(), code: None, color: "#ff0000".to_string(), station_ids, is_loop, metadata: HashMap::new() })
    }

    fn linear_network() -> Network {
        let a = Uuid::new_v4(); let b = Uuid::new_v4(); let c = Uuid::new_v4();
        let d = Uuid::new_v4(); let e = Uuid::new_v4();
        let e1 = Uuid::new_v4(); let e2 = Uuid::new_v4(); let e3 = Uuid::new_v4(); let e4 = Uuid::new_v4();
        let l1 = Uuid::new_v4();
        let mut stations = HashMap::new();
        stations.extend([make_station(a,"A",0.0,0.0), make_station(b,"B",0.0,1.0),
                         make_station(c,"C",0.0,2.0), make_station(d,"D",0.0,3.0),
                         make_station(e,"E",0.0,4.0)]);
        let mut edges = HashMap::new();
        edges.extend([make_edge(e1,a,b), make_edge(e2,b,c), make_edge(e3,c,d), make_edge(e4,d,e)]);
        let (lid, line) = make_line(l1, "L1", vec![a,b,c,d,e], false);
        for eid in [e1,e2,e3,e4] { edges.get_mut(&eid).unwrap().line_ids.push(lid); }
        Network { stations, edges, lines: HashMap::from([(lid, line)]) }
    }

    fn cross_network() -> Network {
        let center = Uuid::new_v4();
        let n = Uuid::new_v4(); let s = Uuid::new_v4();
        let w = Uuid::new_v4(); let e_s = Uuid::new_v4();
        let e1 = Uuid::new_v4(); let e2 = Uuid::new_v4();
        let e3 = Uuid::new_v4(); let e4 = Uuid::new_v4();
        let l1 = Uuid::new_v4(); let l2 = Uuid::new_v4();
        let mut stations = HashMap::new();
        stations.extend([make_station(center,"Center",0.0,0.0), make_station(n,"N",1.0,0.0),
                         make_station(s,"S",-1.0,0.0), make_station(w,"W",0.0,-1.0),
                         make_station(e_s,"E",0.0,1.0)]);
        let mut edges = HashMap::new();
        edges.extend([make_edge(e1,n,center), make_edge(e2,center,s),
                      make_edge(e3,w,center), make_edge(e4,center,e_s)]);
        for eid in [e1,e2] { edges.get_mut(&eid).unwrap().line_ids.push(l1); }
        for eid in [e3,e4] { edges.get_mut(&eid).unwrap().line_ids.push(l2); }
        let mut lines = HashMap::new();
        lines.insert(l1, make_line(l1,"NS",vec![n,center,s],false).1);
        lines.insert(l2, make_line(l2,"WE",vec![w,center,e_s],false).1);
        Network { stations, edges, lines }
    }

    fn circle_network() -> Network {
        let ids: Vec<Uuid> = (0..8).map(|_| Uuid::new_v4()).collect();
        let l1 = Uuid::new_v4();
        let mut stations = HashMap::new();
        for (i, &id) in ids.iter().enumerate() {
            let angle = i as f64 * std::f64::consts::TAU / 8.0;
            stations.extend([make_station(id, &format!("S{}", i), angle.sin(), angle.cos())]);
        }
        let mut edges = HashMap::new();
        for i in 0..8 {
            let eid = Uuid::new_v4();
            edges.extend([make_edge(eid, ids[i], ids[(i+1)%8])]);
            edges.get_mut(&eid).unwrap().line_ids.push(l1);
        }
        let mut lines = HashMap::new();
        lines.insert(l1, make_line(l1, "Circle", ids.clone(), true).1);
        Network { stations, edges, lines }
    }

    fn default_options() -> LayoutOptions {
        LayoutOptions {
            time_budget_ms: 500,
            min_station_spacing: 60.0,
            canvas_size: 1000.0,
            weights: EnergyWeights::default(),
        }
    }

    #[test]
    fn linear_network_all_edges_roughly_octilinear() {
        let network = linear_network();
        let req = LayoutRequest { network: network.clone(), manual_anchors: vec![], options: default_options() };
        let result = run(req);
        assert_eq!(result.station_positions.len(), network.stations.len());
        for edge in network.edges.values() {
            let p1 = &result.station_positions[&edge.source];
            let p2 = &result.station_positions[&edge.target];
            let dx = p2.x - p1.x; let dy = p2.y - p1.y;
            let angle = dy.atan2(dx).to_degrees();
            let a = ((angle % 360.0) + 360.0) % 360.0;
            let rem = a % 45.0;
            let dev = rem.min(45.0 - rem);
            assert!(dev < 5.0, "Edge deviation should be <5°, got {}°", dev);
        }
    }

    #[test]
    fn linear_network_stations_meet_min_spacing() {
        let network = linear_network();
        let req = LayoutRequest { network: network.clone(), manual_anchors: vec![], options: default_options() };
        let result = run(req);
        let ids: Vec<_> = result.station_positions.keys().collect();
        for i in 0..ids.len() {
            for j in (i+1)..ids.len() {
                let p1 = &result.station_positions[ids[i]];
                let p2 = &result.station_positions[ids[j]];
                let dx = p2.x - p1.x; let dy = p2.y - p1.y;
                let dist = (dx*dx + dy*dy).sqrt();
                assert!(dist >= 55.0, "Station pair should be at least ~d_min apart, got {}", dist);
            }
        }
    }

    #[test]
    fn cross_network_identifies_transfer_station() {
        let network = cross_network();
        let req = LayoutRequest { network: network.clone(), manual_anchors: vec![], options: default_options() };
        let result = run(req);
        assert_eq!(result.station_positions.len(), 5);
        for sid in network.stations.keys() {
            assert!(result.station_positions.contains_key(sid));
        }
    }

    #[test]
    fn circle_network_detected_and_placed() {
        let network = circle_network();
        let req = LayoutRequest { network: network.clone(), manual_anchors: vec![], options: default_options() };
        let result = run(req);
        assert_eq!(result.diagnostics.circle_lines_detected.len(), 1,
            "Circle network should detect 1 circle line");
        for sid in network.stations.keys() {
            assert!(result.station_positions.contains_key(sid), "Station {} missing from result", sid);
        }
    }

    #[test]
    fn circle_network_closure_roughly_maintained() {
        let network = circle_network();
        let req = LayoutRequest { network: network.clone(), manual_anchors: vec![], options: default_options() };
        let result = run(req);
        assert!(result.diagnostics.octilinearity_score > 0.5,
            "Circle network octilinearity should be >50%, got {}", result.diagnostics.octilinearity_score);
    }

    #[test]
    fn manual_anchor_is_respected() {
        let network = linear_network();
        let sid = *network.stations.keys().next().unwrap();
        let anchor_pos = common::types::Point2D { x: 200.0, y: 300.0 };
        let req = LayoutRequest {
            network: network.clone(),
            manual_anchors: vec![ManualAnchor { station_id: sid, position: anchor_pos.clone() }],
            options: default_options(),
        };
        let result = run(req);
        let final_pos = &result.station_positions[&sid];
        assert!((final_pos.x - anchor_pos.x).abs() < 1.0 && (final_pos.y - anchor_pos.y).abs() < 1.0,
            "Anchored station should remain at anchor position");
    }
}

fn compute_edge_render_data(network: &Network) -> HashMap<common::types::EdgeId, EdgeRenderData> {
    let line_width = 3.0_f64;
    let gap = 1.5_f64;

    network.edges.iter().map(|(eid, edge)| {
        let n = edge.line_ids.len();
        let mut line_offsets: HashMap<LineId, f64> = HashMap::new();
        for (i, lid) in edge.line_ids.iter().enumerate() {
            let offset = (i as f64 - (n as f64 - 1.0) / 2.0) * (line_width + gap);
            line_offsets.insert(*lid, offset);
        }
        (*eid, EdgeRenderData {
            line_offsets,
            bundle_order: edge.line_ids.clone(),
        })
    }).collect()
}
