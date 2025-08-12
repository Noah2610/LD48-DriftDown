#[derive(PartialEq, Eq, Hash, Clone, Deserialize, Debug)]
pub enum SoundKey {
    Coin,
    LaneSwitch,
    GameOver,
    Shoot,
    Hurt,
}
