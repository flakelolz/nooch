mod context;
mod states;
mod transitions;

use transitions::handle_transitions;

pub use self::{context::*, states::*};
use crate::prelude::*;

#[derive(Component)]
pub struct StateMachine {
    current: Box<dyn State>,
    context: Context,
}

impl Default for StateMachine {
    fn default() -> Self {
        Self {
            current: Box::new(standing::Idle),
            context: Context::default(),
        }
    }
}

impl StateMachine {
    pub fn new(player: Player) -> Self {
        Self {
            current: Box::new(standing::Idle),
            context: Context::new(player),
        }
    }
}

pub trait State {
    fn name(&self) -> &'static str;
    fn enter(&mut self, ctx: &mut Context);
    fn update(&mut self, ctx: &mut Context);
    fn exit(&mut self, ctx: &mut Context);
}

pub fn update_state(world: &mut World) {
    // Update copies of input and physics on the context
    let context_q = world
        .query::<(&mut StateMachine, &Input, &Physics)>()
        .set_cached()
        .build();
    context_q.each(|(state, input, physics)| {
        state.context.input = *input;
        state.context.physics = *physics;
    });

    // Update state machine
    let query = world
        .query_named::<(&mut StateMachine, &ActionData)>("Update State")
        .set_cached()
        .build();
    query.each(|(state, data)| {
        match data.get(state.current.name()) {
            Some(action) => {
                state.context.elapsed += 1;

                if state.context.elapsed > action.total && action.looping {
                    state.context.elapsed = 1;
                }
            }
            None => {
                eprintln!("Action not found!!!");
                state.context.next = Some(Box::new(standing::Idle));
            }
        }

        state.current.update(&mut state.context);
    });

    handle_transitions(world);

    // Update the original input and physics after being set on the context
    let context_q = world
        .query::<(&StateMachine, &mut Input, &mut Physics)>()
        .set_cached()
        .build();
    context_q.each(|(state, input, physics)| {
        *input = state.context.input;
        *physics = state.context.physics;
    });
}
