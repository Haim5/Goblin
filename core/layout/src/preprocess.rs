use common::types::{Network, StationId, LineId, ManualAnchor, Point2D};
use graph::cycles::detect_line_cycles;
use std::collections::{HashMap, HashSet};
use crate::types::{PreprocessedNetwork, AnchorSet, PositionMap};

pub fn preprocess(
    network: &Network,
    manual_anchors: &[ManualAnchor],
    canvas_size: f64,
) -> PreprocessedNetwork {
    let mut degree: HashMap<StationId, usize> = network.stations.keys().map(|id| (*id, 0)).collect();
    for edge in network.edges.values() {
        *degree.entry(edge.source).or_insert(0) += 1;
        *degree.entry(edge.target).or_insert(0) += 1;
    }

    let mut line_membership: HashMap<StationId, Vec<LineId>> = HashMap::new();
    for line in network.lines.values() {
        for sid in &line.station_ids {
            line_membership.entry(*sid).or_default().push(line.id);
        }
    }

    let transfer_stations: HashSet<StationId> = line_membership
        .iter()
        .filter(|(_, lines)| lines.len() >= 2)
        .map(|(sid, _)| *sid)
        .collect();

    let circle_line_ids = detect_line_cycles(network);

    let geo_normalized = normalize_geo(network, canvas_size);

    let mut anchor_set: AnchorSet = HashSet::new();
    let mut anchored_positions: PositionMap = HashMap::new();
    for anchor in manual_anchors {
        if network.stations.contains_key(&anchor.station_id) {
            anchor_set.insert(anchor.station_id);
            anchored_positions.insert(anchor.station_id, anchor.position.clone());
        }
    }

    let mut geo_normalized = geo_normalized;
    for (sid, pos) in anchored_positions {
        geo_normalized.insert(sid, pos);
    }

    PreprocessedNetwork {
        degree,
        line_membership,
        transfer_stations,
        circle_line_ids,
        geo_normalized,
        anchor_set,
    }
}

fn normalize_geo(network: &Network, canvas_size: f64) -> PositionMap {
    if network.stations.is_empty() {
        return HashMap::new();
    }

    let padding = 40.0;
    let usable = canvas_size - 2.0 * padding;

    let lats: Vec<f64> = network.stations.values().map(|s| s.geo.lat).collect();
    let lons: Vec<f64> = network.stations.values().map(|s| s.geo.lon).collect();

    let min_lat = lats.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_lat = lats.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min_lon = lons.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_lon = lons.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    let lat_range = (max_lat - min_lat).max(0.001);
    let lon_range = (max_lon - min_lon).max(0.001);

    let scale = f64::min(usable / lon_range, usable / lat_range);

    let center_lon = (min_lon + max_lon) / 2.0;
    let center_lat = (min_lat + max_lat) / 2.0;

    network.stations.iter().map(|(id, station)| {
        let x = (station.geo.lon - center_lon) * scale + canvas_size / 2.0;
        let y = (center_lat - station.geo.lat) * scale + canvas_size / 2.0;
        (*id, Point2D { x, y })
    }).collect()
}
