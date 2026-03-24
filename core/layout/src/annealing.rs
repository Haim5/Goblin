use common::types::Network;
use crate::types::{PositionMap, AnchorSet};
use crate::energy::{total_energy, Weights};
use crate::operators::{op_translate, op_swap, op_segment_shift, op_reflect};
use std::time::Instant;
use uuid::Uuid;

struct Rng {
    state: u64,
}

impl Rng {
    fn new(seed: u64) -> Self { Self { state: seed } }
    fn next_u64(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }
    fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
    fn next_normal(&mut self) -> f64 {
        let u1 = self.next_f64().max(1e-10);
        let u2 = self.next_f64();
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    }
    fn next_usize(&mut self, n: usize) -> usize {
        (self.next_u64() % n as u64) as usize
    }
}

fn calibrate_t_initial(
    positions: &PositionMap,
    geo_positions: &PositionMap,
    network: &Network,
    weights: &Weights,
    d_min: f64,
    canvas_size: f64,
    rng: &mut Rng,
) -> f64 {
    let ids: Vec<Uuid> = positions.keys().copied().collect();
    if ids.is_empty() { return 1.0; }
    let sigma = canvas_size * 0.05;
    let mut sum_delta = 0.0;
    let mut count = 0;
    let mut pos = positions.clone();
    for _ in 0..100 {
        let id = ids[rng.next_usize(ids.len())];
        let dx = rng.next_normal() * sigma;
        let dy = rng.next_normal() * sigma;
        let e_before = total_energy(&pos, geo_positions, network, weights, d_min, canvas_size);
        if let Some(old) = op_translate(&mut pos, id, dx, dy, canvas_size) {
            let e_after = total_energy(&pos, geo_positions, network, weights, d_min, canvas_size);
            sum_delta += (e_after - e_before).abs();
            count += 1;
            pos.insert(id, old);
        }
    }
    if count == 0 { return 1.0; }
    let avg = sum_delta / count as f64;
    if avg < 1e-10 { 1.0 } else { avg * 1.2 }
}

pub fn run_annealing(
    positions: &mut PositionMap,
    geo_positions: &PositionMap,
    anchors: &AnchorSet,
    network: &Network,
    weights: &Weights,
    d_min: f64,
    canvas_size: f64,
    time_budget_ms: u64,
) -> f64 {
    let ids: Vec<Uuid> = positions.keys().filter(|id| !anchors.contains(id)).copied().collect();
    if ids.is_empty() {
        return total_energy(positions, geo_positions, network, weights, d_min, canvas_size);
    }

    let mut rng = Rng::new(42);
    let t_initial = calibrate_t_initial(positions, geo_positions, network, weights, d_min, canvas_size, &mut rng);
    let t_final = t_initial * 0.001;

    let line_station_seqs: Vec<Vec<Uuid>> = network.lines.values()
        .filter(|l| l.station_ids.len() >= 2)
        .map(|l| l.station_ids.clone())
        .collect();

    let start = Instant::now();
    let mut best_positions = positions.clone();
    let mut best_energy = total_energy(positions, geo_positions, network, weights, d_min, canvas_size);
    let mut current_energy = best_energy;
    let mut iteration = 0u64;
    let mut temperature = t_initial;

    loop {
        iteration += 1;

        if iteration % 500 == 0 {
            let elapsed = start.elapsed().as_millis() as u64;
            if elapsed >= time_budget_ms {
                break;
            }
            let frac = elapsed as f64 / time_budget_ms as f64;
            temperature = t_initial * (t_final / t_initial).powf(frac);
        }

        let op_choice = rng.next_f64();
        let sigma = canvas_size * 0.02 * (temperature / t_initial).sqrt().max(0.01);

        let (delta_e, undo_fn): (f64, Box<dyn FnOnce(&mut PositionMap)>) = if op_choice < 0.5 {
            let id = ids[rng.next_usize(ids.len())];
            let dx = rng.next_normal() * sigma;
            let dy = rng.next_normal() * sigma;
            let e_before = if iteration % 20 == 0 {
                total_energy(positions, geo_positions, network, weights, d_min, canvas_size)
            } else {
                current_energy
            };
            let old = match op_translate(positions, id, dx, dy, canvas_size) {
                Some(p) => p,
                None => continue,
            };
            let e_after = total_energy(positions, geo_positions, network, weights, d_min, canvas_size);
            let de = e_after - e_before;
            (de, Box::new(move |pos: &mut PositionMap| { pos.insert(id, old); }))
        } else if op_choice < 0.7 {
            if ids.len() < 2 { continue; }
            let i = rng.next_usize(ids.len());
            let mut j = rng.next_usize(ids.len() - 1);
            if j >= i { j += 1; }
            let id_a = ids[i];
            let id_b = ids[j];
            let e_before = total_energy(positions, geo_positions, network, weights, d_min, canvas_size);
            op_swap(positions, id_a, id_b);
            let e_after = total_energy(positions, geo_positions, network, weights, d_min, canvas_size);
            let de = e_after - e_before;
            (de, Box::new(move |pos: &mut PositionMap| { op_swap(pos, id_a, id_b); }))
        } else if op_choice < 0.9 && !line_station_seqs.is_empty() {
            let seq = &line_station_seqs[rng.next_usize(line_station_seqs.len())];
            if seq.len() < 2 { continue; }
            let start_idx = rng.next_usize(seq.len());
            let end_idx = start_idx + 1 + rng.next_usize((seq.len() - start_idx).min(4));
            let end_idx = end_idx.min(seq.len());
            let segment: Vec<Uuid> = seq[start_idx..end_idx].to_vec();
            let dx = rng.next_normal() * sigma;
            let dy = rng.next_normal() * sigma;
            let e_before = total_energy(positions, geo_positions, network, weights, d_min, canvas_size);
            let undos = op_segment_shift(positions, &segment, anchors, dx, dy, canvas_size);
            let e_after = total_energy(positions, geo_positions, network, weights, d_min, canvas_size);
            let de = e_after - e_before;
            (de, Box::new(move |pos: &mut PositionMap| {
                for (sid, old_pos) in undos { pos.insert(sid, old_pos); }
            }))
        } else {
            if line_station_seqs.is_empty() { continue; }
            let seq = &line_station_seqs[rng.next_usize(line_station_seqs.len())];
            if seq.len() < 3 { continue; }
            let idx = 1 + rng.next_usize(seq.len() - 2);
            let sid = seq[idx];
            if anchors.contains(&sid) { continue; }
            let na = seq[idx - 1];
            let nb = seq[idx + 1];
            let e_before = total_energy(positions, geo_positions, network, weights, d_min, canvas_size);
            let old = match op_reflect(positions, sid, na, nb, canvas_size) {
                Some(p) => p,
                None => continue,
            };
            let e_after = total_energy(positions, geo_positions, network, weights, d_min, canvas_size);
            let de = e_after - e_before;
            (de, Box::new(move |pos: &mut PositionMap| { pos.insert(sid, old); }))
        };

        let accept = if delta_e <= 0.0 {
            true
        } else if temperature > 1e-10 {
            rng.next_f64() < (-delta_e / temperature).exp()
        } else {
            false
        };

        if accept {
            current_energy += delta_e;
            if current_energy < best_energy {
                best_energy = current_energy;
                best_positions = positions.clone();
            }
        } else {
            undo_fn(positions);
        }
    }

    *positions = best_positions;
    best_energy
}
