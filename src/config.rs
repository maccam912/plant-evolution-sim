/// Configuration constants for the plant evolution simulation
/// These values can be changed and require recompilation

/// World dimensions (in voxels)
pub const WORLD_WIDTH: usize = 64;
pub const WORLD_HEIGHT: usize = 64;
pub const WORLD_DEPTH: usize = 64;

/// Voxel size (meters)
pub const VOXEL_SIZE: f32 = 0.5;

/// Initial number of seed plants
pub const INITIAL_SEED_COUNT: usize = 10;

/// Simulation parameters
pub const TICKS_PER_SECOND: f32 = 10.0;
pub const SUNLIGHT_MAX: f32 = 100.0; // Maximum sunlight value at surface
pub const SUNLIGHT_FALLOFF: f32 = 0.9; // Light reduction per voxel layer downward

/// Soil parameters
pub const SOIL_NUTRIENT_MAX: f32 = 100.0;
pub const SOIL_WATER_MAX: f32 = 100.0;
pub const NUTRIENT_REGEN_RATE: f32 = 0.1; // Per tick
pub const WATER_REGEN_RATE: f32 = 0.2; // Per tick

/// Plant growth parameters
pub const BASE_GROWTH_COST: f32 = 10.0; // Energy cost per new voxel
pub const BASE_MAINTENANCE_COST: f32 = 0.3; // Energy per voxel per tick (increased for more plant death)
pub const PHOTOSYNTHESIS_EFFICIENCY: f32 = 0.5; // Energy per light per tick
pub const ROOT_ABSORPTION_RATE: f32 = 1.0; // Resource absorption per tick

/// Reproduction parameters
pub const MIN_REPRODUCTION_ENERGY: f32 = 100.0;
pub const REPRODUCTION_ENERGY_COST: f32 = 50.0;
pub const SEED_DISPERSAL_RANGE: i32 = 5; // Voxels from parent

/// Evolution parameters
pub const MUTATION_RATE: f32 = 0.05; // Base probability of mutation per gene
pub const MUTATION_STRENGTH: f32 = 0.1; // Max percentage change from mutation

/// Statistics collection
pub const STATS_UPDATE_INTERVAL: f32 = 1.0; // Seconds between stat updates
pub const STATS_HISTORY_SIZE: usize = 1000; // Number of data points to keep

/// UI parameters
pub const CAMERA_MOVE_SPEED: f32 = 50.0;
pub const CAMERA_ROTATE_SPEED: f32 = 2.0;
pub const CAMERA_ZOOM_SPEED: f32 = 10.0;
pub const CAMERA_INITIAL_DISTANCE: f32 = 100.0;

/// Rendering parameters
pub const CHUNK_SIZE: usize = 16; // Voxels per chunk dimension
