use common::types::{Network, LineId};
use std::collections::{HashSet, HashMap};
use uuid::Uuid;

pub fn detect_line_cycles(network: &Network) -> HashSet<LineId> {
    let mut circle_lines = HashSet::new();

    for line in network.lines.values() {
        if line.is_loop {
            circle_lines.insert(line.id);
            continue;
        }

        if line.station_ids.len() >= 3 {
            let first = line.station_ids[0];
            let last = *line.station_ids.last().unwrap();
            if first != last {
                let has_closing_edge = network.edges.values().any(|e| {
                    (e.source == first && e.target == last)
                        || (e.source == last && e.target == first)
                });
                if has_closing_edge {
                    circle_lines.insert(line.id);
                    continue;
                }
            }
        }

        if line.station_ids.len() >= 3 {
            let station_set: HashSet<Uuid> = line.station_ids.iter().copied().collect();
            let mut adj: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
            for &sid in &line.station_ids {
                adj.insert(sid, Vec::new());
            }
            for edge in network.edges.values() {
                if station_set.contains(&edge.source) && station_set.contains(&edge.target) {
                    adj.entry(edge.source).or_default().push(edge.target);
                    adj.entry(edge.target).or_default().push(edge.source);
                }
            }
            if has_cycle_dfs(&adj, &line.station_ids[0]) {
                circle_lines.insert(line.id);
            }
        }
    }

    circle_lines
}

fn has_cycle_dfs(adj: &HashMap<Uuid, Vec<Uuid>>, start: &Uuid) -> bool {
    let mut visited: HashSet<Uuid> = HashSet::new();
    let mut stack: Vec<(Uuid, Option<Uuid>)> = vec![(*start, None)];

    while let Some((node, parent)) = stack.pop() {
        if visited.contains(&node) {
            return true;
        }
        visited.insert(node);
        if let Some(neighbors) = adj.get(&node) {
            for &neighbor in neighbors {
                if Some(neighbor) != parent {
                    stack.push((neighbor, Some(node)));
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;

    #[test]
    fn path_graph_has_no_cycles() {
        let net = linear_network();
        let cycles = detect_line_cycles(&net);
        assert!(cycles.is_empty(), "Linear network should have no cycles");
    }

    #[test]
    fn explicit_loop_is_detected() {
        let net = circle_network();
        let cycles = detect_line_cycles(&net);
        assert_eq!(cycles.len(), 1);
    }

    #[test]
    fn structural_loop_detected_without_flag() {
        let mut net = circle_network();
        let lid = *net.lines.keys().next().unwrap();
        net.lines.get_mut(&lid).unwrap().is_loop = false;
        let cycles = detect_line_cycles(&net);
        assert_eq!(cycles.len(), 1, "Structural check should detect the cycle even without is_loop=true");
    }

    #[test]
    fn cross_network_has_no_cycles() {
        let net = cross_network();
        let cycles = detect_line_cycles(&net);
        assert!(cycles.is_empty());
    }
}
