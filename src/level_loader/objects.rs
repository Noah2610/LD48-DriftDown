use super::data::*;
use super::ObjectType;
use crate::components::prelude::*;
use crate::resource;
use crate::settings::entity_components::add_components_to_entity;
use crate::settings::prelude::*;
use crate::settings::zones_settings::SegmentId;
use amethyst::ecs::{Builder, Entity, EntityBuilder, World, WorldExt};
use deathframe::amethyst;
use deathframe::core::geo::prelude::Axis;
use deathframe::resources::SpriteSheetHandles;
use std::path::PathBuf;

pub fn build_object<'a>(
    world: &'a mut World,
    object_type: ObjectType,
    transform: Transform,
    size_opt: Option<Size>,
) -> Option<EntityBuilder<'a>> {
    match &object_type {
        ObjectType::Player => {
            eprintln!(
                "[WARNING]\n    A `Player` object is placed in the \
                     level!\n    The player is loaded automatically by the \
                     game, don't place them in the levels.\n    The placed \
                     player object will not be loaded."
            );
            // let player_entity = build_player(world, transform, size);
            // let _ = build_camera(world, player_entity);
            None
        }

        object_type => {
            let object_settings = {
                let settings = world.read_resource::<ObjectsSettings>();
                settings.objects.get(object_type).map(|s| s.clone())
            };

            if let Some(object_settings) = object_settings {
                let sprite_render_opt = if let Some(spritesheet) =
                    object_settings.spritesheet.as_ref()
                {
                    let sprite_sheet = world
                        .write_resource::<SpriteSheetHandles<PathBuf>>()
                        .get_or_load(
                            resource(format!("spritesheets/{}", spritesheet)),
                            world,
                        );
                    Some({
                        SpriteRender {
                            sprite_sheet,
                            sprite_number: 0,
                        }
                    })
                } else {
                    None
                };

                let mut entity_builder = world
                    .create_entity()
                    .with(transform)
                    .with(Object::from(object_type.clone()));

                if let Some(size) = size_opt.clone() {
                    entity_builder = entity_builder.with(size);
                }

                if let Some(sprite_render) = sprite_render_opt {
                    entity_builder = entity_builder.with(sprite_render);
                }

                entity_builder = add_components_to_entity(
                    entity_builder,
                    object_settings.components.clone(),
                    size_opt,
                );

                Some(entity_builder)
            } else {
                eprintln!(
                    "[WARNING]\n    No settings for object: {:?}",
                    object_type
                );
                None
            }
        }
    }
}

pub fn build_objects(
    world: &mut World,
    objects: Vec<DataObject>,
    segment: Option<(SegmentId, Entity)>,
    offset_y: f32,
) -> amethyst::Result<()> {
    for object in objects {
        let transform = {
            let mut transform = Transform::default();
            transform.set_translation_x(object.pos.x);
            transform.set_translation_y(-offset_y + object.pos.y);
            if let Some(z) = object.props.get("z").and_then(|val| val.as_f64())
            {
                transform.set_translation_z(z as f32);
            }
            transform
        };
        let size = Size::new(object.size.w, object.size.h);

        match &object.object_type {
            ObjectType::Player => {
                eprintln!(
                    "[WARNING]\n    A `Player` object is placed in the \
                     level!\n    The player is loaded automatically by the \
                     game, don't place them in the levels.\n    The placed \
                     player object will not be loaded."
                );
                // let player_entity = build_player(world, transform, size);
                // let _ = build_camera(world, player_entity);
            }

            object_type => {
                if let Some(mut entity_builder) = build_object(
                    world,
                    object_type.clone(),
                    transform,
                    Some(size),
                ) {
                    if let Some((segment_id, segment_entity)) = segment.as_ref()
                    {
                        entity_builder = entity_builder
                            .with(BelongsToSegment(segment_id.clone()))
                            .with(ParentDelete(*segment_entity));
                    }
                    entity_builder.build();
                }
            }
        }
    }
    Ok(())
}

pub fn build_player(
    world: &mut World,
    mut transform: Transform,
    size: Size,
    player_speed: f32,
) -> Entity {
    let sprite_render = {
        let sprite_sheet = world
            .write_resource::<SpriteSheetHandles<PathBuf>>()
            .get_or_load(resource("spritesheets/player.png"), world);
        SpriteRender {
            sprite_sheet,
            sprite_number: 0,
        }
    };

    let settings = (*world.read_resource::<PlayerSettings>()).clone();

    transform.set_translation_z(settings.z);

    let mut entity_builder = world
        .create_entity()
        .with(transform)
        .with(size.clone())
        .with(Transparent)
        .with(ScaleOnce::default())
        .with(Object::from(ObjectType::Player))
        .with(Player::default())
        .with(Velocity::new(0.0, player_speed))
        .with(sprite_render)
        .with(Invincible::default());

    entity_builder = add_components_to_entity(
        entity_builder,
        settings.components,
        Some(size),
    );

    entity_builder.build()
}

pub fn build_camera(
    world: &mut World,
    player: Option<Entity>,
    level_size: Size,
    camera_size: Option<Size>,
) -> amethyst::Result<()> {
    use amethyst::renderer::Camera as AmethystCamera;
    use amethyst::utils::ortho_camera::{
        CameraNormalizeMode, CameraOrtho, CameraOrthoWorldCoordinates,
    };

    const LOADING_DISTANCE_PADDING: (f32, f32) = (0.0, 16.0);

    let settings = (*world.read_resource::<CameraSettings>()).clone();

    let size = camera_size.unwrap_or(settings.size);

    let camera = AmethystCamera::standard_2d(size.w, size.h);
    let mut camera_ortho =
        CameraOrtho::normalized(CameraNormalizeMode::Contain);
    let half_size = size.half();
    camera_ortho.world_coordinates = CameraOrthoWorldCoordinates {
        top: half_size.h,
        bottom: -half_size.h,
        left: -half_size.w,
        right: half_size.w,
        near: 0.0,
        far: 100.0,
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(
        level_size.w * 0.5,
        level_size.h * 0.5,
        settings.z,
    );

    let loading_distance = (
        size.w * 0.5 + LOADING_DISTANCE_PADDING.0,
        size.h * 0.5 + LOADING_DISTANCE_PADDING.1,
    );

    let mut entity_builder = world
        .create_entity()
        // .with(Confined::from(Rect {
        //     top:    level_size.h,
        //     bottom: 0.0,
        //     left:   0.0,
        //     right:  level_size.w,
        // }))
        .with(transform)
        .with(size)
        .with(camera)
        .with(camera_ortho)
        .with(Camera::default())
        .with(Loader::from(loading_distance));

    if let Some(player) = player {
        entity_builder = entity_builder.with(
            Follow::new(player)
                .with_only_axis(Axis::Y)
                .with_offset(settings.follow_offset),
        );
    }

    entity_builder.build();

    Ok(())
}

pub fn build_segment_collision(
    world: &mut World,
    size: Size,
    segment_id: SegmentId,
    (is_first_segment, is_final_segment): (bool, bool),
    offset_y: f32,
) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(size.w * 0.5, -offset_y + size.h * 0.5, 0.0);
    world
        .create_entity()
        .with(Segment {
            id: segment_id,
            is_first_segment,
            is_final_segment,
        })
        .with(transform)
        .with(size)
        .build()
}
