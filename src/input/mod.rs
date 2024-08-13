mod buttons;
mod config;

pub use self::buttons::*;
pub use self::config::*;
use crate::prelude::*;

pub fn update_input(world: &mut World, rl: &RaylibHandle) {
    let query = world
        .query_named::<(&mut Input, &InputConfig, &Player)>("Update Input")
        .set_cached()
        .build();

    query.each(|(input, config, player)| {
        let port = match player {
            Player::One => 0,
            Player::Two => 1,
        };

        **input = 0;
        if rl.is_key_down(config.kb.up) || rl.is_gamepad_button_down(port, config.pad.up) {
            *input |= Buttons::Up;
        }
        if rl.is_key_down(config.kb.left) || rl.is_gamepad_button_down(port, config.pad.left) {
            *input |= Buttons::Left;
        }
        if rl.is_key_down(config.kb.down) || rl.is_gamepad_button_down(port, config.pad.down) {
            *input |= Buttons::Down;
        }
        if rl.is_key_down(config.kb.right) || rl.is_gamepad_button_down(port, config.pad.right) {
            *input |= Buttons::Right;
        }
        if rl.is_key_down(config.kb.lp) || rl.is_gamepad_button_down(port, config.pad.lp) {
            *input |= Buttons::Lp;
        }
        if rl.is_key_down(config.kb.mp) || rl.is_gamepad_button_down(port, config.pad.mp) {
            *input |= Buttons::Mp;
        }
        if rl.is_key_down(config.kb.hp) || rl.is_gamepad_button_down(port, config.pad.hp) {
            *input |= Buttons::Hp;
        }
        if rl.is_key_down(config.kb.lk) || rl.is_gamepad_button_down(port, config.pad.lk) {
            *input |= Buttons::Lk;
        }
        if rl.is_key_down(config.kb.mk) || rl.is_gamepad_button_down(port, config.pad.mk) {
            *input |= Buttons::Mk;
        }
        if rl.is_key_down(config.kb.hk) || rl.is_gamepad_button_down(port, config.pad.hk) {
            *input |= Buttons::Hk;
        }
    });
}

#[derive(Component, Default, Debug)]
pub struct Input(u32);
impl Input {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn pressed(&self, button: &Buttons) -> bool {
        self.0 & *button == *button
    }

    pub fn released(&self, button: &Buttons) -> bool {
        self.0 & *button == 0
    }
}

impl std::ops::Deref for Input {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Input {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::ops::BitOrAssign<u32> for Input {
    fn bitor_assign(&mut self, rhs: u32) {
        self.0 |= rhs;
    }
}

impl std::ops::BitOrAssign<Buttons> for Input {
    fn bitor_assign(&mut self, rhs: Buttons) {
        self.0 |= rhs.bits();
    }
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
