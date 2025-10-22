use bevy::prelude::*;
use rand::Rng;
use rand::prelude::{SliceRandom, IndexedRandom};
use crate::config::*;
use crate::world::{VoxelWorld, VoxelPos, VoxelType};
use super::biology::{PlantBiology, PlantStructure, GrowthTimer};
use super::genetics::Genome;

/// System to handle plant growth
pub fn plant_growth_system(
    mut plants: Query<(Entity, &mut PlantBiology, &mut PlantStructure, &Genome, &mut GrowthTimer)>,
    mut world: ResMut<VoxelWorld>,
    time: Res<Time>,
) {
    let mut rng = rand::rng();

    for (entity, mut biology, mut structure, genome, mut growth_timer) in plants.iter_mut() {
        if !biology.is_alive {
            continue;
        }

        growth_timer.timer.tick(time.delta());

        if !growth_timer.timer.just_finished() {
            continue;
        }

        // Check if plant has enough energy to grow
        if biology.energy < BASE_GROWTH_COST {
            continue;
        }

        // Check if plant has reached max height
        let current_height = structure
            .voxel_positions
            .iter()
            .map(|p| p.y)
            .max()
            .unwrap_or(0);

        if current_height >= genome.get_max_height() + structure.root_position.y {
            continue;
        }

        // Try to grow upward or branch
        let should_branch = rng.random::<f32>() < genome.get_branching_frequency();

        if should_branch {
            // Try to grow a new branch from an existing voxel
            if let Some(&growth_pos) = structure.voxel_positions.choose(&mut rng) {
                try_grow_branch(
                    entity,
                    &mut biology,
                    &mut structure,
                    genome,
                    growth_pos,
                    &mut world,
                    &mut rng,
                );
            }
        } else {
            // Grow upward from the tallest point
            if let Some(&highest_pos) = structure
                .voxel_positions
                .iter()
                .max_by_key(|p| p.y)
            {
                try_grow_upward(
                    entity,
                    &mut biology,
                    &mut structure,
                    genome,
                    highest_pos,
                    &mut world,
                    &mut rng,
                );
            }
        }

        // Try to grow roots
        if rng.random::<f32>() < 0.3 {
            // 30% chance to grow root
            try_grow_root(
                entity,
                &mut biology,
                &mut structure,
                genome,
                &mut world,
                &mut rng,
            );
        }
    }
}

/// Try to grow upward
fn try_grow_upward(
    plant_id: Entity,
    biology: &mut PlantBiology,
    structure: &mut PlantStructure,
    genome: &Genome,
    from_pos: VoxelPos,
    world: &mut VoxelWorld,
    rng: &mut impl Rng,
) {
    let new_pos = VoxelPos::new(from_pos.x, from_pos.y + 1, from_pos.z);

    if can_grow_at(new_pos, world) {
        grow_voxel(plant_id, new_pos, biology, structure, world);

        // Maybe add a leaf
        if rng.random::<f32>() < genome.get_leaf_density() {
            add_leaf(plant_id, new_pos, biology, structure, world, rng);
        }
    }
}

/// Try to grow a branch
fn try_grow_branch(
    plant_id: Entity,
    biology: &mut PlantBiology,
    structure: &mut PlantStructure,
    genome: &Genome,
    from_pos: VoxelPos,
    world: &mut VoxelWorld,
    rng: &mut impl Rng,
) {
    // Try to grow in a random horizontal direction
    let directions = [
        VoxelPos::new(from_pos.x + 1, from_pos.y, from_pos.z),
        VoxelPos::new(from_pos.x - 1, from_pos.y, from_pos.z),
        VoxelPos::new(from_pos.x, from_pos.y, from_pos.z + 1),
        VoxelPos::new(from_pos.x, from_pos.y, from_pos.z - 1),
    ];

    if let Some(&new_pos) = directions.choose(rng) {
        if can_grow_at(new_pos, world) {
            grow_voxel(plant_id, new_pos, biology, structure, world);

            // Higher chance of leaf on branches
            if rng.random::<f32>() < genome.get_leaf_density() * 1.5 {
                add_leaf(plant_id, new_pos, biology, structure, world, rng);
            }
        }
    }
}

/// Try to grow roots downward
fn try_grow_root(
    plant_id: Entity,
    biology: &mut PlantBiology,
    structure: &mut PlantStructure,
    genome: &Genome,
    world: &mut VoxelWorld,
    rng: &mut impl Rng,
) {
    // Find deepest root
    let deepest_root = structure
        .root_positions
        .iter()
        .min_by_key(|p| p.y)
        .copied()
        .unwrap_or(structure.root_position);

    // Check if we can grow deeper
    let max_depth = structure.root_position.y - genome.get_root_depth();
    if deepest_root.y <= max_depth {
        return;
    }

    let new_pos = VoxelPos::new(deepest_root.x, deepest_root.y - 1, deepest_root.z);

    if can_grow_root_at(new_pos, world) {
        grow_voxel(plant_id, new_pos, biology, structure, world);
        structure.root_positions.push(new_pos);
    }
}

/// Add a leaf voxel
fn add_leaf(
    plant_id: Entity,
    pos: VoxelPos,
    biology: &mut PlantBiology,
    structure: &mut PlantStructure,
    world: &mut VoxelWorld,
    rng: &mut impl Rng,
) {
    // Try to place leaf adjacent to the position
    let offsets = [
        VoxelPos::new(1, 0, 0),
        VoxelPos::new(-1, 0, 0),
        VoxelPos::new(0, 1, 0),
        VoxelPos::new(0, 0, 1),
        VoxelPos::new(0, 0, -1),
    ];

    if let Some(&offset) = offsets.choose(rng) {
        let leaf_pos = VoxelPos::new(pos.x + offset.x, pos.y + offset.y, pos.z + offset.z);

        if can_grow_at(leaf_pos, world) {
            grow_voxel(plant_id, leaf_pos, biology, structure, world);
            structure.leaf_positions.push(leaf_pos);
        }
    }
}

/// Check if we can grow at a position
fn can_grow_at(pos: VoxelPos, world: &VoxelWorld) -> bool {
    if let Some(voxel) = world.get(&pos) {
        voxel.voxel_type.is_air()
    } else {
        false
    }
}

/// Check if we can grow a root at a position
fn can_grow_root_at(pos: VoxelPos, world: &VoxelWorld) -> bool {
    if let Some(voxel) = world.get(&pos) {
        matches!(voxel.voxel_type, VoxelType::Soil | VoxelType::Air)
    } else {
        false
    }
}

/// Actually grow a voxel
fn grow_voxel(
    plant_id: Entity,
    pos: VoxelPos,
    biology: &mut PlantBiology,
    structure: &mut PlantStructure,
    world: &mut VoxelWorld,
) {
    // Deduct energy cost
    biology.energy -= BASE_GROWTH_COST;

    // Add to structure
    structure.voxel_positions.push(pos);
    biology.total_mass = structure.voxel_positions.len() as u32;

    // Update world
    if let Some(voxel) = world.get_mut(&pos) {
        voxel.voxel_type = VoxelType::PlantMaterial {
            plant_id: plant_id.index(),
        };
    }
}
