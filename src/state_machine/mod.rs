mod context;
mod states;
mod transitions;

pub use self::{context::*, states::*, transitions::*};
use crate::prelude::*;

#[derive(Component)]
pub struct StateMachine {
    pub current: Box<dyn State>,
    pub ctx: Context,
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
    pub fn new(player: Player, name: Name) -> Self {
        Self {
            current: Box::new(standing::Idle),
            ctx: Context::new(player, name),
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
        .query_named::<(&mut StateMachine, &InputBuffer, &Physics)>("Update Context")
        .set_cached()
        .build();
    context_q.each(|(state, buffer, physics)| {
        state.ctx.buffer = *buffer;
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
                state.ctx.total = action.total;
                state.ctx.elapsed += 1;

                if state.ctx.elapsed > action.total && action.looping {
                    state.ctx.elapsed = 1;
                }
            }
            None => {
                eprintln!("Action {} not found!!!", state.current.name());
                state.ctx.next = Some(Box::new(standing::Idle));
            }
        }

        state.current.on_update(&mut state.ctx);
    });

    handle_transitions(world);
    handle_modifiers(world);

    // Update the original input and physics after being set on the context
    let context_q = world
        .query_named::<(&StateMachine, &mut InputBuffer, &mut Physics)>("Update from Context")
        .set_cached()
        .build();
    context_q.each(|(state, buffer, physics)| {
        *buffer = state.ctx.buffer;
        *physics = state.ctx.physics;
    });
}
