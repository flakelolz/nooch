use crate::prelude::*;

pub fn common_jumping_attack_transitions(ctx: &mut Context) {
    if handle_ground_collision(ctx) {
        return;
    }
    // Base case
    if ctx.elapsed > ctx.total {
        // Transitions
        ctx.next = Some(Box::new(jumping::AttackEnd));
    }
}

pub fn jump_transitions(ctx: &mut Context) -> bool {
    if ctx.buffer.up() {
        ctx.flags.jump = JumpFlags::Neutral;
        handle_jump_flags(ctx);
        ctx.next = Some(Box::new(jumping::Start));
        return true;
    }
    false
}

pub fn handle_jump_flags(ctx: &mut Context) {
    // Only apply neutral jump at the start, when the flag is None
    if ctx.buffer.up() && ctx.flags.jump == JumpFlags::None {
        ctx.flags.jump = JumpFlags::Neutral;
    }
    // If forward or backward are inputed before jump start finishes, set jump direction
    if ctx.buffer.up_forward() {
        ctx.flags.jump = JumpFlags::Forward;
    }
    if ctx.buffer.up_backward() {
        ctx.flags.jump = JumpFlags::Backward;
    }
}

pub fn handle_ground_collision(ctx: &mut Context) -> bool {
    if ctx.physics.position.y <= 0 {
        ctx.physics.position.y = 0;
        ctx.physics.velocity = IVec2::ZERO;
        ctx.physics.acceleration.y = 0;
        ctx.physics.airborne = false;
        turn_transition(ctx);
        ctx.flags.jump = JumpFlags::Neutral;
        ctx.next = Some(Box::new(jumping::End));

        return true;
    }
    false
}
