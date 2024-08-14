use crate::prelude::*;

pub fn handle_transitions(world: &mut World) {
    let query = world
        .query_named::<(&mut StateMachine, &ActionData, &mut Animator)>("Handle transitions")
        .set_cached()
        .build();
    query.each(|(state, data, animator)| {
        if let Some(mut next) = state.ctx.next.take() {
            // State transition
            state.current.on_exit(&mut state.ctx);
            state.ctx.elapsed = 1;
            next.on_enter(&mut state.ctx);
            state.current = next;

            if let Some(action) = data.get(state.current.name()) {
                state.ctx.total = action.total;

                // Setup action modifiers if there are any
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

            // Setup animation
            animator.reset();
            animator.current = state.current.name().to_string();
        }
    })
}
