use bevy::prelude::*;
use crate::config::*;
use crate::world::{VoxelWorld, VoxelPos, VoxelType};
use super::genetics::Genome;

/// Component to track plant's energy and state
#[derive(Component, Debug)]
pub struct PlantBiology {
    pub energy: f32,
    pub age: f32, // In seconds
    pub is_alive: bool,
    pub total_mass: u32, // Number of voxels
}

impl Default for PlantBiology {
    fn default() -> Self {
        Self {
            energy: 50.0, // Starting energy
            age: 0.0,
            is_alive: true,
            total_mass: 0,
        }
    }
}

/// Component to track plant structure in the world
#[derive(Component, Debug)]
pub struct PlantStructure {
    pub root_position: VoxelPos,
    pub voxel_positions: Vec<VoxelPos>,
    pub leaf_positions: Vec<VoxelPos>,
    pub root_positions: Vec<VoxelPos>,
}

impl PlantStructure {
    pub fn new(root: VoxelPos) -> Self {
        Self {
            root_position: root,
            voxel_positions: vec![root],
            leaf_positions: Vec::new(),
            root_positions: vec![root],
        }
    }
}

/// System to perform photosynthesis for all plants
pub fn photosynthesis_system(
    mut plants: Query<(&mut PlantBiology, &PlantStructure, &Genome)>,
    world: Res<VoxelWorld>,
    time: Res<Time>,
) {
    for (mut biology, structure, genome) in plants.iter_mut() {
        if !biology.is_alive {
            continue;
        }

        let mut total_energy_gain = 0.0;

        // Calculate energy from each leaf
        for leaf_pos in &structure.leaf_positions {
            if let Some(voxel) = world.get(leaf_pos) {
                let light = voxel.environment.light_level;
                let efficiency = genome.get_photosynthesis_efficiency();
                total_energy_gain += light * PHOTOSYNTHESIS_EFFICIENCY * efficiency * time.delta_secs();
            }
        }

        biology.energy += total_energy_gain;
    }
}

/// System to consume resources from soil through roots
pub fn resource_absorption_system(
    mut plants: Query<(&mut PlantBiology, &PlantStructure)>,
    mut world: ResMut<VoxelWorld>,
    time: Res<Time>,
) {
    for (mut biology, structure) in plants.iter_mut() {
        if !biology.is_alive {
            continue;
        }

        let mut nutrients_absorbed = 0.0;
        let mut water_absorbed = 0.0;

        // Absorb from each root position
        for root_pos in &structure.root_positions {
            if let Some(voxel) = world.get_mut(root_pos) {
                let absorption_rate = ROOT_ABSORPTION_RATE * time.delta_secs();

                // Try to absorb nutrients
                let nutrients = absorption_rate.min(voxel.environment.nutrients);
                voxel.environment.nutrients -= nutrients;
                nutrients_absorbed += nutrients;

                // Try to absorb water
                let water = absorption_rate.min(voxel.environment.water);
                voxel.environment.water -= water;
                water_absorbed += water;
            }
        }

        // Convert resources to energy (simplified)
        biology.energy += (nutrients_absorbed + water_absorbed) * 0.1;
    }
}

/// System to consume energy for maintenance
pub fn maintenance_cost_system(
    mut plants: Query<(&mut PlantBiology, &PlantStructure)>,
    time: Res<Time>,
) {
    for (mut biology, structure) in plants.iter_mut() {
        if !biology.is_alive {
            continue;
        }

        // Calculate base maintenance cost
        let base_maintenance = structure.voxel_positions.len() as f32
            * BASE_MAINTENANCE_COST
            * time.delta_secs();

        // Add gravity-based transport cost - higher voxels cost more energy
        let root_height = structure.root_position.y;
        let mut gravity_cost = 0.0;
        for voxel_pos in &structure.voxel_positions {
            // Height difference from root (in voxels)
            let height_diff = (voxel_pos.y - root_height).max(0) as f32;
            // Energy cost increases with height (0.01 energy per voxel per unit height)
            gravity_cost += height_diff * 0.01 * time.delta_secs();
        }

        let total_maintenance = base_maintenance + gravity_cost;
        biology.energy -= total_maintenance;

        // Check if plant dies from lack of energy
        if biology.energy <= 0.0 {
            biology.is_alive = false;
            println!("Plant died at age {:.1} seconds", biology.age);
        }
    }
}

/// System to age plants
pub fn aging_system(mut plants: Query<&mut PlantBiology>, time: Res<Time>) {
    for mut biology in plants.iter_mut() {
        if biology.is_alive {
            biology.age += time.delta_secs();
        }
    }
}

/// Component to mark a plant for growth
#[derive(Component)]
pub struct GrowthTimer {
    pub timer: Timer,
}

impl Default for GrowthTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        }
    }
}
