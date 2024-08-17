use super::Input;
use crate::{physics, prelude::*};

const BUFFER_SIZE: usize = 50;

#[derive(Component, Debug, Clone, Copy)]
pub struct InputBuffer {
    pub index: usize,
    pub buffer: [Input; BUFFER_SIZE],
    pub held: Held,
    pub dash: usize,
    pub attack: usize,
    pub cancels: usize,
}

impl Default for InputBuffer {
    fn default() -> Self {
        Self {
            index: BUFFER_SIZE - 1,
            buffer: [Input::default(); BUFFER_SIZE],
            held: Held::default(),
            dash: 8,
            attack: 1,
            cancels: 2,
        }
    }
}

impl InputBuffer {
    /// Moves the index forward and then adds the new input to the buffer
    pub fn update_buffer(&mut self, input: &Input) {
        self.index = (self.index + 1) % self.buffer.len();
        self.buffer[self.index] = *input;
        self.held.reset(*self.current(), *self.previous());
        self.held.increase(*self.current());
    }

    /// Checks the current input.
    pub fn current(&self) -> &Input {
        &self.buffer[self.index]
    }

    /// Checks the previous input.
    pub fn previous(&self) -> &Input {
        &self.buffer[(self.buffer.len() + self.index - 1) % self.buffer.len()]
    }

    /// Check if the input is currently pressed.
    pub fn pressed(&self, button: Buttons) -> bool {
        let current = self.current();
        self.check_input(&button, current)
    }

    /// Check if the input was just pressed.
    pub fn just_pressed(&self, button: Buttons) -> bool {
        let current = self.current();
        let previous = self.previous();
        self.check_input(&button, current) && !self.check_input(&button, previous)
    }

    pub fn released(&self, button: Buttons) -> bool {
        let current = self.current();
        let previous = self.previous();
        !self.check_input(&button, current) && self.check_input(&button, previous)
    }

    /// Check if the input was pressed on a specific frame.
    fn _pressed_on_frame(&self, button: Buttons, frame: usize) -> bool {
        let buffer_index = frame % self.buffer.len();
        let current = self.buffer[buffer_index];
        self.check_input(&button, &current)
    }

    /// Checks if the input was initially pressed on a specific frame.
    fn just_pressed_on_frame(&self, button: Buttons, frame: usize) -> bool {
        let buffer_index = frame % self.buffer.len();
        let last_index = (self.buffer.len() + frame - 1) % self.buffer.len();

        let current = self.buffer[buffer_index];
        let previous = self.buffer[last_index];
        self.check_input(&button, &current) && !self.check_input(&button, &previous)
    }

    /// Check if an input was performed within a certain duration on the past frames.
    pub fn buffered(&self, button: Buttons, duration: usize) -> bool {
        for i in 0..duration + 1 {
            if self.just_pressed_on_frame(button, self.buffer.len() + self.index - i) {
                return true;
            }
        }
        false
    }

    /// Checks the current forward position based on facing direction.
    pub fn forward(&self) -> bool {
        self.check_input(&Buttons::R, self.current())
    }

    /// Checks the current backward position based on facing direction.
    pub fn backward(&self) -> bool {
        self.check_input(&Buttons::L, self.current())
    }

    /// Checks if down is pressed.
    pub fn down(&self) -> bool {
        self.pressed(Buttons::D)
    }

    /// Checks if up is pressed.
    pub fn up(&self) -> bool {
        self.pressed(Buttons::U)
    }

    /// Checks if up and forward are pressed.
    pub fn up_forward(&self) -> bool {
        self.up() && self.forward()
    }

    /// Checks if up and backward are pressed.
    pub fn up_backward(&self) -> bool {
        self.up() && self.backward()
    }

    /// Gets a mut reference to the current input.
    pub fn current_mut(&mut self) -> &mut Input {
        &mut self.buffer[self.index]
    }
}

pub struct Wrapper(pub [Input; BUFFER_SIZE]);
impl std::fmt::Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in self.0.iter() {
            write!(f, " {}", item)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for InputBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut array = [" "; BUFFER_SIZE];

        array[self.index] = "I";
        for item in array {
            write!(f, " {}", item)?;
        }

        Ok(())
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Held {
    pub u: u8,
    pub d: u8,
    pub l: u8,
    pub r: u8,
    pub lp: u8,
    pub mp: u8,
    pub hp: u8,
    pub lk: u8,
    pub mk: u8,
    pub hk: u8,
}

impl Held {
    pub fn increase(&mut self, input: Input) {
        if input.pressed(Buttons::U) {
            self.u = self.u.saturating_add(1);
        }
        if input.pressed(Buttons::D) {
            self.d = self.d.saturating_add(1);
        }
        if input.pressed(Buttons::L) {
            self.l = self.l.saturating_add(1);
        }
        if input.pressed(Buttons::R) {
            self.r = self.r.saturating_add(1);
        }
        if input.pressed(Buttons::Lp) {
            self.lp = self.lp.saturating_add(1);
        }
        if input.pressed(Buttons::Mp) {
            self.mp = self.mp.saturating_add(1);
        }
        if input.pressed(Buttons::Hp) {
            self.hp = self.hp.saturating_add(1);
        }
        if input.pressed(Buttons::Lk) {
            self.lk = self.lk.saturating_add(1);
        }
        if input.pressed(Buttons::Mk) {
            self.mk = self.mk.saturating_add(1);
        }
        if input.pressed(Buttons::Hk) {
            self.hk = self.hk.saturating_add(1);
        }
    }
    pub fn reset(&mut self, current: Input, previous: Input) {
        if previous.pressed(Buttons::U) && !current.pressed(Buttons::U) {
            self.u = 0;
        }
        if previous.pressed(Buttons::D) && !current.pressed(Buttons::D) {
            self.d = 0;
        }
        if previous.pressed(Buttons::L) && !current.pressed(Buttons::L) {
            self.l = 0;
        }
        if previous.pressed(Buttons::R) && !current.pressed(Buttons::R) {
            self.r = 0;
        }
        if previous.pressed(Buttons::Lp) && !current.pressed(Buttons::Lp) {
            self.lp = 0;
        }
        if previous.pressed(Buttons::Mp) && !current.pressed(Buttons::Mp) {
            self.mp = 0;
        }
        if previous.pressed(Buttons::Hp) && !current.pressed(Buttons::Hp) {
            self.hp = 0;
        }
        if previous.pressed(Buttons::Lk) && !current.pressed(Buttons::Lk) {
            self.lk = 0;
        }
        if previous.pressed(Buttons::Mk) && !current.pressed(Buttons::Mk) {
            self.mk = 0;
        }
        if previous.pressed(Buttons::Hk) && !current.pressed(Buttons::Hk) {
            self.hk = 0;
        }
    }
}

impl std::fmt::Display for Held {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Up:{}  D:{}  L:{}  R:{}  Lp:{}  Mp:{}  Hp:{}  Lk:{}  Mk:{}  Hk:{}",
            self.u, self.d, self.l, self.r, self.lp, self.mp, self.hp, self.lk, self.mk, self.hk
        )
    }
}
