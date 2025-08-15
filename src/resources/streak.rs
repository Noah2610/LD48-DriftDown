use climer::Timer;

#[derive(Default)]
pub struct Streak {
    pub is_active: bool,
    pub base_player_speed: f32,
    streak_timer: Timer,
}

impl Streak {
    pub fn reset(&mut self) {
        self.streak_timer.stop();
        self.is_active = false;
    }

    pub fn restart(&mut self) {
        self.streak_timer.start();
        self.is_active = true;
    }

    pub fn update(&mut self) {
        self.streak_timer.update();
    }

    pub fn pause(&mut self) {
        self.streak_timer.pause();
    }

    pub fn resume(&mut self) {
        self.streak_timer.resume();
    }

    pub fn get_secs(&self) -> f32 {
        if self.is_active {
            self.streak_timer.time_output().as_seconds()
        } else {
            0.0
        }
    }
}
