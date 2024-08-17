use crate::prelude::*;

pub fn crouch_transition(ctx: &mut Context) -> bool {
    if Crouching::Start.set(ctx) {
        return true;
    }
    false
}

pub fn common_crouching_attack_transitions(ctx: &mut Context) {
    // Kara-cancel
    // if context.elapsed == 2 && specials_transitions(context, buffer, physics) {
    //     return;
    // }
    // Base case
    if ctx.elapsed > ctx.total {
        // Transitions
        // if jump_transitions(ctx) {
        //     return;
        // }
        // if specials_transitions(ctx) {
        //     return;
        // }
        if normals_transitions(ctx) {
            return;
        }
        if !ctx.buffer.down() {
            // if dash_transitions(ctx) {
            //     return;
            // }
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
