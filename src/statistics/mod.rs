pub mod collector;
pub mod graphs;

pub use collector::{StatisticsHistory, StatsSnapshot, GenerationStats, collect_statistics_system};
pub use graphs::{
    GraphsVisibility, StatsText, UIState,
    setup_stats_ui, update_stats_display_system,
    ui_toggle_button_system, ui_keyboard_toggle_system, update_panel_visibility_system,
};
