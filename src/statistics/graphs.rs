use bevy::prelude::*;
use super::collector::StatisticsHistory;

/// Resource to control graph visibility
#[derive(Resource, Default)]
pub struct GraphsVisibility {
    pub show_population: bool,
    pub show_traits: bool,
    pub show_resources: bool,
}

/// Component for UI text elements
#[derive(Component)]
pub struct StatsText;

/// Setup the stats display UI
pub fn setup_stats_ui(mut commands: Commands) {
    // Create a root node for the stats panel
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Plant Evolution Simulator\n"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                StatsText,
            ));
        });

    // Create controls panel
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                bottom: Val::Px(10.0),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(
                    "Controls:\n\
                    WASD/Arrows: Pan camera\n\
                    Right Mouse: Rotate camera\n\
                    Mouse Wheel: Zoom\n\
                    Space/Shift: Move up/down\n\
                    P: Pause/Resume\n\
                    ESC: Quit"
                ),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

/// Update stats display
pub fn update_stats_display_system(
    stats: Res<StatisticsHistory>,
    mut query: Query<&mut Text, With<StatsText>>,
    time: Res<Time>,
) {
    if let Some(latest) = stats.snapshots.last() {
        for mut text in query.iter_mut() {
            **text = format!(
                "Plant Evolution Simulator\n\
                \n\
                Time: {:.1}s\n\
                Population: {}\n\
                Species: {}\n\
                \n\
                Averages:\n\
                Energy: {:.1}\n\
                Age: {:.1}s\n\
                Mass: {:.1} voxels\n\
                Genetic Diversity: {:.3}\n\
                \n\
                Evolution:\n\
                Growth Rate: {:.2}\n\
                Max Height: {:.2}\n\
                Photosynthesis: {:.2}\n\
                \n\
                Total Biomass: {} voxels",
                time.elapsed_secs(),
                latest.population,
                latest.species_count,
                latest.avg_energy,
                latest.avg_age,
                latest.avg_mass,
                latest.genetic_diversity,
                latest.avg_growth_rate,
                latest.avg_height_gene,
                latest.avg_photosynthesis,
                latest.total_biomass
            );
        }
    }
}
