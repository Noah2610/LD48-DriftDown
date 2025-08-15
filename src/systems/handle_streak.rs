use super::system_prelude::*;
use deathframe::amethyst::ui::{UiText, UiTransform};

const UI_HEALTH_ID: &str = "health";

#[derive(Default)]
pub struct HandleStreak;

impl<'a> System<'a> for HandleStreak {
    type SystemData = (
        ReadExpect<'a, StreakSettings>,
        WriteExpect<'a, Streak>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (streak_settings, mut streak, player_store, mut velocity_store): Self::SystemData,
    ) {
        if streak.is_active {
            streak.update();
        }

        if let Some((_, velocity)) =
            (&player_store, &mut velocity_store).join().next()
        {
            velocity.y = streak.base_player_speed
                - (streak.get_secs() * streak_settings.speed_mult);
        }
    }
}
