use crate::prelude::*;

#[derive(Default)]
pub struct Context {
    pub player: Player,
    pub next: Option<Box<dyn State>>,
    pub elapsed: u32,
    pub total: u32,
    pub buffer: InputBuffer,
    pub physics: Physics,
    pub flags: Flags,
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

#[derive(Debug, Default)]
pub struct Flags {
    pub jump: JumpFlags,
}

#[derive(Debug, Default, PartialEq)]
pub enum JumpFlags {
    #[default]
    None,
    Neutral,
    Forward,
    Backward,
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
            if let Some(cancels) = &command.cancels {
                for action in cancels {
                    if state.ctx.elapsed >= action.after_frame
                        && state.ctx.elapsed <= action.until_frame.unwrap_or(u32::MAX)
                    {
                        for transition in &action.states {
                            match &action.on {
                                Some(collisions) => {
                                    for kind in collisions {
                                        match kind {
                                            CollisionType::Whiff => {
                                                if transition.set(&mut state.ctx) {
                                                    turn_transition(&mut state.ctx);
                                                    return;
                                                }
                                            }
                                            CollisionType::Hit => todo!(),
                                            CollisionType::Block => todo!(),
                                            CollisionType::Parry => todo!(),
                                        }
                                    }
                                }
                                None => {
                                    if transition.set(&mut state.ctx) {
                                        println!("{} -> {:?}", state.ctx.player, transition);
                                        turn_transition(&mut state.ctx);
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });
}
