use super::system_prelude::*;
use deathframe::physics::query;
use query::prelude::{FindQuery, Query};

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
        ): Self::SystemData,
    ) {
        if !game_over.0 {
            let did_player_get_hit = (
                &player_store,
                &collider_store,
                &mut animations_store,
                &mut velocity_store,
            )
                .join()
                .find_map(|(_, collider, animations, velocity)| {
                    let query_exp = {
                        use query::exp::prelude_variants::*;
                        And(vec![
                            IsState(Steady),
                            IsTag(CollisionTag::Obstacle),
                        ])
                    };
                    let collision = collider
                        .query::<FindQuery<CollisionTag>>()
                        .exp(&query_exp)
                        .run();
                    if let Some(collision) = collision {
                        let do_collide = (&entities, &obstacle_store)
                            .join()
                            .any(|(obstacle_entity, _)| {
                                obstacle_entity.id() == collision.id
                            });
                        if do_collide {
                            Some((animations, velocity))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                });

            if let Some((animations, velocity)) = did_player_get_hit {
                game_over.0 = true;
                velocity.x = 0.0;
                velocity.y = 0.0;
                let _ = animations.play(AnimationKey::GameOver);
                sound_player.add_action(SoundAction::Play(SoundKey::GameOver));
            }
        }
    }
}
