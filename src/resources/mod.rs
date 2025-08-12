pub mod prelude {
    pub use super::animation_key::AnimationKey;
    pub use super::collision_tag::{CollisionTag, SolidTag};
    pub use super::dispatcher_id::DispatcherId;
    pub use super::game_over::GameOver;
    pub use super::lanes::{Lane, Lanes, DEFAULT_SEGMENT_WIDTH};
    pub use super::object_spawner::{ObjectSpawner, ObjectToSpawn};
    pub use super::savefile::{Highscore, Highscores, Savefile};
    pub use super::score::Score;
    pub use super::selected_zone::SelectedZone;
    pub use super::should_load_next_zone::ShouldLoadNextZone;
    pub use super::song_key::SongKey;
    pub use super::sound_key::SoundKey;
    pub use super::zone_progression_mode::ZoneProgressionMode;
    pub use super::zone_size::ZoneSize;
    pub use super::zones_manager::ZonesManager;
    pub use deathframe::resources::prelude::*;
}

mod animation_key;
mod collision_tag;
mod dispatcher_id;
mod game_over;
mod lanes;
mod object_spawner;
mod savefile;
mod score;
mod selected_zone;
mod should_load_next_zone;
mod song_key;
mod sound_key;
mod zone_progression_mode;
mod zone_size;
mod zones_manager;
