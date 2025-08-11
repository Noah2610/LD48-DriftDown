use super::component_prelude::*;

#[derive(Component, Default)]
#[storage(VecStorage)]
pub struct Invincible {
    pub iframes: u32,
}

impl Invincible {
    pub fn is_invincible(&self) -> bool {
        self.iframes > 0
    }

    pub fn tick(&mut self) {
        if self.iframes > 0 {
            self.iframes -= 1;
        }
    }

    pub fn set_iframes(&mut self, iframes: u32) {
        self.iframes = iframes;
    }
}
