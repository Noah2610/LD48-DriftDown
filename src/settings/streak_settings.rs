// resources/settings/streak.ron

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct StreakSettings {
    pub speed_mult: f32,
}
