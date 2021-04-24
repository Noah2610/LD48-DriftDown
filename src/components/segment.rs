use super::component_prelude::*;
use crate::settings::zones_settings::SegmentId;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Segment(pub SegmentId);
