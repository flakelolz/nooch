mod buffer;
mod buttons;
mod config;
mod dashes;
mod motions;

pub use self::buffer::*;
pub use self::buttons::*;
pub use self::config::*;
pub use self::dashes::*;
pub use self::motions::*;
use crate::prelude::*;

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
                *input ^= Buttons::N;
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
}

pub fn update_buffer(world: &mut World) {
    let query = world
        .query_named::<(&Input, &mut Buffer)>("Update Buffer")
        .set_cached()
        .build();

    query.each(|(input, buffer)| {
        buffer.update_buffer(input);
    })
}

#[derive(Component, Default, Debug, Clone, Copy, PartialEq)]
pub struct Input(u32);
impl Input {
    pub fn new() -> Self {
        Self(0)
    }

    /// Primitive check for pressed button. Use the InputBuffer version instead.
    pub fn pressed(&self, button: Buttons) -> bool {
        self.0 & button == button
    }

    pub fn facing_left(&self) -> bool {
        self.0 & Buttons::FacingLeft == Buttons::FacingLeft
    }

    pub fn facing_opponent(&self) -> bool {
        self.0 & Buttons::FacingOpponent == Buttons::FacingOpponent
    }

    pub fn display(&self) -> f32 {
        let mut res = 0.;
        let i = self.0;
        let up = i & Buttons::U == Buttons::U;
        let down = i & Buttons::D == Buttons::D;
        let left = i & Buttons::L == Buttons::L;
        let right = i & Buttons::R == Buttons::R;
        let up_left = i & Buttons::UL == Buttons::UL;
        let up_right = i & Buttons::UR == Buttons::UR;
        let down_left = i & Buttons::DL == Buttons::DL;
        let down_right = i & Buttons::DR == Buttons::DR;
        let neutral = i & Buttons::N == Buttons::N;
        if down_left {
            res += 1.;
        }
        if down && !(left || right) {
            res += 2.;
        }
        if down_right {
            res += 3.;
        }
        if left && !(up || down) {
            res += 4.;
        }
        if neutral {
            res += 5.;
        }
        if right && !(up || down) {
            res += 6.;
        }
        if up_left {
            res += 7.;
        }
        if up && !(left || right) {
            res += 8.;
        }
        if up_right {
            res += 9.;
        }
        if i & Buttons::Lp == Buttons::Lp {
            res += 0.1;
        }
        if i & Buttons::Mp == Buttons::Mp {
            res += 0.2;
        }
        if i & Buttons::Hp == Buttons::Hp {
            res += 0.4;
        }
        if i & Buttons::Lk == Buttons::Lk {
            res += 0.01;
        }
        if i & Buttons::Mk == Buttons::Mk {
            res += 0.02;
        }
        if i & Buttons::Hk == Buttons::Hk {
            res += 0.04;
        }

        res
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
