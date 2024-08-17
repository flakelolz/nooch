use crate::prelude::*;

#[derive(Debug)]
pub enum Dashes {
    Forward,
    Backward,
    ForcedForward,
    ForcedBackward,
}

impl InputBuffer {
    pub fn dashed(&self, dash: Dashes, mut limit: usize) -> bool {
        if limit > (self.buffer.len() + self.index) {
            limit = self.buffer.len() + self.index;
        }

        let mut stage = 0;

        match dash {
            Dashes::Forward => {
                let motion = [Buttons::R, Buttons::N, Buttons::R];

                for i in 0..limit {
                    let buffer_index =
                        (self.buffer.len() + self.index - (limit - 1) + i) % self.buffer.len();

                    let current = self.buffer[buffer_index];
                    let direction = motion[stage];

                    // Invalide if there's a down or backward input
                    if self.check_input(&Buttons::D, &current)
                        || self.check_input(&Buttons::L, &current)
                    {
                        stage = 0;
                    }

                    // Invalidate if there's a down forward input before the last forward
                    if stage == 2 && self.check_input(&Buttons::DR, &current) {
                        stage = 0;
                    }

                    if self.check_input_loose(&direction, &current) {
                        stage += 1;
                    }

                    if stage >= motion.len() {
                        return true;
                    }
                }
                false
            }
            Dashes::Backward => {
                let motion = [Buttons::L, Buttons::N, Buttons::L];

                for i in 0..limit {
                    let buffer_index =
                        (self.buffer.len() + self.index - (limit - 1) + i) % self.buffer.len();

                    let current = self.buffer[buffer_index];
                    let direction = motion[stage];

                    // Invalide if there's a down or backward input
                    if self.check_input(&Buttons::D, &current)
                        || self.check_input(&Buttons::R, &current)
                    {
                        stage = 0;
                    }

                    // Invalidate if there's a down forward input before the last forward
                    if stage == 2 && self.check_input(&Buttons::DL, &current) {
                        stage = 0;
                    }

                    if self.check_input_loose(&direction, &current) {
                        stage += 1;
                    }

                    if stage >= motion.len() {
                        return true;
                    }
                }
                false
            }
            Dashes::ForcedForward => {
                let motion = [Buttons::N, Buttons::R, Buttons::N, Buttons::R];

                for i in 0..limit {
                    let buffer_index =
                        (self.buffer.len() + self.index - (limit - 1) + i) % self.buffer.len();

                    let current = self.buffer[buffer_index];
                    let direction = motion[stage];

                    // Invalide if there's a down or backward input
                    if self.check_input(&Buttons::D, &current)
                        || self.check_input(&Buttons::L, &current)
                    {
                        stage = 0;
                    }

                    // Invalidate if there's a down forward input before the last forward
                    if stage == 3 && self.check_input(&Buttons::DR, &current) {
                        stage = 0;
                    }

                    if self.check_input_loose(&direction, &current) {
                        stage += 1;
                    }

                    if stage >= motion.len() {
                        return true;
                    }
                }
                false
            }
            Dashes::ForcedBackward => {
                let motion = [Buttons::N, Buttons::L, Buttons::N, Buttons::L];

                for i in 0..limit {
                    let buffer_index =
                        (self.buffer.len() + self.index - (limit - 1) + i) % self.buffer.len();

                    let current = self.buffer[buffer_index];
                    let direction = motion[stage];

                    // Invalide if there's a down or backward input
                    if self.check_input(&Buttons::D, &current)
                        || self.check_input(&Buttons::R, &current)
                    {
                        stage = 0;
                    }

                    // Invalidate if there's a down forward input before the last forward
                    if stage == 3 && self.check_input(&Buttons::DL, &current) {
                        stage = 0;
                    }

                    if self.check_input_loose(&direction, &current) {
                        stage += 1;
                    }

                    if stage >= motion.len() {
                        return true;
                    }
                }
                false
            }
        }
    }

    /// Checks if a button has been pressed for motion inputs, not caring about colliding inputs
    pub fn check_input_loose(&self, button: &Buttons, current: &Input) -> bool {
        let forward;
        let backward;

        if current.facing_left() {
            if current.facing_opponent() {
                forward = current.pressed(Buttons::L);
                backward = current.pressed(Buttons::R);
            } else {
                forward = current.pressed(Buttons::R);
                backward = current.pressed(Buttons::L);
            }
        } else if current.facing_opponent() {
            forward = current.pressed(Buttons::R);
            backward = current.pressed(Buttons::L);
        } else {
            forward = current.pressed(Buttons::L);
            backward = current.pressed(Buttons::R);
        }
        let up = current.pressed(Buttons::U);
        let down = current.pressed(Buttons::D);
        let neutral = current.pressed(Buttons::N);
        match button {
            Buttons::N => neutral,
            Buttons::U => up,
            Buttons::D => down,
            Buttons::L => backward,
            Buttons::R => forward,
            Buttons::DR => down && forward,
            Buttons::DL => down && backward,
            Buttons::UR => up && forward,
            Buttons::UL => up && backward,
            Buttons::Lp => current.pressed(Buttons::Lp),
            Buttons::Mp => current.pressed(Buttons::Mp),
            Buttons::Hp => current.pressed(Buttons::Hp),
            Buttons::Lk => current.pressed(Buttons::Lk),
            Buttons::Mk => current.pressed(Buttons::Mk),
            Buttons::Hk => current.pressed(Buttons::Hk),
            _ => false,
        }
    }
}
