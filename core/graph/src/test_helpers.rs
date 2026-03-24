use common::types::{Network, Station, Edge, Line, GeoPoint};
use std::collections::HashMap;
use uuid::Uuid;

pub fn make_station(id: Uuid, name: &str, lat: f64, lon: f64) -> (Uuid, Station) {
    (id, Station {
        id,
        name: name.to_string(),
        geo: GeoPoint { lat, lon },
        metadata: HashMap::new(),
    })
}

pub fn make_edge(id: Uuid, source: Uuid, target: Uuid) -> (Uuid, Edge) {
    (id, Edge {
        id,
        source,
        target,
        line_ids: vec![],
        metadata: HashMap::new(),
    })
}

pub fn make_edge_with_lines(id: Uuid, source: Uuid, target: Uuid, line_ids: Vec<Uuid>) -> (Uuid, Edge) {
    (id, Edge {
        id,
        source,
        target,
        line_ids,
        metadata: HashMap::new(),
    })
}

pub fn make_line(id: Uuid, name: &str, station_ids: Vec<Uuid>, is_loop: bool) -> (Uuid, Line) {
    (id, Line {
        id,
        name: name.to_string(),
        code: None,
        color: "#ff0000".to_string(),
        station_ids,
        is_loop,
        metadata: HashMap::new(),
    })
}

pub fn linear_network() -> Network {
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
    for eid in [e1,e2,e3,e4] {
        edges.get_mut(&eid).unwrap().line_ids.push(lid);
    }

    Network { stations, edges, lines: HashMap::from([(lid, line)]) }
}

pub fn cross_network() -> Network {
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

pub fn circle_network() -> Network {
    let ids: Vec<Uuid> = (0..8).map(|_| Uuid::new_v4()).collect();
    let l1 = Uuid::new_v4();

    let mut stations = HashMap::new();
    for (i, &id) in ids.iter().enumerate() {
        let angle = i as f64 * std::f64::consts::TAU / 8.0;
        stations.extend([make_station(id, &format!("S{}", i), angle.sin(), angle.cos())]);
    }

    let mut edges = HashMap::new();
    let mut edge_ids = Vec::new();
    for i in 0..8 {
        let eid = Uuid::new_v4();
        edges.extend([make_edge(eid, ids[i], ids[(i+1)%8])]);
        edges.get_mut(&eid).unwrap().line_ids.push(l1);
        edge_ids.push(eid);
    }

    let mut lines = HashMap::new();
    lines.insert(l1, make_line(l1, "Circle", ids.clone(), true).1);

    Network { stations, edges, lines }
}
