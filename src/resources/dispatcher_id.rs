#[derive(PartialEq, Eq, Hash, Debug)]
pub enum DispatcherId {
    MainMenu,
    Ingame,
    Paused,
    ZoneTransition,
    GameOver,
    ZoneSelect,
}
