use crate::settings::prelude::LanesSettings;

pub struct Lanes {
    pub lanes: Vec<Lane>,
}

impl Lanes {
    pub fn set(&mut self, settings: &LanesSettings) {
        *self = Self::from(settings);
    }
}

pub struct Lane {
    pub x: f32,
}

pub const DEFAULT_SEGMENT_WIDTH: f32 = 128.0;

impl From<&LanesSettings> for Lanes {
    fn from(settings: &LanesSettings) -> Self {
        let segment_width =
            settings.segment_width.unwrap_or(DEFAULT_SEGMENT_WIDTH);
        let center_x = segment_width * 0.5;
        let total_lanes_width = settings.spacing * settings.count as f32;
        let half_lanes_width = total_lanes_width * 0.5;
        let half_lane_width = settings.spacing * 0.5;

        let lanes = (0..settings.count)
            .into_iter()
            .map(|i| Lane {
                x: center_x + (i as f32 * settings.spacing) - half_lanes_width
                    + half_lane_width,
            })
            .collect();

        Self { lanes }
    }
}
