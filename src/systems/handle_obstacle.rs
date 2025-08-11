use super::system_prelude::*;
use deathframe::physics::query;
use query::prelude::{FindQuery, Query};

const IFRAMES: u32 = 120;

#[derive(Default)]
pub struct HandleObstacle;

impl<'a> System<'a> for HandleObstacle {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameOver>,
        WriteExpect<'a, SoundPlayer<SoundKey>>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Obstacle>,
        ReadStorage<'a, Collider<CollisionTag>>,
        WriteStorage<'a, AnimationsContainer<AnimationKey>>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Health>,
        WriteStorage<'a, Invincible>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut game_over,
            mut sound_player,
            player_store,
            obstacle_store,
            collider_store,
            mut animations_store,
            mut velocity_store,
            mut health_store,
            mut invincible_store,
        ): Self::SystemData,
    ) {
        if game_over.0 {
            return;
        }

        if let Some((_, collider, animations, velocity, health, invincible)) = (
            &player_store,
            &collider_store,
            &mut animations_store,
            &mut velocity_store,
            &mut health_store,
            &mut invincible_store,
        )
            .join()
            .next()
        {
            if invincible.is_invincible() {
                invincible.tick();
                if !invincible.is_invincible() {
                    animations.play(AnimationKey::Idle);
                }
                return;
            }

            let mut did_player_get_hit = false;

            let query_exp = {
                use query::exp::prelude_variants::*;
                And(vec![IsState(Steady), IsTag(CollisionTag::Obstacle)])
            };
            let collision = collider
                .query::<FindQuery<CollisionTag>>()
                .exp(&query_exp)
                .run();
            if let Some(collision) = collision {
                did_player_get_hit = (&entities, &obstacle_store).join().any(
                    |(obstacle_entity, _)| obstacle_entity.id() == collision.id,
                );
            }

            if did_player_get_hit {
                health.lose(1);
                invincible.set_iframes(IFRAMES);
                animations.play(AnimationKey::Invincible);

                if !health.is_alive() {
                    game_over.0 = true;
                    velocity.x = 0.0;
                    velocity.y = 0.0;
                    let _ = animations.play(AnimationKey::GameOver);
                    sound_player
                        .add_action(SoundAction::Play(SoundKey::GameOver));
                    return;
                }
            }
        }
    }
}
