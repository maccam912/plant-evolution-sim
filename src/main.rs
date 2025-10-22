mod config;
mod world;
mod plant;
mod camera;
mod statistics;
mod rendering;

use bevy::prelude::*;
use rand::Rng;

use config::*;
use world::*;
use plant::*;
use camera::*;
use statistics::*;
use rendering::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Plant Evolution Simulator".to_string(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        // Resources
        .insert_resource(VoxelWorld::new(WORLD_WIDTH, WORLD_HEIGHT, WORLD_DEPTH))
        .insert_resource(DayNightCycle::default())
        .insert_resource(StatisticsHistory::default())
        .insert_resource(GenerationStats::default())
        .insert_resource(GraphsVisibility::default())
        .insert_resource(RenderState::default())
        .insert_resource(SimulationState::default())
        .insert_resource(UIState::default())
        .insert_resource(TouchState::default())
        .insert_resource(SpeciesCounter { next_id: INITIAL_SEED_COUNT as u32 })
        // Startup systems
        .add_systems(Startup, (
            setup_camera,
            setup_rendering,
            setup_stats_ui,
            spawn_initial_plants,
        ))
        // Environment systems
        .add_systems(Update, (
            update_day_night_system,
            update_light_system,
            regenerate_resources_system,
        ).run_if(simulation_running))
        // Plant systems
        .add_systems(Update, (
            plant_growth_system,
            photosynthesis_system,
            resource_absorption_system,
            maintenance_cost_system,
            aging_system,
            reproduction_system,
            cleanup_dead_plants_system,
        ).run_if(simulation_running))
        // Camera systems
        .add_systems(Update, (
            camera_rotation_system,
            camera_zoom_system,
            camera_pan_system,
            camera_touch_system,
        ))
        // Statistics and UI
        .add_systems(Update, (
            collect_statistics_system,
            update_stats_display_system,
            update_world_mesh_system,
            ui_toggle_button_system,
            ui_keyboard_toggle_system,
            update_panel_visibility_system,
        ))
        // Control systems
        .add_systems(Update, pause_system)
        .run();
}

/// Spawn initial plants
fn spawn_initial_plants(mut commands: Commands, world: Res<VoxelWorld>) {
    let mut rng = rand::rng();

    for species_id in 0..INITIAL_SEED_COUNT as u32 {
        // Find a random soil position
        let x = rng.random_range(0..WORLD_WIDTH) as i32;
        let z = rng.random_range(0..WORLD_DEPTH) as i32;
        let y = (WORLD_HEIGHT / 2 - 1) as i32; // Just below surface

        let pos = VoxelPos::new(x, y, z);

        // Check if it's a valid position
        if let Some(voxel) = world.get(&pos) {
            if matches!(voxel.voxel_type, VoxelType::Soil) {
                let genome = Genome::random(&mut rng);
                spawn_plant(&mut commands, pos, genome, 0, None, species_id);
            }
        }
    }

    println!("Spawned {} initial plants", INITIAL_SEED_COUNT);
}

/// Resource to track simulation pause state
#[derive(Resource, Default)]
struct SimulationState {
    paused: bool,
}

/// Condition to check if simulation is running
fn simulation_running(state: Res<SimulationState>) -> bool {
    !state.paused
}

/// System to handle pause/resume
fn pause_system(
    mut state: ResMut<SimulationState>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyP) {
        state.paused = !state.paused;
        if state.paused {
            println!("Simulation paused");
        } else {
            println!("Simulation resumed");
        }
    }
}
