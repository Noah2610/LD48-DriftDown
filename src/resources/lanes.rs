use crate::components::prelude::Size;
use crate::settings::prelude::LanesSettings;

pub struct Lanes {
    pub lanes: Vec<Lane>,
}

pub struct Lane {
    pub x: f32,
}

impl Lanes {
    pub fn get(&self, i: usize) -> Option<&Lane> {
        self.lanes.get(i)
    }
}

impl From<(&LanesSettings, &Size)> for Lanes {
    fn from((settings, level_size): (&LanesSettings, &Size)) -> Self {
        let center_x = level_size.w * 0.5;
        let total_lanes_width = settings.spacing * settings.count as f32;
        let half_lanes_width = total_lanes_width * 0.5;

        let lanes = (0 .. settings.count)
            .into_iter()
            .map(|i| Lane {
                x: center_x + (i as f32 * settings.spacing) - half_lanes_width,
            })
            .collect();

        Self { lanes }
    }
}