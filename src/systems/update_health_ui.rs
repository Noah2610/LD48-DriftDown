use super::system_prelude::*;
use deathframe::amethyst::ui::{UiText, UiTransform};

const UI_HEALTH_ID: &str = "health";

#[derive(Default)]
pub struct UpdateHealthUi;

impl<'a> System<'a> for UpdateHealthUi {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Health>,
        WriteStorage<'a, UiTransform>,
        WriteStorage<'a, UiText>,
    );

    fn run(
        &mut self,
        (player_store, health_store, mut ui_transform_store, mut ui_text_store): Self::SystemData,
    ) {
        if let Some(player_health) = (&player_store, &health_store)
            .join()
            .next()
            .map(|(_, health)| health)
        {
            for ui_text in (&ui_transform_store, &mut ui_text_store)
                .join()
                .filter_map(|(transform, text)| {
                    if &transform.id == UI_HEALTH_ID {
                        Some(text)
                    } else {
                        None
                    }
                })
            {
                ui_text.text = format!(
                    "HEALTH\n{} / {}",
                    player_health.health, player_health.max_health
                );
            }
        }
    }
}
