use crate::prelude::*;

#[derive(Component)]
pub enum Player {
    One,
    Two,
}

pub fn setup(world: &mut World) {
    world.set(InputConfig::default());

    world
        .entity_named("Player 1")
        .set(Player::One)
        .add::<Input>()
        .add::<Physics>();

    world
        .entity_named("Player 2")
        .set(Player::Two)
        .add::<Input>()
        .add::<Physics>();
}
