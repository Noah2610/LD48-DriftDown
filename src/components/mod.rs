pub mod belongs_to_segment;
pub mod camera;
pub mod coin;
pub mod cutscene;
pub mod delete_delay;
pub mod invincible;
pub mod object;
pub mod obstacle;
pub mod on_lane;
pub mod parent_delete;
pub mod player;
pub mod portal;
pub mod rotate;
pub mod segment;
pub mod tile;
pub mod turret;

pub mod prelude {
    pub use super::belongs_to_segment::BelongsToSegment;
    pub use super::camera::Camera;
    pub use super::coin::Coin;
    pub use super::cutscene::Cutscene;
    pub use super::delete_delay::DeleteDelay;
    pub use super::invincible::Invincible;
    pub use super::object::Object;
    pub use super::obstacle::Obstacle;
    pub use super::on_lane::OnLane;
    pub use super::parent_delete::ParentDelete;
    pub use super::player::Player;
    pub use super::portal::Portal;
    pub use super::rotate::Rotate;
    pub use super::segment::Segment;
    pub use super::tile::Tile;
    pub use super::turret::Turret;
    pub use deathframe::amethyst::core::transform::Parent;
    pub use deathframe::components::prelude::*;
}

mod component_prelude {
    pub use super::prelude::*;
    pub use deathframe::components::component_prelude::*;
}
