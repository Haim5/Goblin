# Goblin

A transit network editor for designing and visualizing metro/rail systems. Draw stations and lines on a real map, then generate a clean schematic diagram.

## Features

- **Map-based editing** — place stations on a real OpenStreetMap base layer
- **Line management** — create lines with custom names and colors, draw edges per line
- **Schematic generation** — automatically generates an octilinear schematic diagram from the geographic layout using simulated annealing

## Tech Stack

- **Frontend** — React, TypeScript, Leaflet
- **Backend** — Rust

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://rustup.rs/) (stable)

### Run

**Terminal 1 — backend:**
```bash
cd core
cargo run -p api
```

**Terminal 2 — frontend:**
```bash
cd frontend
npm install
npm run dev
```

Then open [http://localhost:5173](http://localhost:5173).

## Usage

1. **Add stations** — select the *Add Station* tool and click on the map
2. **Create a line** — click *+ New Line* in the sidebar, give it a name and color
3. **Draw edges** — click *Draw* on a line, then click pairs of stations on the map to connect them. Edges are automatically assigned to that line. Multiple lines can share the same edge.
4. **Generate schematic** — click *Generate Schematic* to produce an octilinear diagram on the right panel
5. **Export** — use the export buttons to save your network as JSON or the diagram as SVG
