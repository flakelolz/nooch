use crate::prelude::*;

pub fn common_standing_reaction_transitions(
    ctx: &mut Context,
    buffer: &mut Buffer,
    physics: &mut Physics,
) {
    // Base case
    if ctx.elapsed > ctx.total {
        // Transitions
        if turn_transition(ctx, buffer, physics) {
            return;
        }
        if specials_transitions(ctx, buffer, physics) {
            return;
        }
        if normals_transitions(ctx, buffer, physics) {
            return;
        }
        if jump_transitions(ctx, buffer, physics) {
            return;
        }
        if crouch_transition(ctx, buffer, physics) {
            return;
        }
        if dash_transitions(ctx, buffer, physics) {
            return;
        }
        if walk_transition(ctx, buffer, physics) {
            return;
        }
        // Return to idle
        ctx.next = Some(Box::new(standing::Idle));
    }
}

pub fn common_crouching_reaction_transitions(
    ctx: &mut Context,
    buffer: &mut Buffer,
    physics: &mut Physics,
) {
    if ctx.elapsed > ctx.total {
        // Transitions
        if turn_transition(ctx, buffer, physics) {
            return;
        }
        if specials_transitions(ctx, buffer, physics) {
            return;
        }
        if normals_transitions(ctx, buffer, physics) {
            return;
        }
        if jump_transitions(ctx, buffer, physics) {
            return;
        }
        if buffer.down() {
            ctx.next = Some(Box::new(crouching::Idle));
            return;
        }
        if dash_transitions(ctx, buffer, physics) {
            return;
        }
        if walk_transition(ctx, buffer, physics) {
            return;
        }
        // Return to idle
        ctx.next = Some(Box::new(standing::Idle));
    }
}
