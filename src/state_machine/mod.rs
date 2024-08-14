mod context;
mod states;
mod transitions;

use transitions::handle_transitions;

pub use self::{context::*, states::*};
use crate::prelude::*;

#[derive(Component)]
pub struct StateMachine {
    current: Box<dyn State>,
    ctx: Context,
    pub modifiers: Modifier,
}

impl Default for StateMachine {
    fn default() -> Self {
        Self {
            current: Box::new(standing::Idle),
            ctx: Context::default(),
            modifiers: Modifier::default(),
        }
    }
}

impl StateMachine {
    pub fn new(player: Player) -> Self {
        Self {
            current: Box::new(standing::Idle),
            ctx: Context::new(player),
            modifiers: Modifier::default(),
        }
    }
}

#[derive(Default)]
pub struct Modifier {
    pub index: usize,
    pub commands: Option<Modifiers>,
}

pub trait State {
    fn name(&self) -> &'static str;
    fn on_enter(&mut self, ctx: &mut Context);
    fn on_update(&mut self, ctx: &mut Context);
    fn on_exit(&mut self, ctx: &mut Context);
}

pub fn update_state(world: &mut World) {
    // Update copies of input and physics on the context
    let context_q = world
        .query_named::<(&mut StateMachine, &Input, &Physics)>("Update Context")
        .set_cached()
        .build();
    context_q.each(|(state, input, physics)| {
        state.ctx.input = *input;
        state.ctx.physics = *physics;
    });

    // Update state machine
    let query = world
        .query_named::<(&mut StateMachine, &ActionData)>("Update State")
        .set_cached()
        .build();
    query.each(|(state, data)| {
        match data.get(state.current.name()) {
            Some(action) => {
                state.ctx.elapsed += 1;

                if state.ctx.elapsed > action.total && action.looping {
                    state.ctx.elapsed = 1;
                }
            }
            None => {
                eprintln!("Action not found!!!");
                state.ctx.next = Some(Box::new(standing::Idle));
            }
        }

        state.current.on_update(&mut state.ctx);
    });

    handle_transitions(world);
    handle_modifiers(world);

    // Update the original input and physics after being set on the context
    let context_q = world
        .query_named::<(&StateMachine, &mut Input, &mut Physics)>("Update from Context")
        .set_cached()
        .build();
    context_q.each(|(state, input, physics)| {
        *input = state.ctx.input;
        *physics = state.ctx.physics;
    });
}
