use crate::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
pub enum Name {
    Ken,
    Ryu,
}

impl From<Name> for &str {
    fn from(name: Name) -> Self {
        match name {
            Name::Ken => "ken",
            Name::Ryu => "ryu",
        }
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub enum Player {
    One,
    Two,
}

pub fn setup(world: &mut World, rl: &mut RaylibHandle, thread: &RaylibThread) {
    world.set_target_fps(60.0);
    // Singletons
    world.set(InputConfig::default());
    world.set(Assets::new(rl, thread));

    // Player 1
    let name = Name::Ken;
    world
        .entity_named("Player 1")
        .set(Player::One)
        .add::<Input>()
        .set(Physics::new((200 * 1000, 0), false))
        .set(name)
        .set(AnimationData::new(name))
        .set(Animator::new("St Idle".into(), 10, Vec2::new(0.5, 0.835)));

    // Player 2
    world
        .entity_named("Player 2")
        .set(Player::Two)
        .add::<Input>()
        .set(Physics::new((400 * 1000, 0), true))
        .set(name)
        .set(AnimationData::new(name))
        .set(Animator::new(
            "St MediumPunch".into(),
            10,
            Vec2::new(0.5, 0.835),
        ));
}
