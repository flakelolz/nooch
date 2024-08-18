use crate::prelude::*;

pub fn crouch_transition(ctx: &mut Context) -> bool {
    if Crouching::Start.set(ctx) {
        return true;
    }
    false
}

pub fn common_crouching_attack_transitions(ctx: &mut Context) {
    // Kara-cancel
    if ctx.elapsed == 2 && specials_transitions(ctx) {
        return;
    }
    // Base case
    if ctx.elapsed > ctx.total {
        // Transitions
        if turn_transition(ctx) {
            return;
        }
        if jump_transitions(ctx) {
            return;
        }
        if specials_transitions(ctx) {
            return;
        }
        if normals_transitions(ctx) {
            return;
        }
        if !ctx.buffer.down() {
            if dash_transitions(ctx) {
                return;
            }
            if walk_transition(ctx) {
                return;
            }
            // Return to idle
            ctx.next = Some(Box::new(crouching::End));
        } else {
            ctx.next = Some(Box::new(crouching::Idle));
        }
    }
}
