pub mod collector;
pub mod graphs;

pub use collector::{StatisticsHistory, StatsSnapshot, GenerationStats, collect_statistics_system};
pub use graphs::{GraphsVisibility, StatsText, setup_stats_ui, update_stats_display_system};
