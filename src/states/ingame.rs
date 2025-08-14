use super::menu_prelude::*;
use super::state_prelude::*;
use crate::components::prelude::{Size, Transform};
use crate::input::prelude::{MenuAction, MenuBindings};
use crate::level_loader::build_segment;
use crate::level_loader::objects::{build_camera, build_object, build_player};

const UI_SKIP_TEXT_ID: &str = "skip_zone_text";
const UI_SCORE_ID: &str = "score";

pub struct Ingame {
    initial_zone_idx: Option<usize>,
    is_infinite_zone: bool,
    load_new_zone_on_resume: bool,
    ui_data: UiData,
    is_zone_skippable: bool,
}

impl Default for Ingame {
    fn default() -> Self {
        Self {
            initial_zone_idx: None,
            is_infinite_zone: false,
            load_new_zone_on_resume: true,
            ui_data: Default::default(),
            is_zone_skippable: false,
        }
    }
}

impl Ingame {
    pub fn with_initial_zone_idx(mut self, initial_zone_idx: usize) -> Self {
        self.initial_zone_idx = Some(initial_zone_idx);
        self
    }

    pub fn with_is_infinite_zone(mut self, is_infinite_zone: bool) -> Self {
        self.is_infinite_zone = is_infinite_zone;
        self
    }

    fn start<'a, 'b>(&mut self, mut data: &mut StateData<GameData<'a, 'b>>) {
        if self.load_new_zone_on_resume {
            self.load_new_zone_on_resume = false;

            data.world.delete_all();

            self.create_ui(
                &mut data,
                resource("ui/ingame.ron").to_str().unwrap(),
            );

            data.world.insert(ObjectSpawner::default());
            data.world.write_resource::<ZoneSize>().reset();

            self.load_zone(&mut data.world);
        }
    }

    fn load_zone(&mut self, world: &mut World) {
        let mut player_speed_opt = None;

        {
            use deathframe::amethyst::ecs::{ReadExpect, WriteExpect};

            world.exec(
                |(
                    mut zones_manager,
                    settings,
                    mut songs,
                    savefile_settings,
                    mut savefile,
                ): (
                    WriteExpect<ZonesManager>,
                    ReadExpect<ZonesSettings>,
                    WriteExpect<Songs<SongKey>>,
                    ReadExpect<SavefileSettings>,
                    WriteExpect<Savefile>,
                )| {
                    {
                        zones_manager.stage_initial_segments(&settings);
                        player_speed_opt =
                            zones_manager.get_current_player_speed(&settings);
                        if let Some(is_skippable) =
                            zones_manager.is_current_zone_skippable(&settings)
                        {
                            self.is_zone_skippable = is_skippable;
                        }
                        songs.stop_all();
                        if let Some(song_key) =
                            zones_manager.get_current_song(&settings)
                        {
                            songs.play(song_key);
                        }
                    }

                    {
                        if let Some(zone) = zones_manager.current_zone() {
                            savefile.unlock(zone.clone());
                            savefile.handle_save(&*savefile_settings);
                        }
                    }
                },
            );
        }

        {
            world.write_resource::<Score>().locked = self.is_zone_skippable;
        }

        if let Some(player_speed) = player_speed_opt {
            {
                let mut streak = world.write_resource::<Streak>();
                streak.base_player_speed = player_speed;
                streak.restart();
            }

            let mut transform = Transform::default();
            transform.set_translation_xyz(0.0, 64.0, 2.0);
            let size = Size::new(32.0, 32.0);
            let player = build_player(world, transform, size, player_speed);
            let _ = build_camera(
                world,
                Some(player),
                Size::new(DEFAULT_SEGMENT_WIDTH, 0.0),
                None,
            );
        } else {
            eprintln!(
                "[WARNING]\n    No `player_speed` configured for current \
                 zone.\n    Player will NOT be spawned."
            );
        }
    }

    fn handle_input<'a, 'b>(
        &mut self,
        input_manager: &InputManager<MenuBindings>,
    ) -> Option<Trans<GameData<'a, 'b>, StateEvent>> {
        if input_manager.is_down(MenuAction::Quit) {
            return Some(Trans::Pop);
        }
        if input_manager.is_down(MenuAction::Pause) {
            return Some(Trans::Push(Box::new(Pause::default())));
        }
        if self.is_zone_skippable && input_manager.is_down(MenuAction::Start) {
            self.load_new_zone_on_resume = true;
            return Some(Trans::Push(Box::new(ZoneTransition::default())));
        }

        None
    }

    fn handle_load_segments(&self, world: &mut World) {
        let levels_to_load =
            world.write_resource::<ZonesManager>().levels_to_load();
        for (segment_id, level) in levels_to_load {
            build_segment(world, level, segment_id).unwrap();
        }
    }

    fn handle_load_next_zone<'a, 'b>(
        &mut self,
        world: &mut World,
    ) -> Option<Trans<GameData<'a, 'b>, StateEvent>> {
        let mut should_load_next_zone =
            world.write_resource::<ShouldLoadNextZone>();
        if should_load_next_zone.0 {
            self.load_new_zone_on_resume = true;
            should_load_next_zone.0 = false;
            Some(Trans::Push(Box::new(ZoneTransition::default())))
        } else {
            None
        }
    }

    fn handle_game_over<'a, 'b>(
        &self,
        world: &mut World,
    ) -> Option<Trans<GameData<'a, 'b>, StateEvent>> {
        let mut game_over = world.write_resource::<GameOver>();
        if game_over.0 {
            game_over.0 = false;
            Some(Trans::Push(Box::new(GameOverState::default())))
        } else {
            None
        }
    }

    fn handle_spawn_objects(&self, world: &mut World) {
        let objects_to_spawn =
            world.write_resource::<ObjectSpawner>().objects_to_spawn();
        if !objects_to_spawn.is_empty() {
            for object in objects_to_spawn {
                let transform = {
                    let mut transform = Transform::default();
                    transform.set_translation_xyz(
                        object.pos.0,
                        object.pos.1,
                        object.pos.2,
                    );
                    transform
                };
                if let Some(entity_builder) = build_object(
                    world,
                    object.object_type,
                    transform,
                    object.size.map(Into::into),
                ) {
                    entity_builder.build();
                }
            }
        }
    }

    fn handle_skippable_zone(&self, world: &mut World) {
        use deathframe::amethyst::core::HiddenPropagate;
        use deathframe::amethyst::ecs::{
            Entities, Join, ReadStorage, WriteStorage,
        };
        use deathframe::amethyst::ui::UiTransform;

        if self.is_zone_skippable {
            world.exec(
                |(entities, ui_transform_store, mut hidden_propagate_store): (
                    Entities,
                    ReadStorage<UiTransform>,
                    WriteStorage<HiddenPropagate>,
                )| {
                    for (entity, ui_transform) in
                        (&entities, &ui_transform_store).join()
                    {
                        if &ui_transform.id == UI_SKIP_TEXT_ID {
                            let _ = hidden_propagate_store.remove(entity);
                        } else if &ui_transform.id == UI_SCORE_ID {
                            let _ = hidden_propagate_store
                                .insert(entity, HiddenPropagate::new());
                        }
                    }
                },
            );
        }
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Ingame {
    fn on_start(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        let zones_manager = {
            let mut zones_manager = ZonesManager::default();
            if let Some(&initial_zone_idx) = self.initial_zone_idx.as_ref() {
                zones_manager.set_initial_zone_idx(initial_zone_idx);
            }
            zones_manager.set_infinite_zone(self.is_infinite_zone);
            zones_manager
        };

        data.world.insert(ZoneProgressionMode::from_is_infinite(
            zones_manager.is_infinite_zone(),
        ));
        data.world.insert(zones_manager);
        data.world.insert(ZoneSize::default());
        data.world.insert(ShouldLoadNextZone::default());
        data.world.insert(GameOver::default());
        data.world.insert(Score::default());
        data.world.insert(Streak::default());

        {
            let lanes_default =
                Lanes::from(&*data.world.read_resource::<LanesSettings>());
            data.world.insert(lanes_default);
        }

        {
            use deathframe::amethyst::ecs::{ReadExpect, WriteExpect};

            data.world.exec(
                |(mut lanes, mut zones_manager, settings): (
                    WriteExpect<Lanes>,
                    WriteExpect<ZonesManager>,
                    ReadExpect<ZonesSettings>,
                )| {
                    zones_manager.stage_next_zone(&settings);
                    if let Some(Some(lanes_settings)) = zones_manager
                        .current_zone()
                        .and_then(|zone_id| settings.zones.get(zone_id))
                        .map(|zone_settings| &zone_settings.lanes)
                    {
                        lanes.set(lanes_settings);
                    }
                },
            );
        }

        self.start(&mut data);
    }

    fn on_resume(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.start(&mut data);
    }

    fn on_stop(&mut self, data: StateData<GameData<'a, 'b>>) {
        // self.delete_ui(&mut data);
        data.world.delete_all();
        self.ui_data = UiData::default();
    }

    fn on_pause(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        if self.load_new_zone_on_resume {
            self.delete_ui(&mut data);
        }
    }

    fn update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update(data.world, DispatcherId::Ingame).unwrap();

        if let Some(trans) = self.handle_input(
            &*data.world.read_resource::<InputManager<MenuBindings>>(),
        ) {
            return trans;
        }

        Trans::None
    }

    fn fixed_update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        self.handle_load_segments(data.world);

        if let Some(trans) = self.handle_load_next_zone(data.world) {
            return trans;
        }

        if let Some(trans) = self.handle_game_over(data.world) {
            return trans;
        }

        self.handle_spawn_objects(data.world);

        self.handle_skippable_zone(data.world);

        Trans::None
    }
}

impl<'a, 'b> Menu<GameData<'a, 'b>, StateEvent> for Ingame {
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
