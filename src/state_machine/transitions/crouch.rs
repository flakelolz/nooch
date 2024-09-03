use crate::prelude::*;

pub fn crouch_transition(ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) -> bool {
    if Crouching::Start.set(ctx, buffer, physics) {
        return true;
    }
    false
}

pub fn common_crouching_attack_transitions(
    ctx: &mut Context,
    buffer: &mut Buffer,
    physics: &mut Physics,
) {
    // Kara-cancel
    if ctx.elapsed == 2 && specials_transitions(ctx, buffer, physics) {
        return;
    }
    // Base case
    if ctx.elapsed > ctx.total {
        // Transitions
        if turn_transition(ctx, buffer, physics) {
            return;
        }
        if jump_transitions(ctx, buffer, physics) {
            return;
        }
        if specials_transitions(ctx, buffer, physics) {
            return;
        }
        if normals_transitions(ctx, buffer, physics) {
            return;
        }
        if !buffer.down() {
            if dash_transitions(ctx, buffer, physics) {
                return;
            }
            if walk_transition(ctx, buffer, physics) {
                return;
            }
            // Return to idle
            ctx.next = Some(Box::new(crouching::End));
        } else {
            ctx.next = Some(Box::new(crouching::Idle));
        }
    }
}
