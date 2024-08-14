use crate::prelude::*;

pub fn handle_transitions(world: &mut World) {
    let query = world
        .query_named::<(&mut StateMachine, &ActionData, &mut Animator)>("Handle transitions")
        .set_cached()
        .build();
    query.each(|(state, data, animator)| {
        if let Some(mut next) = state.context.next.take() {
            // State transition
            state.current.exit(&mut state.context);
            state.context.elapsed = 1;
            next.enter(&mut state.context);
            state.current = next;

            if let Some(action) = data.get(state.current.name()) {
                state.context.total = action.total;
            }

            // Setup animation
            animator.reset();
            animator.current = state.current.name().to_string();
        }
    })
}
