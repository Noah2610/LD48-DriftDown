// resources/settings/lanes.ron

#[derive(Deserialize, Clone, Debug)]
pub struct LanesSettings {
    pub count: usize,
    pub spacing: f32,
    #[serde(default)]
    pub segment_width: Option<f32>,
}
