mod confine_camera_to_first_and_final_segment;
mod control_player;
mod handle_coin_collection;
mod handle_delete_delay;
mod handle_obstacle;
mod handle_parent_delete;
mod handle_segment_loading;
mod handle_streak;
mod handle_turret;
mod handle_zone_select;
mod handle_zone_switch;
mod update_health_ui;
mod update_highscore_ui;
mod update_on_lane;
mod update_rotate;
mod update_score_ui;
mod update_selected_zone_ui;

pub mod prelude {
    pub use super::confine_camera_to_first_and_final_segment::ConfineCameraToFirstAndFinalSegment;
    pub use super::control_player::ControlPlayer;
    pub use super::handle_coin_collection::HandleCoinCollection;
    pub use super::handle_delete_delay::HandleDeleteDelay;
    pub use super::handle_obstacle::HandleObstacle;
    pub use super::handle_parent_delete::HandleParentDelete;
    pub use super::handle_segment_loading::HandleSegmentLoading;
    pub use super::handle_streak::HandleStreak;
    pub use super::handle_turret::HandleTurret;
    pub use super::handle_zone_select::HandleZoneSelect;
    pub use super::handle_zone_switch::HandleZoneSwitch;
    pub use super::update_health_ui::UpdateHealthUi;
    pub use super::update_highscore_ui::UpdateHighscoreUi;
    pub use super::update_on_lane::UpdateOnLane;
    pub use super::update_rotate::UpdateRotate;
    pub use super::update_score_ui::UpdateScoreUi;
    pub use super::update_selected_zone_ui::UpdateSelectedZoneUi;
    pub use deathframe::systems::prelude::*;
}

mod system_prelude {
    pub use crate::components::prelude::*;
    pub use crate::input::prelude::*;
    pub use crate::resources::prelude::*;
    pub use crate::settings::prelude::*;
    pub use deathframe::core::geo::prelude::*;
    pub use deathframe::systems::system_prelude::*;
}
