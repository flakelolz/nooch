use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Motions {
    Qcf,
    Qcb,
    Dpf,
    Dpb,
    Hcf,
    Hcb,
}

impl Motions {
    pub fn motions(&self) -> Vec<Vec<u8>> {
        match self {
            Motions::Qcf => {
                vec![vec![2, 3, 6]]
            }
            Motions::Qcb => {
                vec![vec![2, 1, 4]]
            }
            Motions::Dpf => {
                vec![vec![6, 2, 3], vec![2, 3, 2, 3]]
            }
            Motions::Dpb => {
                vec![vec![4, 2, 1], vec![2, 1, 2, 1]]
            }
            Motions::Hcf => {
                vec![vec![4, 1, 2, 3, 6], vec![4, 1, 3, 6], vec![4, 2, 6]]
            }
            Motions::Hcb => {
                vec![vec![6, 3, 2, 1, 4], vec![6, 3, 1, 4], vec![6, 2, 4]]
            }
        }
    }

    pub fn translate(&self, dir: u8) -> Buttons {
        match dir {
            1 => Buttons::DL,
            2 => Buttons::D,
            3 => Buttons::DR,
            4 => Buttons::L,
            5 => Buttons::N,
            6 => Buttons::R,
            7 => Buttons::UL,
            8 => Buttons::U,
            9 => Buttons::UR,
            _ => Buttons::None,
        }
    }

    pub fn notation(&self, button: Buttons) -> Vec<Vec<Buttons>> {
        let mut result: Vec<Vec<Buttons>> = Vec::new();
        for (i, motion) in self.motions().iter().enumerate() {
            result.push(vec![]);
            result[i].push(button);
            for dir in motion.iter().rev() {
                result[i].push(self.translate(*dir));
            }
        }
        result
    }
}

impl InputBuffer {
    /// Checks if a button has been pressed for motion inputs
    pub fn check_input(&self, button: &Buttons, current: &Input) -> bool {
        let forward;
        let backward;

        if current.facing_left() {
            forward = current.pressed(Buttons::L);
            backward = current.pressed(Buttons::R);
        } else {
            forward = current.pressed(Buttons::R);
            backward = current.pressed(Buttons::L);
        }
        let up = current.pressed(Buttons::U);
        let down = current.pressed(Buttons::D);
        let neutral = current.pressed(Buttons::N);
        match button {
            Buttons::N => neutral && !(forward || backward || up || down),
            Buttons::U => up && !(forward || backward),
            Buttons::D => down && !(forward || backward),
            Buttons::L => backward && !(up || down),
            Buttons::R => forward && !(up || down),
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
    /// Checks for a motion input limiting the time between stages.
    pub fn motion(&self, motion: Motions, button: Buttons, limit: usize) -> bool {
        let motions = motion.notation(button);
        let mut stage: usize;

        for motion in &motions {
            // Pointer to the end of the slice
            let mut r = self.index;
            // Left if looking 9 frames into the past of the buffer
            let mut l = (self.buffer.len() + r - (limit - 1) - 1) % self.buffer.len();

            stage = 0;

            for btn in motion {
                // Buffer slice of the last n inputs, n = limit
                let slice = if l > r {
                    // When left is greater than right take whats everything from left pointer to
                    // the end and everything from 0 to right pointer and contactenate them
                    let left_slice = &self.buffer[l..];
                    let right_slice = &self.buffer[..=r];
                    [left_slice, right_slice].concat()
                } else {
                    self.buffer[l..=r].to_vec()
                };

                let mut found = false;
                for (i, current) in slice.iter().rev().enumerate() {
                    if self.check_input(btn, current) {
                        found = true;
                        // Update buffer slice based on where the input was found
                        r = (self.buffer.len() + r - i) % self.buffer.len();
                        l = (self.buffer.len() + r - (limit - 1) - 1) % self.buffer.len();
                        stage += 1;
                        break;
                    }
                }
                if !found {
                    break;
                }
                if stage >= motion.len() {
                    return true;
                }
            }
        }

        println!("not {:?}", motion);
        false
    }

    pub fn motion_exact(&self, motion: &[u8], button: Buttons, left: bool) {}

    pub fn motion_timed(&self, motion: Motions, button: Buttons, range: usize, left: bool) {}
}
