use super::hitbox_config::HitboxConfig;
use crate::components::prelude::*;
use crate::resources::prelude::{AnimationKey, CollisionTag, SolidTag};
use amethyst::ecs::{Builder, EntityBuilder};
use deathframe::amethyst;

pub type EntityComponents = Vec<EntityComponent>;

#[derive(Deserialize, Clone)]
pub enum EntityComponent {
    Velocity(Velocity),
    Size(Size),
    Gravity(Gravity),
    Animation(Animation),
    Animations(AnimationsContainer<AnimationKey>),
    BaseFriction(BaseFriction),
    Hitbox(HitboxConfig),
    Collider(Collider<CollisionTag>),
    Collidable(Collidable<CollisionTag>),
    Solid(Solid<SolidTag>),
    SolidPusher(SolidPusher),
    SolidPushable(SolidPushable),
    ScaleOnce(ScaleOnce),
    OnLane(OnLane),
    Portal(Portal),
    Obstacle(Obstacle),
    Coin(Coin),
    Turret(Turret),
    Loadable(Loadable),
    DeleteDelay(DeleteDelay),
    Rotate(Rotate),
    Cutscene(Cutscene),
    Health(Health),
}

pub fn add_components_to_entity(
    entity_builder: EntityBuilder,
    components: Vec<EntityComponent>,
    mut size_opt: Option<Size>,
) -> EntityBuilder {
    use self::EntityComponent as Comp;

    components
        .into_iter()
        .fold(entity_builder, |builder, component| match component {
            Comp::Velocity(velocity) => builder.with(velocity),
            Comp::Size(size) => {
                size_opt = Some(size.clone());
                builder.with(size)
            }
            Comp::Gravity(gravity) => builder.with(gravity),
            Comp::Animation(mut animation) => {
                animation.play_cycle();
                builder.with(animation)
            }
            Comp::Animations(mut animations) => {
                let _ = animations.play(AnimationKey::Idle);
                builder.with(animations)
            }
            Comp::BaseFriction(base_friction) => builder.with(base_friction),
            Comp::Hitbox(hitbox) => {
                hitbox.add_hitbox_to_entity(builder, size_opt.as_ref())
            }
            Comp::Collider(collider) => builder.with(collider),
            Comp::Collidable(collidable) => builder.with(collidable),
            Comp::Solid(solid) => builder.with(solid),
            Comp::SolidPusher(solid_pusher) => builder.with(solid_pusher),
            Comp::SolidPushable(solid_pushable) => builder.with(solid_pushable),
            Comp::ScaleOnce(scale_once) => builder.with(scale_once),
            Comp::OnLane(on_lane) => builder.with(on_lane),
            Comp::Portal(portal) => builder.with(portal),
            Comp::Obstacle(obstacle) => builder.with(obstacle),
            Comp::Coin(coin) => builder.with(coin),
            Comp::Turret(turret) => builder.with(turret),
            Comp::Loadable(loadable) => {
                builder.with(loadable).with(Unloaded::default())
            }
            Comp::DeleteDelay(delete_delay) => builder.with(delete_delay),
            Comp::Rotate(rotate) => builder.with(rotate),
            Comp::Cutscene(cutscene) => builder.with(cutscene),
            Comp::Health(health) => builder.with(health),
        })
}
