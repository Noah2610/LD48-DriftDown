#[derive(PartialEq, Eq, Hash, Clone, Deserialize, Debug)]
pub enum AnimationKey {
    Idle,
    LaneSwitch,
    GameOver,
    Cutscene,
    Invincible,
}
