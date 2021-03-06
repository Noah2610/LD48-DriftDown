use super::menu_prelude::*;
use super::state_prelude::*;
use crate::input::prelude::{MenuAction, MenuBindings};

#[derive(Default)]
pub struct GameOverState {
    ui_data: UiData,
}

impl GameOverState {
    fn start<'a, 'b>(&mut self, data: &mut StateData<GameData<'a, 'b>>) {
        self.create_ui(data, resource("ui/game_over.ron").to_str().unwrap());
        self.update_highscore(data.world);
    }

    fn update_highscore(&self, world: &mut World) {
        use deathframe::amethyst::ecs::{Read, ReadExpect, WriteExpect};

        world.exec(
            |(
                score,
                mut savefile,
                savefile_settings,
                selected_zone,
                zone_progression_mode,
            ): (
                ReadExpect<Score>,
                WriteExpect<Savefile>,
                ReadExpect<SavefileSettings>,
                Read<SelectedZone>,
                Read<ZoneProgressionMode>,
            )| {
                match *zone_progression_mode {
                    ZoneProgressionMode::Progression => {
                        savefile.update_highscore_progression(score.coins);
                    }
                    ZoneProgressionMode::Infinite => {
                        if let Some(zone) =
                            selected_zone.0.as_ref().map(|selected| &selected.1)
                        {
                            savefile.update_highscore_infinite(
                                score.coins,
                                zone.clone(),
                            );
                        } else {
                            eprintln!(
                                "[WARNING]\n    Couldn't update infinite \
                                 highscore on game-over because there is no \
                                 selected zone"
                            );
                        }
                    }
                }
                savefile.handle_save(&*savefile_settings);
            },
        );
    }

    fn stop<'a, 'b>(&mut self, data: &mut StateData<GameData<'a, 'b>>) {
        // self.delete_ui(data);
        data.world.delete_all();
        self.ui_data = UiData::default();
    }

    fn handle_input<'a, 'b>(
        &mut self,
        input_manager: &InputManager<MenuBindings>,
    ) -> Option<Trans<GameData<'a, 'b>, StateEvent>> {
        if input_manager.is_down(MenuAction::Start)
            || input_manager.is_down(MenuAction::Quit)
        {
            Some(Trans::Sequence(vec![Trans::Pop, Trans::Pop]))
        } else {
            None
        }
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for GameOverState {
    fn on_start(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.start(&mut data);
    }

    fn on_resume(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.start(&mut data);
    }

    fn on_stop(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.stop(&mut data);
    }

    fn on_pause(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.stop(&mut data);
    }

    fn update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data
            .update(data.world, DispatcherId::GameOver)
            .unwrap();

        if let Some(trans) = self.handle_input(
            &*data.world.read_resource::<InputManager<MenuBindings>>(),
        ) {
            return trans;
        }

        Trans::None
    }

    fn fixed_update(
        &mut self,
        mut data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        if let Some(trans) = self.update_ui_events(&mut data) {
            trans
        } else {
            Trans::None
        }
    }
}

impl<'a, 'b> Menu<GameData<'a, 'b>, StateEvent> for GameOverState {
    fn event_triggered(
        &mut self,
        _data: &mut StateData<GameData<'a, 'b>>,
        event_name: String,
        event: UiEvent,
    ) -> Option<Trans<GameData<'a, 'b>, StateEvent>> {
        if let UiEventType::ClickStop = event.event_type {
            match event_name.as_str() {
                _ => None,
            }
        } else {
            None
        }
    }

    fn ui_data(&self) -> &UiData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UiData {
        &mut self.ui_data
    }
}
