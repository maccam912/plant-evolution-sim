pub mod genetics;
pub mod biology;
pub mod growth;
pub mod reproduction;

pub use genetics::{Gene, Genome, GeneticLineage};
pub use biology::{PlantBiology, PlantStructure, GrowthTimer, photosynthesis_system,
                 resource_absorption_system, maintenance_cost_system, aging_system};
pub use growth::plant_growth_system;
pub use reproduction::{reproduction_system, spawn_plant, cleanup_dead_plants_system, SpeciesCounter};
