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

#[derive(Component, Default, Clone, Copy, Debug)]
pub enum Player {
    #[default]
    One,
    Two,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::One => write!(f, "P1"),
            Player::Two => write!(f, "P2"),
        }
    }
}

pub fn setup(world: &mut World, rl: &mut RaylibHandle, thread: &RaylibThread) {
    world.set_target_fps(60.0);
    // Singletons
    world.set(InputConfig::default());
    world.set(Assets::new(rl, thread));

    // Player 1
    let name = Name::Ken;
    let player = Player::One;
    world
        .entity_named("Player 1")
        .set(name)
        .set(player)
        .add::<Input>()
        .set(Physics::new((200 * 1000, 0), false))
        .set(StateMachine::new(player))
        .set(ActionData::new(name))
        .set(Animator::new("St Idle".into(), 11, Vec2::new(0.5, 0.835)))
        .set(AnimationData::new(name));

    // Player 2
    let name = Name::Ken;
    let player = Player::Two;
    world
        .entity_named("Player 2")
        .set(player)
        .add::<Input>()
        .set(Physics::new((400 * 1000, 0), true))
        .set(name)
        .set(StateMachine::new(player))
        .set(ActionData::new(name))
        .set(Animator::new("St Idle".into(), 10, Vec2::new(0.5, 0.835)))
        .set(AnimationData::new(name));
}
