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

        let score = data.world.read_resource::<Score>().coins;
        match data
            .world
            .read_resource::<SavefileSettings>()
            .savefile_path()
        {
            Ok(savefile_path) => {
                let mut savefile = data.world.write_resource::<Savefile>();
                savefile.update_highscore_progression(score);
                if savefile.should_save() {
                    if let Err(e) = savefile.save(savefile_path) {
                        eprintln!("[WARNING]    \n{}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("[WARNING]\n    {}", e);
            }
        }
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
