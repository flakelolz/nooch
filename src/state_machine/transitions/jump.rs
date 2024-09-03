use crate::prelude::*;

pub fn common_jumping_attack_transitions(
    ctx: &mut Context,
    buffer: &mut Buffer,
    physics: &mut Physics,
) {
    if handle_ground_collision(ctx, buffer, physics) {
        return;
    }
    // Base case
    if ctx.elapsed > ctx.total {
        // Transitions
        ctx.next = Some(Box::new(jumping::AttackEnd));
    }
}

pub fn jump_transitions(ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) -> bool {
    if buffer.up() {
        ctx.flags.jump = JumpFlags::Neutral;
        handle_jump_flags(ctx, buffer, physics);
        ctx.next = Some(Box::new(jumping::Start));
        return true;
    }
    false
}

pub fn handle_jump_flags(ctx: &mut Context, buffer: &mut Buffer, _physics: &mut Physics) {
    // Only apply neutral jump at the start, when the flag is None
    if buffer.up() && ctx.flags.jump == JumpFlags::None {
        ctx.flags.jump = JumpFlags::Neutral;
    }
    // If forward or backward are inputed before jump start finishes, set jump direction
    if buffer.up_forward() {
        ctx.flags.jump = JumpFlags::Forward;
    }
    if buffer.up_backward() {
        ctx.flags.jump = JumpFlags::Backward;
    }
}

pub fn handle_ground_collision(
    ctx: &mut Context,
    buffer: &mut Buffer,
    physics: &mut Physics,
) -> bool {
    if physics.position.y <= 0 {
        physics.position.y = 0;
        physics.velocity = IVec2::ZERO;
        physics.acceleration.y = 0;
        physics.airborne = false;
        turn_transition(ctx, buffer, physics);
        ctx.flags.jump = JumpFlags::Neutral;
        ctx.next = Some(Box::new(jumping::End));

        return true;
    }
    false
}
