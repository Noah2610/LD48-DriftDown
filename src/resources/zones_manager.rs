use crate::level_loader::data::DataLevel;
use crate::level_loader::load_level;
use crate::resource;
use crate::resources::prelude::SongKey;
use crate::settings::zones_settings::{SegmentId, ZoneId, ZonesSettings};
use rand::prelude::SliceRandom;
use replace_with::replace_with_or_abort;
use std::collections::HashMap;

const KEEP_COUNT_SEGMENTS_LOADED: usize = 2;

#[derive(Default)]
pub struct ZonesManager {
    current_zone:           Option<ZoneState>,
    initial_zone_idx:       Option<usize>,
    is_infinite_zone:       bool,
    last_staged_segment:    Option<SegmentId>,
    staged_segments:        Vec<SegmentId>,
    levels:                 HashMap<SegmentId, DataLevel>,
    segment_loading_locked: bool,
}

#[derive(Debug)]
struct ZoneState {
    pub id:                    ZoneId,
    pub order_idx:             usize,
    pub total_segments_loaded: usize,
}

impl ZoneState {
    pub fn new(id: ZoneId, order_idx: usize) -> Self {
        Self {
            id,
            order_idx,
            total_segments_loaded: 0,
        }
    }
}

impl ZonesManager {
    pub fn set_initial_zone_idx(&mut self, initial_zone_idx: usize) {
        self.initial_zone_idx = Some(initial_zone_idx);
    }

    pub fn set_infinite_zone(&mut self, is_infinite_zone: bool) {
        self.is_infinite_zone = is_infinite_zone;
    }

    pub fn is_infinite_zone(&self) -> bool {
        self.is_infinite_zone
    }

    pub fn current_zone(&self) -> Option<&ZoneId> {
        self.current_zone.as_ref().map(|current| &current.id)
    }

    pub fn lock_segment_loading(&mut self) {
        self.segment_loading_locked = true;
    }

    pub fn levels_to_load(&mut self) -> Vec<(SegmentId, DataLevel)> {
        self.staged_segments
            .split_off(0)
            .into_iter()
            .filter_map(|segment| {
                self.get_level_or_load(segment.clone())
                    .map(|level| (segment, level))
            })
            .collect()
    }

    fn get_level_or_load(&mut self, segment: SegmentId) -> Option<DataLevel> {
        self.levels.get(&segment).map(Clone::clone).or_else(
            || match load_level(resource(format!("levels/zones/{}", &segment)))
            {
                Ok(level) => {
                    self.levels.insert(segment, level.clone());
                    Some(level)
                }
                Err(e) => {
                    eprintln!(
                        "[WARNING]\n    Failed parsing level JSON file:\n{:#?}",
                        e
                    );
                    None
                }
            },
        )
    }

    fn load_all_segments(&mut self, settings: &ZonesSettings) {
        if let Some(segments) = self
            .current_zone
            .as_ref()
            .and_then(|zone| settings.zones.get(&zone.id))
            .map(|zone| {
                let mut segments = zone
                    .segments
                    .keys()
                    .map(Clone::clone)
                    .collect::<Vec<SegmentId>>();
                segments.append(&mut zone.first_segment.clone());
                segments.append(&mut zone.final_segment.clone());
                segments
            })
        {
            for segment in segments {
                self.load_segment(segment);
            }
        } else {
            eprintln!(
                "[WARNING]\n    Can't load segments if there is no current \
                 zone"
            );
        }
    }

    fn load_segment(&mut self, segment: SegmentId) {
        if !self.levels.contains_key(&segment) {
            match load_level(resource(format!("levels/zones/{}", &segment))) {
                Ok(level) => {
                    self.levels.insert(segment, level);
                }
                Err(e) => {
                    eprintln!(
                        "[WARNING]\n    Couldn't load segment {}!\n{}",
                        &segment, e
                    )
                }
            }
        }
    }

    pub fn stage_next_zone(&mut self, settings: &ZonesSettings) {
        if let Some((next_zone, next_order_idx)) = self.get_next_zone(settings)
        {
            self.reset();
            self.current_zone = Some(ZoneState::new(next_zone, next_order_idx));
            self.load_all_segments(settings);
        } else {
            eprintln!("[WARNING]\n    There is no next zone to load!");
        }
    }

    fn get_next_zone(
        &self,
        settings: &ZonesSettings,
    ) -> Option<(ZoneId, usize)> {
        if let Some(current_zone) = self.current_zone.as_ref() {
            let order_idx = current_zone.order_idx + 1;
            settings
                .config
                .zone_order
                .get(order_idx)
                .map(|next| (next, order_idx))
        } else {
            if let Some(&initial_zone_idx) = self.initial_zone_idx.as_ref() {
                settings
                    .config
                    .zone_order
                    .get(initial_zone_idx)
                    .map(|initial_zone| (initial_zone, initial_zone_idx))
            } else {
                settings.config.zone_order.first().map(|first| (first, 0))
            }
        }
        .map(|(next, order_idx)| (next.clone(), order_idx))
    }

    fn reset(&mut self) {
        replace_with_or_abort(self, |self_| ZonesManager {
            current_zone:           None,
            initial_zone_idx:       self_.initial_zone_idx,
            is_infinite_zone:       self_.is_infinite_zone,
            last_staged_segment:    None,
            staged_segments:        Vec::new(),
            levels:                 HashMap::new(),
            segment_loading_locked: false,
        });
    }

    pub fn stage_initial_segments(&mut self, settings: &ZonesSettings) {
        for _ in 0 .. KEEP_COUNT_SEGMENTS_LOADED {
            self.stage_next_segment(settings);
        }
    }

    pub fn stage_next_segment(&mut self, settings: &ZonesSettings) {
        if let Some(next_segment) = self.get_next_segment(settings) {
            self.current_zone
                .as_mut()
                .expect(
                    "Should have current zone, if next segment could be found",
                )
                .total_segments_loaded += 1;
            self.last_staged_segment = Some(next_segment.clone());
            self.staged_segments.push(next_segment);
        } else {
            self.last_staged_segment = None;
            eprintln!(
                "[WARNING]\n    ZonesManager couldn't find possible next \
                 segment, for zone and segment: {:?} {:?}",
                &self.current_zone, &self.last_staged_segment
            );
        }
    }

    fn get_next_segment(&self, settings: &ZonesSettings) -> Option<SegmentId> {
        if self.segment_loading_locked {
            return None;
        }

        let mut rng = rand::thread_rng();
        self.current_zone
            .as_ref()
            .and_then(|current_zone| {
                settings
                    .zones
                    .get(&current_zone.id)
                    .map(|settings| (current_zone, settings))
            })
            .map(|(current_zone, zone_settings)| {
                let should_load_next_zone = zone_settings
                    .total_segments
                    .map(|total_segments| {
                        self.is_infinite_zone
                            || current_zone.total_segments_loaded
                                < total_segments
                    })
                    .unwrap_or(true);
                if should_load_next_zone {
                    self.last_staged_segment
                        .as_ref()
                        .and_then(|current_segment| {
                            zone_settings.segments.get(current_segment)
                        })
                        .unwrap_or(&zone_settings.first_segment)
                } else {
                    &zone_settings.final_segment
                }
            })
            .and_then(|possible_segments| {
                possible_segments.choose(&mut rng).map(ToString::to_string)
            })
    }

    pub fn get_current_song<'a>(
        &self,
        settings: &'a ZonesSettings,
    ) -> Option<&'a SongKey> {
        self.current_zone
            .as_ref()
            .and_then(|current_zone| settings.zones.get(&current_zone.id))
            .and_then(|zone_settings| zone_settings.song.as_ref())
    }

    pub fn get_current_player_speed(
        &self,
        settings: &ZonesSettings,
    ) -> Option<f32> {
        self.current_zone
            .as_ref()
            .and_then(|current_zone| settings.zones.get(&current_zone.id))
            .map(|zone_settings| zone_settings.player_speed)
    }

    pub fn is_current_zone_skippable(
        &self,
        settings: &ZonesSettings,
    ) -> Option<bool> {
        if self.is_infinite_zone {
            Some(false)
        } else {
            self.current_zone
                .as_ref()
                .and_then(|current_zone| settings.zones.get(&current_zone.id))
                .map(|zone_settings| zone_settings.is_skippable)
        }
    }
}
