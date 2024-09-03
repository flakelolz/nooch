mod crouch;
mod jump;
mod react;
mod stand;

pub use self::crouch::*;
pub use self::jump::*;
pub use self::react::*;
pub use self::stand::*;

use crate::prelude::*;

pub fn handle_transitions(world: &mut World) {
    let query = world
        .query_named::<(&mut StateMachine, &mut Animator)>("Setup animation")
        .set_cached()
        .build();
    query.each(|(state, animator)| {
        if let Some(next) = &state.ctx.next {
            // Setup animation
            animator.reset();
            animator.current = next.name().to_string();
        }
    });

    let query = world
        .query_named::<(&mut StateMachine, &ActionData, &mut Buffer, &mut Physics)>(
            "Handle transitions",
        )
        .set_cached()
        .build();
    query.each(|(state, data, buffer, physics)| {
        if let Some(next) = state.ctx.next.take() {
            // State transition
            state.current.on_exit(&mut state.ctx, buffer, physics);
            state.current = next;
            state.ctx.elapsed = 1;
            state.ctx.reaction.has_hit = false;
            state.current.on_enter(&mut state.ctx, buffer, physics);

            if let Some(action) = data.get(state.current.name()) {
                state.ctx.total = action.total;

                // Setup action modifiers
                match &action.modifiers {
                    Some(_) => {
                        state.modifiers.index = 0;
                        state.modifiers.commands.clone_from(&action.modifiers);
                    }
                    None => {
                        state.modifiers.commands = None;
                    }
                }
            }
        }
    })
}
