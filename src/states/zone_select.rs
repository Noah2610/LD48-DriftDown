use super::menu_prelude::*;
use super::state_prelude::*;
use crate::input::prelude::{MenuAction, MenuBindings};

#[derive(Default)]
pub struct ZoneSelect {
    ui_data: UiData,
}

impl ZoneSelect {
    fn start<'a, 'b>(&mut self, data: &mut StateData<GameData<'a, 'b>>) {
        self.create_ui(data, resource("ui/zone_select.ron").to_str().unwrap());
        {
            let mut songs = data.world.write_resource::<Songs<SongKey>>();
            if let Some(song) = songs.get(&SongKey::MainMenu) {
                if !song.is_playing() {
                    songs.stop_all();
                    songs.play(&SongKey::MainMenu)
                }
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
        selected_zone_idx: Option<usize>,
    ) -> Option<Trans<GameData<'a, 'b>, StateEvent>> {
        if input_manager.is_down(MenuAction::Start) {
            if let Some(selected_zone_idx) = selected_zone_idx {
                return Some(Trans::Push(Box::new(
                    Ingame::default()
                        .with_initial_zone_idx(selected_zone_idx)
                        // TODO
                        .with_is_infinite_zone(true),
                )));
            } else {
                return Some(Trans::Push(Box::new(Ingame::default())));
            }
        }
        if input_manager.is_down(MenuAction::Quit) {
            return Some(Trans::Pop);
        }
        None
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for ZoneSelect {
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
            .update(data.world, DispatcherId::ZoneSelect)
            .unwrap();

        if let Some(trans) = self.handle_input(
            &*data.world.read_resource::<InputManager<MenuBindings>>(),
            data.world
                .read_resource::<SelectedZone>()
                .0
                .as_ref()
                .map(|selected| selected.0),
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

impl<'a, 'b> Menu<GameData<'a, 'b>, StateEvent> for ZoneSelect {
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
