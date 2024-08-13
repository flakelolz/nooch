use crate::prelude::*;

#[derive(Component)]
pub enum Player {
    One,
    Two,
}

pub fn setup(world: &mut World) {

    world
        .entity_named("Player 1")
        .set(Player::One)
        .set(InputConfig::one())
        .add::<Input>()
        .add::<Physics>();
}
