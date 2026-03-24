use common::types::{Network, StationId, Point2D};
use std::collections::{HashMap, VecDeque};
use crate::types::{PositionMap, AnchorSet};

pub fn all_pairs_bfs(network: &Network) -> HashMap<(StationId, StationId), f64> {
    let mut distances: HashMap<(StationId, StationId), f64> = HashMap::new();
    let station_ids: Vec<StationId> = network.stations.keys().copied().collect();

    let mut adj: HashMap<StationId, Vec<StationId>> = HashMap::new();
    for id in &station_ids {
        adj.insert(*id, Vec::new());
    }
    for edge in network.edges.values() {
        adj.entry(edge.source).or_default().push(edge.target);
        adj.entry(edge.target).or_default().push(edge.source);
    }

    for &source in &station_ids {
        let mut dist: HashMap<StationId, f64> = HashMap::new();
        dist.insert(source, 0.0);
        let mut queue = VecDeque::new();
        queue.push_back(source);
        while let Some(cur) = queue.pop_front() {
            let cur_dist = dist[&cur];
            for &neighbor in adj.get(&cur).unwrap_or(&Vec::new()) {
                if !dist.contains_key(&neighbor) {
                    dist.insert(neighbor, cur_dist + 1.0);
                    queue.push_back(neighbor);
                }
            }
        }
        for (&target, &d) in &dist {
            if source != target {
                distances.insert((source, target), d);
            }
        }
    }

    distances
}

pub fn stress_majorization(
    positions: &mut PositionMap,
    distances: &HashMap<(StationId, StationId), f64>,
    anchors: &AnchorSet,
    unit_length: f64,
    iterations: u32,
    canvas_size: f64,
) {
    let ids: Vec<StationId> = positions.keys().copied().collect();

    for _ in 0..iterations {
        let old_positions = positions.clone();

        for &i in &ids {
            if anchors.contains(&i) {
                continue;
            }

            let mut num_x = 0.0_f64;
            let mut num_y = 0.0_f64;
            let mut denom = 0.0_f64;

            let pi = &old_positions[&i];

            for &j in &ids {
                if i == j {
                    continue;
                }
                let d_ij = match distances.get(&(i, j)) {
                    Some(&d) if d > 0.0 => d,
                    _ => continue,
                };

                let pj = &old_positions[&j];
                let dx = pi.x - pj.x;
                let dy = pi.y - pj.y;
                let euc = (dx * dx + dy * dy).sqrt().max(1e-6);

                let w_ij = 1.0 / (d_ij * d_ij);
                let ideal = d_ij * unit_length;

                num_x += w_ij * (pj.x + ideal * dx / euc);
                num_y += w_ij * (pj.y + ideal * dy / euc);
                denom += w_ij;
            }

            if denom > 1e-10 {
                let new_x = (num_x / denom).clamp(0.0, canvas_size);
                let new_y = (num_y / denom).clamp(0.0, canvas_size);
                positions.insert(i, Point2D { x: new_x, y: new_y });
            }
        }
    }
}
