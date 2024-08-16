mod buffer;
mod buttons;
mod config;
mod motions;

pub use self::buffer::*;
pub use self::buttons::*;
pub use self::config::*;
pub use self::motions::*;
use crate::prelude::*;

pub fn update_buffer(world: &mut World) {
    let query = world
        .query_named::<(&Input, &mut InputBuffer)>("Update Buffer")
        .set_cached()
        .build();

    query.each(|(input, buffer)| {
        buffer.update_buffer(input);
    })
}

pub fn update_input(world: &mut World, rl: &RaylibHandle) {
    let config_q = world
        .query_named::<&InputConfig>("Get Input Config")
        .set_cached()
        .singleton()
        .build();

    let query = world
        .query_named::<(&mut Input, &Player)>("Update Input")
        .set_cached()
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
            **input = Buttons::None.num();
            let left =
                rl.is_key_down(keyboard.left) || rl.is_gamepad_button_down(port, gamepad.left);
            let right =
                rl.is_key_down(keyboard.right) || rl.is_gamepad_button_down(port, gamepad.right);
            let up = rl.is_key_down(keyboard.up) || rl.is_gamepad_button_down(port, gamepad.up);
            let down =
                rl.is_key_down(keyboard.down) || rl.is_gamepad_button_down(port, gamepad.down);

            if up {
                *input |= Buttons::U;
            }
            if down {
                *input |= Buttons::D;
            }
            if left {
                *input |= Buttons::L;
            }
            if right {
                *input |= Buttons::R;
            }
            if !up && !down && !left && !right {
                *input |= Buttons::N;
            }
            // Horizontal SOCD
            if left && right {
                *input ^= Buttons::L;
                *input ^= Buttons::R;
            }
            if up && down {
                *input ^= Buttons::D;
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

    // Apply facing direction to the input itself
    let facing_q = world.query::<(&mut Input, &mut Physics)>().build();
    facing_q.each(|(input, physics)| {
        if physics.facing_left {
            *input |= Buttons::FacingLeft;
        }
        if physics.facing_opponent {
            *input |= Buttons::FacingOpponent;
        }
    });
}

#[derive(Component, Default, Debug, Clone, Copy, PartialEq)]
pub struct Input(u32);
impl Input {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn pressed(&self, button: Buttons) -> bool {
        self.0 & button == button
    }

    pub fn facing_left(&self) -> bool {
        self.0 & Buttons::FacingLeft == Buttons::FacingLeft
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

impl std::ops::BitAnd<Buttons> for Input {
    type Output = u32;
    fn bitand(self, rhs: Buttons) -> u32 {
        self.0 & rhs.num()
    }
}

impl std::ops::BitOrAssign<u32> for Input {
    fn bitor_assign(&mut self, rhs: u32) {
        self.0 |= rhs;
    }
}

impl std::ops::BitOrAssign<Buttons> for Input {
    fn bitor_assign(&mut self, rhs: Buttons) {
        self.0 |= rhs.num();
    }
}

impl std::ops::BitAndAssign<u32> for Input {
    fn bitand_assign(&mut self, rhs: u32) {
        self.0 &= rhs;
    }
}

impl std::ops::BitAndAssign<Buttons> for Input {
    fn bitand_assign(&mut self, rhs: Buttons) {
        self.0 &= rhs.num();
    }
}

impl std::ops::BitXorAssign<u32> for Input {
    fn bitxor_assign(&mut self, rhs: u32) {
        self.0 ^= rhs;
    }
}

impl std::ops::BitXorAssign<Buttons> for Input {
    fn bitxor_assign(&mut self, rhs: Buttons) {
        self.0 ^= rhs.num();
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
