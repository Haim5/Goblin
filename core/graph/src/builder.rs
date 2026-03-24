use common::types::{Network, ValidationError};
use std::collections::HashMap;
use uuid::Uuid;

pub type AdjacencyList = HashMap<Uuid, Vec<(Uuid, Uuid)>>;

pub fn build(network: &Network) -> (AdjacencyList, Vec<ValidationError>) {
    let mut errors = Vec::new();
    let mut adj: AdjacencyList = HashMap::new();

    for station_id in network.stations.keys() {
        adj.insert(*station_id, Vec::new());
    }

    for edge in network.edges.values() {
        if !network.stations.contains_key(&edge.source) {
            errors.push(ValidationError {
                code: "EDGE_MISSING_SOURCE".to_string(),
                message: format!("Edge {} references non-existent station {}", edge.id, edge.source),
                entity_id: Some(edge.id.to_string()),
            });
        }
        if !network.stations.contains_key(&edge.target) {
            errors.push(ValidationError {
                code: "EDGE_MISSING_TARGET".to_string(),
                message: format!("Edge {} references non-existent station {}", edge.id, edge.target),
                entity_id: Some(edge.id.to_string()),
            });
        }

        if network.stations.contains_key(&edge.source) && network.stations.contains_key(&edge.target) {
            let entry = adj.entry(edge.source).or_default();
            entry.push((edge.target, edge.id));
            let entry = adj.entry(edge.target).or_default();
            entry.push((edge.source, edge.id));
        }
    }

    let mut seen_pairs: std::collections::HashSet<(Uuid, Uuid)> = std::collections::HashSet::new();
    for edge in network.edges.values() {
        let pair = if edge.source < edge.target {
            (edge.source, edge.target)
        } else {
            (edge.target, edge.source)
        };
        if !seen_pairs.insert(pair) {
            errors.push(ValidationError {
                code: "DUPLICATE_EDGE".to_string(),
                message: format!("Duplicate edge between {} and {}", edge.source, edge.target),
                entity_id: Some(edge.id.to_string()),
            });
        }
    }

    for line in network.lines.values() {
        for sid in &line.station_ids {
            if !network.stations.contains_key(sid) {
                errors.push(ValidationError {
                    code: "LINE_MISSING_STATION".to_string(),
                    message: format!("Line {} references non-existent station {}", line.id, sid),
                    entity_id: Some(line.id.to_string()),
                });
            }
        }

        if line.station_ids.len() >= 2 {
            for window in line.station_ids.windows(2) {
                let a = window[0];
                let b = window[1];
                let connected = network.edges.values().any(|e| {
                    (e.source == a && e.target == b) || (e.source == b && e.target == a)
                });
                if !connected {
                    errors.push(ValidationError {
                        code: "LINE_PATH_BROKEN".to_string(),
                        message: format!(
                            "Line {} has no edge between {} and {}",
                            line.id, a, b
                        ),
                        entity_id: Some(line.id.to_string()),
                    });
                }
            }

            if line.is_loop {
                let first = line.station_ids[0];
                let last = *line.station_ids.last().unwrap();
                let connected = network.edges.values().any(|e| {
                    (e.source == first && e.target == last) || (e.source == last && e.target == first)
                });
                if !connected {
                    errors.push(ValidationError {
                        code: "LOOP_NOT_CLOSED".to_string(),
                        message: format!("Loop line {} is not closed (no edge between first and last station)", line.id),
                        entity_id: Some(line.id.to_string()),
                    });
                }
            }
        }
    }

    (adj, errors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;
    use uuid::Uuid;

    #[test]
    fn valid_linear_network_has_no_errors() {
        let net = linear_network();
        let (_, errors) = build(&net);
        assert!(errors.is_empty(), "Expected no errors but got: {:?}", errors);
    }

    #[test]
    fn missing_station_in_edge_returns_error() {
        let mut net = linear_network();
        let ghost = Uuid::new_v4();
        let eid = Uuid::new_v4();
        net.edges.extend([make_edge(eid, ghost, *net.stations.keys().next().unwrap())]);
        let (_, errors) = build(&net);
        assert!(errors.iter().any(|e| e.code == "EDGE_MISSING_SOURCE"));
    }

    #[test]
    fn duplicate_edge_returns_error() {
        let mut net = linear_network();
        let ids: Vec<Uuid> = net.stations.keys().copied().collect();
        let (a, b) = (ids[0], ids[1]);
        let existing = net.edges.values().find(|e| {
            (e.source == a && e.target == b) || (e.source == b && e.target == a)
        }).cloned();
        if let Some(existing) = existing {
            let dup_id = Uuid::new_v4();
            net.edges.extend([make_edge(dup_id, existing.source, existing.target)]);
            let (_, errors) = build(&net);
            assert!(errors.iter().any(|e| e.code == "DUPLICATE_EDGE"));
        }
    }

    #[test]
    fn line_with_missing_station_returns_error() {
        let mut net = linear_network();
        let ghost = Uuid::new_v4();
        let lid = *net.lines.keys().next().unwrap();
        net.lines.get_mut(&lid).unwrap().station_ids.push(ghost);
        let (_, errors) = build(&net);
        assert!(errors.iter().any(|e| e.code == "LINE_MISSING_STATION"));
    }

    #[test]
    fn line_with_broken_path_returns_error() {
        let mut net = linear_network();
        let ghost = Uuid::new_v4();
        let (gid, gs) = make_station(ghost, "Ghost", 99.0, 99.0);
        net.stations.insert(gid, gs);
        let lid = *net.lines.keys().next().unwrap();
        net.lines.get_mut(&lid).unwrap().station_ids.push(ghost);
        let (_, errors) = build(&net);
        assert!(errors.iter().any(|e| e.code == "LINE_PATH_BROKEN"));
    }
}
