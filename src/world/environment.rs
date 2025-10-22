use bevy::prelude::*;
use crate::config::*;
use super::voxel::{VoxelWorld, VoxelPos, VoxelType};

/// System to update light levels in the world
pub fn update_light_system(mut world: ResMut<VoxelWorld>) {
    // Calculate light levels from top to bottom
    for x in 0..world.width() {
        for z in 0..world.depth() {
            let mut light = SUNLIGHT_MAX;

            // Propagate light downward
            for y in (0..world.height()).rev() {
                let pos = VoxelPos::new(x as i32, y as i32, z as i32);

                if let Some(voxel) = world.get_mut(&pos) {
                    voxel.environment.light_level = light;

                    // Reduce light based on material type
                    match voxel.voxel_type {
                        VoxelType::PlantMaterial { .. } => {
                            // Plants block 40% of light (canopy shading)
                            light *= 0.6;
                        }
                        VoxelType::Soil => {
                            // Soil blocks almost all light
                            light *= 0.1;
                        }
                        VoxelType::Air => {
                            // Air doesn't block light
                        }
                    }
                }
            }
        }
    }
}

/// System to regenerate soil nutrients and water
pub fn regenerate_resources_system(mut world: ResMut<VoxelWorld>) {
    for pos in world.iter_positions().collect::<Vec<_>>() {
        if let Some(voxel) = world.get_mut(&pos) {
            if matches!(voxel.voxel_type, VoxelType::Soil) {
                // Regenerate nutrients slowly
                voxel.environment.nutrients = (voxel.environment.nutrients + NUTRIENT_REGEN_RATE)
                    .min(SOIL_NUTRIENT_MAX);

                // Regenerate water faster
                voxel.environment.water = (voxel.environment.water + WATER_REGEN_RATE)
                    .min(SOIL_WATER_MAX);
            }
        }
    }
}

/// Resource to track day/night cycle
#[derive(Resource)]
pub struct DayNightCycle {
    pub time_of_day: f32, // 0.0 to 1.0
    pub day_length: f32,  // seconds
}

impl Default for DayNightCycle {
    fn default() -> Self {
        Self {
            time_of_day: 0.5, // Start at noon
            day_length: 60.0, // 60 second days
        }
    }
}

/// System to update day/night cycle
pub fn update_day_night_system(mut cycle: ResMut<DayNightCycle>, time: Res<Time>) {
    cycle.time_of_day += time.delta_secs() / cycle.day_length;
    cycle.time_of_day %= 1.0;
}

/// Get sunlight multiplier based on time of day
pub fn get_sunlight_multiplier(cycle: &DayNightCycle) -> f32 {
    // Simple sine wave for day/night
    let angle = cycle.time_of_day * std::f32::consts::PI * 2.0;
    (angle.sin() * 0.5 + 0.5).max(0.1) // Minimum 10% light at night
}

/// Resource to track yearly seasonal cycle
#[derive(Resource)]
pub struct YearCycle {
    pub time_of_year: f32, // 0.0 to 1.0
    pub year_length: f32,  // seconds
}

impl Default for YearCycle {
    fn default() -> Self {
        Self {
            time_of_year: 0.0, // Start at spring
            year_length: 300.0, // 5 minute years (300 seconds)
        }
    }
}

/// System to update yearly cycle
pub fn update_year_cycle_system(mut cycle: ResMut<YearCycle>, time: Res<Time>) {
    cycle.time_of_year += time.delta_secs() / cycle.year_length;
    cycle.time_of_year %= 1.0;
}

/// Get seasonal multiplier based on time of year
/// This creates a harsh winter bottleneck where plants must survive on energy reserves
pub fn get_seasonal_multiplier(cycle: &YearCycle) -> f32 {
    // Seasons:
    // 0.00 - 0.25: Spring (growing season) - 80% light
    // 0.25 - 0.50: Summer (peak season) - 100% light
    // 0.50 - 0.75: Autumn (declining) - 60% light
    // 0.75 - 1.00: Winter (harsh bottleneck) - 5% light

    if cycle.time_of_year < 0.25 {
        // Spring: linearly increase from 60% to 100%
        0.6 + (cycle.time_of_year / 0.25) * 0.4
    } else if cycle.time_of_year < 0.50 {
        // Summer: stay at 100%
        1.0
    } else if cycle.time_of_year < 0.75 {
        // Autumn: linearly decrease from 100% to 40%
        let progress = (cycle.time_of_year - 0.50) / 0.25;
        1.0 - (progress * 0.6)
    } else {
        // Winter: harsh bottleneck at 5% (force plants to use reserves)
        0.05
    }
}

/// Get the current season name for display
pub fn get_season_name(cycle: &YearCycle) -> &'static str {
    if cycle.time_of_year < 0.25 {
        "Spring"
    } else if cycle.time_of_year < 0.50 {
        "Summer"
    } else if cycle.time_of_year < 0.75 {
        "Autumn"
    } else {
        "Winter"
    }
}
