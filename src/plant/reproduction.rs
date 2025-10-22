use bevy::prelude::*;
use rand::Rng;
use crate::config::*;
use crate::world::{VoxelWorld, VoxelPos, VoxelType};
use super::biology::{PlantBiology, PlantStructure, GrowthTimer};
use super::genetics::{Genome, GeneticLineage};

/// Tracks the next species ID to assign
#[derive(Resource, Default)]
pub struct SpeciesCounter {
    pub next_id: u32,
}

/// Threshold for genetic distance to be considered a new species
const SPECIES_DIVERGENCE_THRESHOLD: f32 = 0.25;

/// System to handle plant reproduction
pub fn reproduction_system(
    mut commands: Commands,
    mut plants: Query<(Entity, &mut PlantBiology, &PlantStructure, &Genome, &GeneticLineage)>,
    world: Res<VoxelWorld>,
    mut species_counter: ResMut<SpeciesCounter>,
) {
    let mut rng = rand::rng();
    let mut seeds_to_spawn = Vec::new();

    for (entity, mut biology, structure, genome, lineage) in plants.iter_mut() {
        if !biology.is_alive {
            continue;
        }

        let reproduction_threshold = genome.get_reproduction_threshold();

        // Check if plant has enough energy to reproduce
        if biology.energy >= reproduction_threshold {
            // Deduct reproduction cost
            biology.energy -= REPRODUCTION_ENERGY_COST;

            // Find a position for the seed
            if let Some(seed_pos) = find_seed_position(&structure.root_position, &world, &mut rng) {
                // Create offspring genome
                let offspring_genome = genome.reproduce(&mut rng);

                // Calculate genetic distance to determine species
                let genetic_distance = genome.distance(&offspring_genome);
                let offspring_species_id = if genetic_distance > SPECIES_DIVERGENCE_THRESHOLD {
                    // Diverged enough to be a new species
                    let new_species_id = species_counter.next_id;
                    species_counter.next_id += 1;
                    println!("New species {} emerged from species {} (distance: {:.3})",
                             new_species_id, lineage.species_id, genetic_distance);
                    new_species_id
                } else {
                    // Same species as parent
                    lineage.species_id
                };

                seeds_to_spawn.push((
                    seed_pos,
                    offspring_genome,
                    lineage.generation + 1,
                    Some(entity),
                    offspring_species_id,
                ));

                println!(
                    "Plant reproduced! Generation {} -> {}",
                    lineage.generation,
                    lineage.generation + 1
                );
            }
        }
    }

    // Spawn seeds
    for (pos, genome, generation, parent_id, species_id) in seeds_to_spawn {
        spawn_plant(&mut commands, pos, genome, generation, parent_id, species_id);
    }
}

/// Find a valid position to place a seed
fn find_seed_position(
    parent_pos: &VoxelPos,
    world: &VoxelWorld,
    rng: &mut impl Rng,
) -> Option<VoxelPos> {
    // Try random positions within dispersal range
    for _ in 0..20 {
        let offset_x = rng.random_range(-SEED_DISPERSAL_RANGE..=SEED_DISPERSAL_RANGE);
        let offset_z = rng.random_range(-SEED_DISPERSAL_RANGE..=SEED_DISPERSAL_RANGE);

        let candidate = VoxelPos::new(
            parent_pos.x + offset_x,
            parent_pos.y,
            parent_pos.z + offset_z,
        );

        // Check if position is valid (on soil and not occupied)
        if is_valid_seed_position(&candidate, world) {
            return Some(candidate);
        }
    }

    None
}

/// Check if a position is valid for planting a seed
fn is_valid_seed_position(pos: &VoxelPos, world: &VoxelWorld) -> bool {
    // Check if the position is soil
    if let Some(voxel) = world.get(pos) {
        if !matches!(voxel.voxel_type, VoxelType::Soil) {
            return false;
        }

        // Check if there's air above for growth
        let above = VoxelPos::new(pos.x, pos.y + 1, pos.z);
        if let Some(above_voxel) = world.get(&above) {
            return above_voxel.voxel_type.is_air();
        }
    }

    false
}

/// Spawn a new plant
pub fn spawn_plant(
    commands: &mut Commands,
    root_pos: VoxelPos,
    genome: Genome,
    generation: u32,
    parent_id: Option<Entity>,
    species_id: u32,
) {
    commands.spawn((
        PlantBiology::default(),
        PlantStructure::new(root_pos),
        genome,
        GeneticLineage {
            generation,
            parent_id,
            species_id,
        },
        GrowthTimer::default(),
    ));
}

/// System to remove dead plants from the world
pub fn cleanup_dead_plants_system(
    mut commands: Commands,
    dead_plants: Query<(Entity, &PlantBiology, &PlantStructure), Changed<PlantBiology>>,
    mut world: ResMut<VoxelWorld>,
) {
    for (entity, biology, structure) in dead_plants.iter() {
        if !biology.is_alive {
            // Remove plant voxels from world
            for pos in &structure.voxel_positions {
                if let Some(voxel) = world.get_mut(pos) {
                    // Convert to soil (decomposition)
                    voxel.voxel_type = VoxelType::Soil;
                }
            }

            // Despawn entity
            commands.entity(entity).despawn();
        }
    }
}
