use super::menu_prelude::*;
use super::state_prelude::*;
use crate::input::prelude::{MenuAction, MenuBindings};

#[derive(Default)]
pub struct MainMenu {
    ui_data: UiData,
}

impl MainMenu {
    fn start<'a, 'b>(&mut self, data: &mut StateData<GameData<'a, 'b>>) {
        data.world.delete_all();
        self.create_ui(data, resource("ui/main_menu.ron").to_str().unwrap());
        {
            let mut songs = data.world.write_resource::<Songs<SongKey>>();
            if let Some(song) = songs.get(&SongKey::MainMenu) {
                if !song.is_playing() {
                    songs.stop_all();
                    songs.play(&SongKey::MainMenu)
                }
            }
        }

        data.world.insert(SelectedZone::default());
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
        if input_manager.is_down(MenuAction::Start) {
            return Some(Trans::Push(Box::new(Ingame::default())));
        }
        if input_manager.is_down(MenuAction::StartZoneSelect) {
            return Some(Trans::Push(Box::new(ZoneSelect::default())));
        }
        if input_manager.is_down(MenuAction::Quit) {
            return Some(Trans::Quit);
        }
        None
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for MainMenu {
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
            .update(data.world, DispatcherId::MainMenu)
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

impl<'a, 'b> Menu<GameData<'a, 'b>, StateEvent> for MainMenu {
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
