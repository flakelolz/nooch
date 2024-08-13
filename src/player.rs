use crate::prelude::*;

#[derive(Component)]
pub enum Player {
    One,
    Two,
}

#[derive(Component, Default)]
pub struct Physics {
    position: IVec2,
    velocity: IVec2,
    acceleration: IVec2,
    /// Used to determine the z index when drawing the sprite
    depth: u8,
}

pub fn setup(world: &mut World) {
    world.entity_named("Ken").set(Player::One).add::<Physics>();
}
