use bevy::prelude::*;
use super::collector::StatisticsHistory;

/// Resource to control graph visibility
#[derive(Resource, Default)]
pub struct GraphsVisibility {
    pub show_population: bool,
    pub show_traits: bool,
    pub show_resources: bool,
}

/// Resource to control UI visibility
#[derive(Resource)]
pub struct UIState {
    pub collapsed: bool,
}

impl Default for UIState {
    fn default() -> Self {
        Self { collapsed: false }
    }
}

/// Component for UI text elements
#[derive(Component)]
pub struct StatsText;

/// Component marker for the stats panel
#[derive(Component)]
pub struct StatsPanel;

/// Component marker for the controls panel
#[derive(Component)]
pub struct ControlsPanel;

/// Component marker for the toggle button
#[derive(Component)]
pub struct UIToggleButton;

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
            StatsPanel,
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
            ControlsPanel,
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
                    H: Toggle UI\n\
                    ESC: Quit\n\
                    \n\
                    Touch Controls:\n\
                    1 Finger: Orbit camera\n\
                    2 Fingers Drag: Pan\n\
                    Pinch: Zoom"
                ),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });

    // Create toggle button (always visible)
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(10.0),
                top: Val::Px(10.0),
                width: Val::Px(40.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.9)),
            Button,
            UIToggleButton,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("☰"),
                TextFont {
                    font_size: 24.0,
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

/// Handle UI toggle button clicks
pub fn ui_toggle_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<UIToggleButton>)
    >,
    mut ui_state: ResMut<UIState>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                ui_state.collapsed = !ui_state.collapsed;
                *color = BackgroundColor(Color::srgba(0.4, 0.4, 0.4, 0.9));
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 0.9));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.9));
            }
        }
    }
}

/// Handle keyboard toggle
pub fn ui_keyboard_toggle_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ui_state: ResMut<UIState>,
) {
    if keyboard.just_pressed(KeyCode::KeyH) {
        ui_state.collapsed = !ui_state.collapsed;
    }
}

/// Update panel visibility based on UI state
pub fn update_panel_visibility_system(
    ui_state: Res<UIState>,
    mut stats_query: Query<&mut Visibility, (With<StatsPanel>, Without<ControlsPanel>)>,
    mut controls_query: Query<&mut Visibility, With<ControlsPanel>>,
) {
    let visibility = if ui_state.collapsed {
        Visibility::Hidden
    } else {
        Visibility::Visible
    };

    for mut vis in stats_query.iter_mut() {
        *vis = visibility;
    }

    for mut vis in controls_query.iter_mut() {
        *vis = visibility;
    }
}
