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
    pub fn new(player: Player, name: Name, data: CharacterData) -> Self {
        Self {
            current: Box::new(standing::Idle),
            ctx: Context::new(player, name, data),
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
    fn on_enter(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics);
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics);
    fn on_exit(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics);
}

pub fn update_state(world: &mut World) {
    let query = world
        .query_named::<(&mut StateMachine, &ActionData, &mut Buffer, &mut Physics)>("Update State")
        .set_cached()
        .build();
    query.each(|(state, data, buffer, physics)| {
        match data.get(state.current.name()) {
            Some(action) => {
                state.ctx.total = action.total;
                if state.ctx.reaction.hitstop == 0 {
                    state.ctx.elapsed += 1;
                }

                if state.ctx.elapsed > action.total && action.looping {
                    state.ctx.elapsed = 1;
                }
            }
            None => {
                eprintln!("Action {} not found!!!", state.current.name());
                state.ctx.next = Some(Box::new(standing::Idle));
            }
        }

        state.ctx.locks.dash_lockout(buffer, 6);
        state.current.on_update(&mut state.ctx, buffer, physics);
    });

    handle_transitions(world);
    handle_modifiers(world);
}
