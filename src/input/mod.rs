mod buttons;
mod config;

pub use self::buttons::*;
pub use self::config::*;
use crate::prelude::*;

pub fn update_input(world: &mut World, rl: &RaylibHandle) {
    let query = world
        .query_named::<(&mut Input, &Player)>("Update Input")
        .set_cached()
        .build();

    let config_q = world
        .query::<&InputConfig>()
        .set_cached()
        .singleton()
        .build();

    config_q.each(|config| {
        query.each(|(input, player)| {
            let port = match player {
                Player::One => 0,
                Player::Two => 1,
            };
            let keyboard = &config.kb[port];
            let gamepad = &config.pad[port];
            let port = port as i32;

            // Reset input
            **input = Buttons::None.bits();

            if rl.is_key_down(keyboard.up) || rl.is_gamepad_button_down(port, gamepad.up) {
                *input |= Buttons::Up;
            }
            if rl.is_key_down(keyboard.left) || rl.is_gamepad_button_down(port, gamepad.left) {
                *input |= Buttons::Left;
            }
            if rl.is_key_down(keyboard.down) || rl.is_gamepad_button_down(port, gamepad.down) {
                *input |= Buttons::Down;
            }
            if rl.is_key_down(keyboard.right) || rl.is_gamepad_button_down(port, gamepad.right) {
                *input |= Buttons::Right;
            }
            if rl.is_key_down(keyboard.lp) || rl.is_gamepad_button_down(port, gamepad.lp) {
                *input |= Buttons::Lp;
            }
            if rl.is_key_down(keyboard.mp) || rl.is_gamepad_button_down(port, gamepad.mp) {
                *input |= Buttons::Mp;
            }
            if rl.is_key_down(keyboard.hp) || rl.is_gamepad_button_down(port, gamepad.hp) {
                *input |= Buttons::Hp;
            }
            if rl.is_key_down(keyboard.lk) || rl.is_gamepad_button_down(port, gamepad.lk) {
                *input |= Buttons::Lk;
            }
            if rl.is_key_down(keyboard.mk) || rl.is_gamepad_button_down(port, gamepad.mk) {
                *input |= Buttons::Mk;
            }
            if rl.is_key_down(keyboard.hk) || rl.is_gamepad_button_down(port, gamepad.hk) {
                *input |= Buttons::Hk;
            }
        });
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

impl std::fmt::Binary for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = self.0;
        std::fmt::Binary::fmt(&val, f) // delegate to u32's implementation
    }
}
