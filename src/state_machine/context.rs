use crate::prelude::*;

#[derive(Default)]
pub struct Context {
    pub player: Player,
    pub next: Option<Box<dyn State>>,
    pub elapsed: u32,
    pub total: u32,
    pub input: Input,
    pub physics: Physics,
}

impl Context {
    pub fn new(player: Player) -> Self {
        Self {
            player,
            next: None,
            input: Input::default(),
            physics: Physics::default(),
            elapsed: 1,
            total: 1,
        }
    }
}
