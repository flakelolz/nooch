use crate::prelude::*;

pub fn common_standing_reaction_transitions(ctx: &mut Context) {
    // Base case
    if ctx.elapsed > ctx.total {
        // Transitions
        if turn_transition(ctx) {
            return;
        }
        if specials_transitions(ctx) {
            return;
        }
        if normals_transitions(ctx) {
            return;
        }
        if jump_transitions(ctx) {
            return;
        }
        if crouch_transition(ctx) {
            return;
        }
        if dash_transitions(ctx) {
            return;
        }
        if walk_transition(ctx) {
            return;
        }
        // Return to idle
        ctx.next = Some(Box::new(standing::Idle));
    }
}
