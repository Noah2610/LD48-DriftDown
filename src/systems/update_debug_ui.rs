use super::system_prelude::*;
use deathframe::amethyst::ui::{UiText, UiTransform};

const UI_DEBUG_ID: &str = "debug_text";

#[derive(Default)]
pub struct UpdateDebugUi;

impl<'a> System<'a> for UpdateDebugUi {
    type SystemData = (
        ReadStorage<'a, UiTransform>,
        WriteStorage<'a, UiText>,
        Option<Read<'a, Streak>>,
    );

    fn run(
        &mut self,
        (ui_transform_store, mut ui_text_store, streak_opt): Self::SystemData,
    ) {
        if let Some((ui_transform, ui_text)) =
            (&ui_transform_store, &mut ui_text_store)
                .join()
                .find(|(transform, _)| transform.id == UI_DEBUG_ID)
        {
            let mut output = String::from("DEBUG");

            let mut addln = |line: &str| {
                if !output.is_empty() {
                    output.push('\n');
                }
                output.push_str(line);
            };

            if let Some(streak) = streak_opt {
                addln(&format!("Streak: {}", streak.get_secs()));
            }

            ui_text.text = output;
        }
    }
}
