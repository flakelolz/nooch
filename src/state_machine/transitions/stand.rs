use crate::prelude::*;

pub fn common_standing_attack_transitions(
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

pub fn normals_transitions(ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) -> bool {
    if !physics.airborne && Group::Normals.set(ctx, buffer, physics) {
        return true;
    }
    if physics.airborne && Group::AirNormals.set(ctx, buffer, physics) {
        return true;
    }
    false
}

pub fn walk_transition(ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) -> bool {
    if Group::Walks.set(ctx, buffer, physics) {
        return true;
    }
    false
}

pub fn turn_transition(ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) -> bool {
    if face_opponent(physics, buffer) {
        // Base case
        if buffer.down() {
            ctx.next = Some(Box::new(crouching::Turn));
            return true;
        }
        ctx.next = Some(Box::new(standing::Turn));
        return true;
    }
    false
}

pub fn dash_transitions(ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) -> bool {
    if Group::Dashes.set(ctx, buffer, physics) {
        return true;
    }
    false
}

pub fn specials_transitions(ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) -> bool {
    if Group::Specials.set(ctx, buffer, physics) {
        return true;
    }
    false
}
