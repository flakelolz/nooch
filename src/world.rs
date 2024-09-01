use crate::prelude::*;

#[derive(Component, Default, Clone, Copy, Debug, PartialEq)]
pub enum Name {
    #[default]
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
    // NOTE: This will also cap the game's framerate if lower than raylib's target_fps, so I want to
    // make higher, just in case
    world.set_target_fps(64.0);
    // Singletons
    world.add::<InputConfig>();
    world.set(Assets::new(rl, thread));
    world.add::<DebugUI>();
    world.add::<Collisions>();
    world.add::<HitEvents>();
    world.add::<Animation>();

    // Player 1
    let name = Name::Ken;
    let player = Player::One;
    let character_data = load_character_data(name.into());
    let action_data = ActionData::new(name);
    let origin = character_data.origin;
    let action_names = action_data.keys().cloned().collect::<Vec<_>>();
    world
        .entity_named("Player 1")
        .set(name)
        .set(player)
        .add::<Input>()
        .add::<InputBuffer>()
        .set(Physics::new((112 * 1000, 0), false))
        .set(StateMachine::new(player, name, character_data))
        .set(action_data)
        .set(Animator::new("St Idle".into(), 11, origin))
        .set(AnimationData::new(name));

    world.set(EditorData::new(action_names));

    // Player 2
    let name = Name::Ken;
    let player = Player::Two;
    let data = load_character_data(name.into());
    let origin = data.origin;
    world
        .entity_named("Player 2")
        .set(name)
        .set(player)
        .add::<Input>()
        .add::<InputBuffer>()
        .set(Physics::new((304 * 1000, 0), true))
        .set(StateMachine::new(player, name, data))
        .set(ActionData::new(name))
        .set(Animator::new("St Idle".into(), 10, origin))
        .set(AnimationData::new(name));
}
