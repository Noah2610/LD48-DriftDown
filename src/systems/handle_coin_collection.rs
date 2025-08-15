use super::system_prelude::*;
use deathframe::physics::query;
use query::prelude::{FilterQuery, Query};
use std::collections::HashSet;

#[derive(Default)]
pub struct HandleCoinCollection;

impl<'a> System<'a> for HandleCoinCollection {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Streak>,
        ReadExpect<'a, StreakSettings>,
        WriteExpect<'a, Score>,
        WriteExpect<'a, SoundPlayer<SoundKey>>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Coin>,
        ReadStorage<'a, Collider<CollisionTag>>,
    );

    fn run(
        &mut self,
        (
            entities,
            streak,
            streak_settings,
            mut score,
            mut sound_player,
            player_store,
            coin_store,
            collider_store,
        ): Self::SystemData,
    ) {
        let query_exp = {
            use query::exp::prelude_variants::*;
            And(vec![IsState(Steady), IsTag(CollisionTag::Coin)])
        };

        let mut collected_coin_ids = HashSet::new();

        for (_, collider) in (&player_store, &collider_store).join() {
            let collisions = collider
                .query::<FilterQuery<CollisionTag>>()
                .exp(&query_exp)
                .run();
            for collision in collisions {
                collected_coin_ids.insert(collision.id);
            }
        }

        for (coin_entity, _) in (&entities, &coin_store).join() {
            if collected_coin_ids.contains(&coin_entity.id()) {
                let _ = entities.delete(coin_entity);
                if !score.locked {
                    score.coins += 1
                        + (streak.get_secs() * streak_settings.coin_mult)
                            as usize;
                }
                sound_player.add_action(SoundAction::Play(SoundKey::Coin));
            }
        }
    }
}
