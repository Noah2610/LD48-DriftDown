use crate::resource;
use crate::resources::prelude::*;
use crate::settings::Settings;
use crate::states::aliases::{CustomData, GameDataBuilder};
use amethyst::prelude::Config;
use amethyst::window::DisplayConfig;
use deathframe::amethyst;

pub(super) fn build_game_data<'a, 'b>(
    settings: &Settings,
) -> amethyst::Result<GameDataBuilder<'a, 'b>> {
    use crate::input::prelude::*;
    use crate::systems::prelude::*;
    use amethyst::core::transform::TransformBundle;
    use amethyst::renderer::types::DefaultBackend;
    use amethyst::renderer::{RenderFlat2D, RenderToWindow, RenderingBundle};
    use amethyst::ui::{RenderUi, UiBundle};
    use amethyst::utils::fps_counter::FpsCounterBundle;
    use amethyst::utils::ortho_camera::CameraOrthoSystem;
    use deathframe::bundles::*;

    let transform_bundle = TransformBundle::new();
    let display_config = get_display_config()?;
    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config(display_config)
                .with_clear([0.0, 0.0, 0.0, 1.0]),
        )
        .with_plugin(RenderUi::default())
        .with_plugin(RenderFlat2D::default());
    let audio_bundle = AudioBundle::<SoundKey, SongKey>::default()
        .with_sounds_default_volume(settings.audio.volume);
    let menu_input_bundle = MenuBindings::bundle()?;
    let ingame_input_bundle = IngameBindings::bundle()?;
    let physics_bundle =
        PhysicsBundle::<CollisionTag, SolidTag>::new().with_deps(&[]);
    let animation_bundle = AnimationBundle::<AnimationKey>::new();

    let custom_game_data = GameDataBuilder::default()
        .custom(CustomData::default())
        .dispatcher(DispatcherId::MainMenu)?
        .dispatcher(DispatcherId::Ingame)?
        .dispatcher(DispatcherId::Paused)?
        .dispatcher(DispatcherId::ZoneTransition)?
        .dispatcher(DispatcherId::GameOver)?
        .dispatcher(DispatcherId::ZoneSelect)?
        .with_core_bundle(FpsCounterBundle)?
        .with_core_bundle(transform_bundle)?
        .with_core_bundle(rendering_bundle)?
        .with_core_bundle(audio_bundle)?
        .with_core_bundle(menu_input_bundle)?
        .with_core_bundle(UiBundle::<MenuBindings>::new())?
        .with_core_bundle(animation_bundle)?
        .with_core(PrintFpsSystem::default(), "print_fps_system", &[])?
        .with_core(CameraOrthoSystem::default(), "camera_ortho_system", &[])?
        .with_core(ScaleSpritesSystem::default(), "scale_sprites_system", &[])?
        .with_core(
            InputManagerSystem::<MenuBindings>::default(),
            "menu_input_manager_system",
            &[],
        )?
        .with_bundle(DispatcherId::Ingame, ingame_input_bundle)?
        .with_bundle(DispatcherId::Ingame, physics_bundle)?
        .with(
            DispatcherId::Ingame,
            InputManagerSystem::<IngameBindings>::default(),
            "ingame_input_manager_system",
            &[],
        )?
        .with(
            DispatcherId::Paused,
            InputManagerSystem::<MenuBindings>::default(),
            "paused_input_manager_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            FollowSystem::default(),
            "follow_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            ConfineEntitiesSystem::default(),
            "confine_entities_system",
            &["move_entities_system"],
        )?
        .with(
            DispatcherId::Ingame,
            EntityLoaderSystem::default(),
            "entity_loader_system",
            &[
                "move_entities_system",
                "follow_system",
                "confine_entities_system",
            ],
        )?
        .with(
            DispatcherId::Ingame,
            UpdateHealthSystem::default(),
            "update_health_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            UpdateLifecycleSystem::default(),
            "update_lifecycle_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            ControlPlayer::default(),
            "control_player_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            UpdateOnLane::default(),
            "update_on_lane_system",
            &["control_player_system", "move_entities_system"],
        )?
        .with(
            DispatcherId::Ingame,
            HandleSegmentLoading::default(),
            "handle_segment_loading_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            ConfineCameraToFirstAndFinalSegment::default(),
            "confine_camera_to_first_and_final_segment",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            HandleZoneSwitch::default(),
            "handle_zone_switch_system",
            &["update_collisions_system"],
        )?
        .with(
            DispatcherId::Ingame,
            HandleParentDelete::default(),
            "handle_parent_delete_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            HandleObstacle::default(),
            "handle_obstacle_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            HandleCoinCollection::default(),
            "handle_coin_collection_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            UpdateScoreUi::default(),
            "update_score_ui_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            UpdateHighscoreUi::default(),
            "update_highscore_ui_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            UpdateHealthUi::default(),
            "update_health_ui_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            HandleTurret::default(),
            "handle_turret_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            HandleDeleteDelay::default(),
            "handle_delete_delay_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            UpdateRotate::default(),
            "update_rotate_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            HandleStreak::default(),
            "handle_streak_system",
            &["handle_obstacle_system"],
        )?
        .with(
            DispatcherId::MainMenu,
            UpdateHighscoreUi::default(),
            "main_menu_update_highscore_ui_system",
            &[],
        )?
        .with(
            DispatcherId::GameOver,
            UpdateScoreUi::default(),
            "game_over_update_score_ui_system",
            &[],
        )?
        .with(
            DispatcherId::GameOver,
            UpdateHighscoreUi::default(),
            "game_over_update_highscore_ui_system",
            &[],
        )?
        .with(
            DispatcherId::ZoneSelect,
            UpdateHighscoreUi::default(),
            "zone_select_update_highscore_ui_system",
            &[],
        )?
        .with(
            DispatcherId::GameOver,
            UpdateRotate::default(),
            "game_over_update_rotate_system",
            &[],
        )?
        .with(
            DispatcherId::ZoneSelect,
            HandleZoneSelect::default(),
            "handle_zone_select_system",
            &[],
        )?
        .with(
            DispatcherId::ZoneSelect,
            UpdateSelectedZoneUi::default(),
            "update_selected_zone_ui_system",
            &[],
        )?;

    Ok(custom_game_data)
}

fn get_display_config() -> amethyst::Result<DisplayConfig> {
    #[cfg(not(feature = "dev"))]
    let display_config = DisplayConfig::load(resource("config/display.ron"))?;
    #[cfg(feature = "dev")]
    let display_config = {
        let mut config = DisplayConfig::load(resource("config/display.ron"))?;
        config.min_dimensions = config.dimensions.clone();
        config.max_dimensions = config.dimensions.clone();
        config
    };
    Ok(display_config)
}
