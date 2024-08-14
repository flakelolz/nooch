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
            elapsed: 1,
            total: 1,
            ..Default::default()
        }
    }
}

pub fn handle_modifiers(world: &mut World) {
    let query = world
        .query_named::<&mut StateMachine>("Handle modifiers")
        .set_cached()
        .build();
    query.each(|state| {
        if let Some(command) = &state.modifiers.commands {
            if let Some(positions) = &command.positions {
                if let Some(position) = positions.get(state.modifiers.index) {
                    if position.on_frame == state.ctx.elapsed {
                        state.ctx.physics.set_forward_position(position.value.x);
                        state.ctx.physics.position.y = position.value.y;
                        state.modifiers.index += 1;
                    }
                }
            }
        }
    });
}
