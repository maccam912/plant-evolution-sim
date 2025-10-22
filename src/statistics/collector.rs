use bevy::prelude::*;
use crate::config::*;
use crate::plant::{PlantBiology, Genome, GeneticLineage};
use crate::world::VoxelWorld;

/// Snapshot of simulation statistics at a point in time
#[derive(Debug, Clone)]
pub struct StatsSnapshot {
    pub timestamp: f32,
    pub population: usize,
    pub species_count: usize,
    pub avg_energy: f32,
    pub avg_age: f32,
    pub avg_mass: f32,
    pub genetic_diversity: f32,
    pub avg_growth_rate: f32,
    pub avg_height_gene: f32,
    pub avg_photosynthesis: f32,
    pub total_biomass: u32,
}

/// Resource to store statistics history
#[derive(Resource)]
pub struct StatisticsHistory {
    pub snapshots: Vec<StatsSnapshot>,
    pub update_timer: Timer,
}

impl Default for StatisticsHistory {
    fn default() -> Self {
        Self {
            snapshots: Vec::new(),
            update_timer: Timer::from_seconds(STATS_UPDATE_INTERVAL, TimerMode::Repeating),
        }
    }
}

/// System to collect statistics
pub fn collect_statistics_system(
    mut stats: ResMut<StatisticsHistory>,
    plants: Query<(&PlantBiology, &Genome, &GeneticLineage)>,
    time: Res<Time>,
) {
    stats.update_timer.tick(time.delta());

    if !stats.update_timer.just_finished() {
        return;
    }

    let plant_count = plants.iter().filter(|(b, _, _)| b.is_alive).count();

    if plant_count == 0 {
        return;
    }

    // Collect data
    let mut total_energy = 0.0;
    let mut total_age = 0.0;
    let mut total_mass = 0;
    let mut total_growth_rate = 0.0;
    let mut total_height_gene = 0.0;
    let mut total_photosynthesis = 0.0;
    let mut genomes: Vec<&Genome> = Vec::new();

    for (biology, genome, _) in plants.iter() {
        if !biology.is_alive {
            continue;
        }

        total_energy += biology.energy;
        total_age += biology.age;
        total_mass += biology.total_mass;
        total_growth_rate += genome.growth_rate.value;
        total_height_gene += genome.max_height.value;
        total_photosynthesis += genome.photosynthesis_efficiency.value;
        genomes.push(genome);
    }

    let count = plant_count as f32;

    // Calculate genetic diversity (average pairwise distance)
    let genetic_diversity = calculate_genetic_diversity(&genomes);

    // Count species (simplified - group by genetic similarity)
    let species_count = estimate_species_count(&genomes);

    let snapshot = StatsSnapshot {
        timestamp: time.elapsed_secs(),
        population: plant_count,
        species_count,
        avg_energy: total_energy / count,
        avg_age: total_age / count,
        avg_mass: total_mass as f32 / count,
        genetic_diversity,
        avg_growth_rate: total_growth_rate / count,
        avg_height_gene: total_height_gene / count,
        avg_photosynthesis: total_photosynthesis / count,
        total_biomass: total_mass,
    };

    stats.snapshots.push(snapshot);

    // Keep only recent history
    if stats.snapshots.len() > STATS_HISTORY_SIZE {
        stats.snapshots.remove(0);
    }
}

/// Calculate average genetic diversity
fn calculate_genetic_diversity(genomes: &[&Genome]) -> f32 {
    if genomes.len() < 2 {
        return 0.0;
    }

    let mut total_distance = 0.0;
    let mut comparisons = 0;

    // Sample pairwise distances
    for i in 0..genomes.len().min(50) {
        for j in (i + 1)..genomes.len().min(50) {
            total_distance += genomes[i].distance(genomes[j]);
            comparisons += 1;
        }
    }

    if comparisons > 0 {
        total_distance / comparisons as f32
    } else {
        0.0
    }
}

/// Estimate number of species using genetic clustering
fn estimate_species_count(genomes: &[&Genome]) -> usize {
    if genomes.is_empty() {
        return 0;
    }

    let threshold = 0.15; // Genetic distance threshold for same species
    let mut species = Vec::new();

    for genome in genomes {
        let mut found_species = false;

        for representative in &species {
            if genome.distance(representative) < threshold {
                found_species = true;
                break;
            }
        }

        if !found_species {
            species.push((*genome).clone());
        }
    }

    species.len()
}

/// Resource to track generation stats
#[derive(Resource, Default)]
pub struct GenerationStats {
    pub current_generation: u32,
    pub total_births: u64,
    pub total_deaths: u64,
}
