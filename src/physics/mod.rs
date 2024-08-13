use crate::prelude::*;

#[derive(Component, Default)]
pub struct Physics {
    position: IVec2,
    velocity: IVec2,
    acceleration: IVec2,
    layer: u8,
}
