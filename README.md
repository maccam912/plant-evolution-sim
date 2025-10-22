# Plant Evolution Simulator

A 3D plant growth and evolution simulation built with Bevy and Rust. Watch plants compete for resources, evolve traits, and differentiate into species through natural selection.

## Features

### Simulation
- **3D Voxel World**: Minecraft-style blocky 3D environment (default 128x128x64)
- **Realistic Plant Biology**:
  - Photosynthesis (sunlight → energy)
  - Root systems for nutrient/water absorption
  - Energy management (growth costs vs maintenance)
  - Death from energy depletion
- **Genetic Evolution**:
  - 8-gene genome controlling traits
  - Mutation during reproduction
  - Species differentiation through genetic drift
  - Natural selection through resource competition

### Genetics System
Plants have 8 genes that control their behavior:
1. **Growth Rate**: How fast the plant grows
2. **Max Height**: Target maximum height
3. **Leaf Density**: Number of leaves (affects photosynthesis)
4. **Root Depth**: How deep roots can grow
5. **Branching Frequency**: How often branches split
6. **Photosynthesis Efficiency**: Energy gained from light
7. **Reproduction Threshold**: Energy needed to reproduce
8. **Mutation Rate**: How much offspring mutate

### Environmental Systems
- **Light Propagation**: Sunlight diminishes with depth, creating vertical competition
- **Soil Resources**: Nutrients and water that regenerate over time
- **Day/Night Cycle**: Affects photosynthesis rates
- **Resource Competition**: Plants deplete nearby soil resources

### Controls
- **WASD / Arrow Keys**: Pan camera
- **Right Mouse Drag**: Rotate camera
- **Mouse Wheel**: Zoom in/out
- **Space / Shift**: Move camera up/down
- **P**: Pause/Resume simulation
- **ESC**: Quit

### Statistics Display
Real-time tracking of:
- Population count and species diversity
- Average energy, age, and biomass
- Genetic diversity metrics
- Evolution of trait averages over time

## Building & Running

```bash
cd plant-evolution-sim
cargo run --release
```

## Configuration

Edit `src/config.rs` to customize simulation parameters:
- World dimensions (requires recompilation)
- Initial seed count
- Growth rates and costs
- Mutation rates
- Resource regeneration
- Camera settings

## Architecture

```
src/
├── main.rs              # Application entry point
├── config.rs            # Configurable parameters
├── world/               # Voxel world and environment
│   ├── voxel.rs         # Voxel types and grid
│   └── environment.rs   # Light, nutrients, water
├── plant/               # Plant systems
│   ├── genetics.rs      # Genome and mutation
│   ├── biology.rs       # Energy and photosynthesis
│   ├── growth.rs        # Growth mechanics
│   └── reproduction.rs  # Seeding and inheritance
├── camera.rs            # Orbital camera controls
├── statistics/          # Data collection and display
│   ├── collector.rs     # Statistics tracking
│   └── graphs.rs        # UI rendering
└── rendering.rs         # 3D visualization
```

## How It Works

1. **Initial Spawn**: Random plants with random genetics
2. **Growth**: Plants extend upward, branch, and grow leaves/roots
3. **Energy Balance**:
   - Gain: Photosynthesis from leaves
   - Cost: Growth and maintenance
4. **Reproduction**: When energy exceeds threshold, spawn seed with mutated genes
5. **Competition**: Taller plants shade shorter ones, roots compete for nutrients
6. **Selection**: Plants with poor energy balance die, successful traits spread
7. **Evolution**: Over generations, traits optimized for survival become dominant

## Expected Emergent Behaviors

- Height competition (tall plants shade competitors)
- Root depth optimization
- Energy efficiency evolution
- Spatial distribution strategies
- Species differentiation in different niches

## Technical Details

- **Engine**: Bevy 0.17 (Rust game engine)
- **Edition**: Rust 2024
- **Dependencies**:
  - `bevy` 0.17 - Game engine
  - `rand` 0.9 - Random number generation
  - `noise` 0.9 - Procedural generation (future use)

## Notes

- The current rendering system shows a simple ground plane. Full voxel rendering would require additional implementation using Bevy's internal mesh APIs or an external voxel library.
- Performance may degrade with large populations (>1000 plants). Consider reducing world size or optimizing update frequencies for better performance.
- Statistics are collected every second by default (configurable in `config.rs`).

## Future Enhancements

Potential additions:
- Full voxel mesh rendering with proper faces
- More complex environmental factors (temperature, seasons)
- Predator-prey dynamics
- Save/load simulation state
- Interactive planting and gene editing
- 3D graphing of evolution over time
- Multi-threaded simulation for better performance

## License

This is a demonstration project. Feel free to use and modify as needed.
