use crate::prelude::*;

pub fn common_standing_attack_transitions(ctx: &mut Context) {
    // // Kara-cancel
    // if context.elapsed == 2 && specials_transitions(context, buffer, physics) {
    //     return;
    // }
    // // Base case
    if ctx.elapsed > ctx.total {
        // Transitions
        if turn_transition(ctx) {
            return;
        }
        //     if specials_transitions(context, buffer, physics) {
        //         return;
        //     }
        if normals_transitions(ctx) {
            return;
        }
        //     if jump_transitions(context, buffer, physics) {
        //         return;
        //     }
        if crouch_transition(ctx) {
            return;
        }
        //     if dash_transitions(context, buffer, physics) {
        //         return;
        //     }
        if walk_transition(ctx) {
            return;
        }
        // Return to idle
        ctx.next = Some(Box::new(standing::Idle));
    }
}

pub fn normals_transitions(ctx: &mut Context) -> bool {
    if !ctx.physics.airborne && Group::Normals.set(ctx) {
        return true;
    }
    if ctx.physics.airborne && Group::AirNormals.set(ctx) {
        return true;
    }
    false
}

pub fn walk_transition(ctx: &mut Context) -> bool {
    if Group::Walks.set(ctx) {
        return true;
    }
    false
}

pub fn turn_transition(ctx: &mut Context) -> bool {
    if face_opponent(&mut ctx.physics, &mut ctx.buffer) {
        // Base case
        if ctx.buffer.down() {
            ctx.next = Some(Box::new(crouching::Turn));
            return true;
        }
        ctx.next = Some(Box::new(standing::Turn));
        return true;
    }
    false
}
