use bevy::prelude::*;
use rand::Rng;
use crate::config::*;

/// Individual gene that controls a plant trait
#[derive(Debug, Clone, Copy)]
pub struct Gene {
    pub value: f32, // Normalized value 0.0 to 1.0
}

impl Gene {
    pub fn new(value: f32) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
        }
    }

    pub fn random(rng: &mut impl Rng) -> Self {
        Self::new(rng.random())
    }

    /// Mutate the gene by a small random amount
    pub fn mutate(&mut self, rng: &mut impl Rng) {
        if rng.random::<f32>() < MUTATION_RATE {
            let change = (rng.random::<f32>() - 0.5) * MUTATION_STRENGTH * 2.0;
            self.value = (self.value + change).clamp(0.0, 1.0);
        }
    }
}

/// Complete genome for a plant
#[derive(Component, Debug, Clone)]
pub struct Genome {
    pub growth_rate: Gene,              // How fast the plant grows
    pub max_height: Gene,               // Maximum height target
    pub leaf_density: Gene,             // How many leaves per branch
    pub root_depth: Gene,               // How deep roots can go
    pub branching_frequency: Gene,      // How often branches split
    pub photosynthesis_efficiency: Gene, // Energy gained from light
    pub reproduction_threshold: Gene,   // Energy needed to reproduce
    pub mutation_rate: Gene,            // How much offspring mutate
    pub horizontal_growth_tendency: Gene, // Preference for horizontal vs vertical growth
}

impl Genome {
    /// Create a random genome
    pub fn random(rng: &mut impl Rng) -> Self {
        Self {
            growth_rate: Gene::random(rng),
            max_height: Gene::random(rng),
            leaf_density: Gene::random(rng),
            root_depth: Gene::random(rng),
            branching_frequency: Gene::random(rng),
            photosynthesis_efficiency: Gene::random(rng),
            reproduction_threshold: Gene::random(rng),
            mutation_rate: Gene::random(rng),
            horizontal_growth_tendency: Gene::random(rng),
        }
    }

    /// Create offspring genome with mutations
    pub fn reproduce(&self, rng: &mut impl Rng) -> Self {
        let mut child = self.clone();

        child.growth_rate.mutate(rng);
        child.max_height.mutate(rng);
        child.leaf_density.mutate(rng);
        child.root_depth.mutate(rng);
        child.branching_frequency.mutate(rng);
        child.photosynthesis_efficiency.mutate(rng);
        child.reproduction_threshold.mutate(rng);
        child.horizontal_growth_tendency.mutate(rng);

        // Mutation rate itself can mutate, but less frequently
        if rng.random::<f32>() < MUTATION_RATE * 0.5 {
            child.mutation_rate.mutate(rng);
        }

        child
    }

    /// Calculate genetic distance from another genome (for species tracking)
    pub fn distance(&self, other: &Genome) -> f32 {
        let diff_sum = (self.growth_rate.value - other.growth_rate.value).abs()
            + (self.max_height.value - other.max_height.value).abs()
            + (self.leaf_density.value - other.leaf_density.value).abs()
            + (self.root_depth.value - other.root_depth.value).abs()
            + (self.branching_frequency.value - other.branching_frequency.value).abs()
            + (self.photosynthesis_efficiency.value - other.photosynthesis_efficiency.value).abs()
            + (self.reproduction_threshold.value - other.reproduction_threshold.value).abs()
            + (self.mutation_rate.value - other.mutation_rate.value).abs()
            + (self.horizontal_growth_tendency.value - other.horizontal_growth_tendency.value).abs();

        diff_sum / 9.0 // Average difference
    }

    /// Get actual values from normalized genes
    pub fn get_growth_rate(&self) -> f32 {
        // 0.1 to 2.0 blocks per second
        0.1 + self.growth_rate.value * 1.9
    }

    pub fn get_max_height(&self) -> i32 {
        // 5 to 50 blocks
        (5.0 + self.max_height.value * 45.0) as i32
    }

    pub fn get_leaf_density(&self) -> f32 {
        // 0.1 to 1.0 (probability of leaf growth)
        0.1 + self.leaf_density.value * 0.9
    }

    pub fn get_root_depth(&self) -> i32 {
        // 1 to 20 blocks
        (1.0 + self.root_depth.value * 19.0) as i32
    }

    pub fn get_branching_frequency(&self) -> f32 {
        // 0.01 to 0.3 (probability per growth tick)
        0.01 + self.branching_frequency.value * 0.29
    }

    pub fn get_photosynthesis_efficiency(&self) -> f32 {
        // 0.3 to 1.5 multiplier
        0.3 + self.photosynthesis_efficiency.value * 1.2
    }

    pub fn get_reproduction_threshold(&self) -> f32 {
        // 50 to 500 energy
        50.0 + self.reproduction_threshold.value * 450.0
    }

    pub fn get_mutation_rate(&self) -> f32 {
        // 0.01 to 0.2 per gene
        0.01 + self.mutation_rate.value * 0.19
    }

    pub fn get_horizontal_growth_tendency(&self) -> f32 {
        // 0.0 to 1.0 (0 = vertical only, 1 = horizontal only, 0.5 = balanced)
        self.horizontal_growth_tendency.value
    }
}

/// Component to track genetic lineage
#[derive(Component, Debug)]
pub struct GeneticLineage {
    pub generation: u32,
    pub parent_id: Option<Entity>,
    pub species_id: u32, // Calculated based on genetic similarity
}

impl Default for GeneticLineage {
    fn default() -> Self {
        Self {
            generation: 0,
            parent_id: None,
            species_id: 0,
        }
    }
}
