use crate::prelude::*;

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct Physics {
    pub position: IVec2,
    pub velocity: IVec2,
    pub acceleration: IVec2,
    pub flipped: bool,
}

impl Physics {
    pub fn new((x, y): (i32, i32), flipped: bool) -> Self {
        Self {
            position: IVec2::new(x, y),
            velocity: IVec2::ZERO,
            acceleration: IVec2::ZERO,
            flipped,
        }
    }
}
