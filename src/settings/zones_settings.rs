// resources/settings/zones

use crate::resources::prelude::SongKey;
use crate::settings::prelude::LanesSettings;
use deathframe::core::components::prelude::Merge;
use replace_with::replace_with_or_abort;
use std::collections::HashMap;

pub type ZoneId = String;
pub type SegmentId = String;

#[derive(Deserialize, Default, Debug)]
#[serde(deny_unknown_fields)]
pub struct ZonesSettings {
    #[serde(default)]
    pub config: ZonesConfig,
    #[serde(default)]
    pub zones: HashMap<ZoneId, ZoneSettings>,
}

#[derive(Deserialize, Default, Debug)]
#[serde(deny_unknown_fields)]
pub struct ZonesConfig {
    pub zone_order: Vec<ZoneId>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ZoneSettings {
    #[serde(default)]
    pub song: Option<SongKey>,
    pub player_speed: f32,
    #[serde(default)]
    pub is_skippable: bool,
    pub total_segments: Option<usize>,
    pub first_segment: Vec<SegmentId>,
    pub final_segment: Vec<SegmentId>,
    pub segments: HashMap<SegmentId, Vec<SegmentId>>,
    #[serde(default)]
    pub lanes: Option<LanesSettings>,
}

impl Merge for ZonesSettings {
    fn merge(&mut self, other: Self) {
        let ZonesSettings {
            config: other_config,
            zones: mut other_zones,
        } = other;
        replace_with_or_abort(self, |self_| {
            let mut zones = self_
                .zones
                .into_iter()
                .map(|(zone_id, zone_settings)| {
                    let merged_zone_settings = if let Some(other_zone) =
                        other_zones.remove(&zone_id)
                    {
                        zone_settings.merged(other_zone)
                    } else {
                        zone_settings
                    };
                    (zone_id, merged_zone_settings)
                })
                .collect::<HashMap<_, _>>();
            zones.extend(other_zones.into_iter());
            ZonesSettings {
                config: self_.config.merged(other_config),
                zones,
            }
        });
    }
}

impl Merge for ZonesConfig {
    fn merge(&mut self, other: Self) {
        let ZonesConfig {
            zone_order: mut other_zone_order,
        } = other;
        if self.zone_order.is_empty() {
            self.zone_order = other_zone_order;
        } else {
            if !other_zone_order.is_empty() {
                eprintln!(
                    "[WARNING]\n    Careful, you have `config.zone_order` \
                     arrays configured in multiple zone configs.\n    This \
                     will merge multiple `config.zone_order` arrays \
                     together.\n    This is probably not intended."
                );
                self.zone_order.append(&mut other_zone_order);
            }
        }
    }
}

impl Merge for ZoneSettings {
    fn merge(&mut self, other: Self) {
        let ZoneSettings {
            song: _,
            player_speed: _,
            is_skippable: _,
            total_segments: _,
            first_segment: _,
            final_segment: _,
            segments: _,
            lanes: _,
        } = other;
        eprintln!(
            "[WARNING]\n    Careful, you have the same `zones.<ZONE-ID>` \
             configured in multiple zone configs.\n    This will NOT merge \
             them together.\n    You should probably find and fix the \
             duplicate configurations."
        );
    }
}
