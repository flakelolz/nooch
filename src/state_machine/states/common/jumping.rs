use crate::prelude::*;

pub struct Start;
impl State for Start {
    fn name(&self) -> &'static str {
        "Jmp Start"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Jmp Start enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        if specials_transitions(ctx, buffer, physics) {
            return;
        }
        // Set jump direction
        handle_jump_flags(ctx, buffer, physics);
        // Base case
        if ctx.elapsed > ctx.total {
            // Transitions
            match ctx.flags.jump {
                JumpFlags::None => handle_jump_flags(ctx, buffer, physics),
                JumpFlags::Neutral => ctx.next = Some(Box::new(jumping::Neutral)),
                JumpFlags::Forward => ctx.next = Some(Box::new(jumping::Forward)),
                JumpFlags::Backward => ctx.next = Some(Box::new(jumping::Backward)),
            }
        }
    }
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Jmp Start exit", ctx.player);
    }
}

pub struct Neutral;
impl State for Neutral {
    fn name(&self) -> &'static str {
        "Jmp Neutral"
    }

    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, physics: &mut Physics) {
        println!("{} -> Jmp Neutral enter", ctx.player);
        if physics.position.y <= 0 {
            physics.velocity.y = ctx.data.jump_velocity;
            physics.acceleration.y = ctx.data.jump_deceleration;
            physics.airborne = true;
        }
    }

    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        // Base case
        if handle_ground_collision(ctx, buffer, physics) {
            return;
        }
        // Transitions
        if specials_transitions(ctx, buffer, physics) {
            return;
        }
        normals_transitions(ctx, buffer, physics);
    }

    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Jmp Neutral exit", ctx.player);
    }
}

pub struct Forward;
impl State for Forward {
    fn name(&self) -> &'static str {
        "Jmp Forward"
    }

    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, physics: &mut Physics) {
        println!("{} -> Jmp Forward enter", ctx.player);
        if physics.position.y <= 0 {
            physics.velocity.y = ctx.data.jump_velocity;
            physics.acceleration.y = ctx.data.jump_deceleration;
            physics.set_forward_velocity(ctx.data.jump_forward);
            physics.airborne = true;
        }
    }

    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        // Base case
        if handle_ground_collision(ctx, buffer, physics) {
            return;
        }
        // Transitions
        if specials_transitions(ctx, buffer, physics) {
            return;
        }
        normals_transitions(ctx, buffer, physics);
    }

    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Jmp Forward exit", ctx.player);
    }
}

pub struct Backward;
impl State for Backward {
    fn name(&self) -> &'static str {
        "Jmp Backward"
    }

    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, physics: &mut Physics) {
        println!("{} -> Jmp Backward enter", ctx.player);
        if physics.position.y <= 0 {
            physics.velocity.y = ctx.data.jump_velocity;
            physics.acceleration.y = ctx.data.jump_deceleration;
            physics.set_backward_velocity(ctx.data.jump_backward);
            physics.airborne = true;
        }
    }

    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        // Base case
        if handle_ground_collision(ctx, buffer, physics) {
            return;
        }
        // Transitions
        if specials_transitions(ctx, buffer, physics) {
            return;
        }
        normals_transitions(ctx, buffer, physics);
    }

    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Jmp Backward exit", ctx.player);
    }
}

pub struct End;
impl State for End {
    fn name(&self) -> &'static str {
        "Jmp End"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Jmp End enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        // Base case
        if ctx.elapsed > ctx.total {
            // Transitions
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
            if walk_transition(ctx, buffer, physics) {
                return;
            }
            // return to idle
            ctx.next = Some(Box::new(standing::Idle));
        }
    }
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Jmp End exit", ctx.player);
    }
}

pub struct LightPunch;
impl State for LightPunch {
    fn name(&self) -> &'static str {
        "Jmp LightPunch"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Jmp LightPunch enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        common_jumping_attack_transitions(ctx, buffer, physics);
    }
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Jmp LightPunch exit", ctx.player);
    }
}

pub struct MediumPunch;
impl State for MediumPunch {
    fn name(&self) -> &'static str {
        "Jmp MediumPunch"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Jmp MediumPunch enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        common_jumping_attack_transitions(ctx, buffer, physics);
    }
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Jmp MediumPunch exit", ctx.player);
    }
}

pub struct HeavyPunch;
impl State for HeavyPunch {
    fn name(&self) -> &'static str {
        "Jmp HeavyPunch"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Jmp HeavyPunch enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        common_jumping_attack_transitions(ctx, buffer, physics);
    }
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Jmp HeavyPunch exit", ctx.player);
    }
}

pub struct LightKick;
impl State for LightKick {
    fn name(&self) -> &'static str {
        "Jmp LightKick"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Jmp LightKick enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        common_jumping_attack_transitions(ctx, buffer, physics);
    }
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Jmp LightKick exit", ctx.player);
    }
}

pub struct MediumKick;
impl State for MediumKick {
    fn name(&self) -> &'static str {
        "Jmp MediumKick"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Jmp MediumKick enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        common_jumping_attack_transitions(ctx, buffer, physics);
    }
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Jmp MediumKick exit", ctx.player);
    }
}

pub struct HeavyKick;
impl State for HeavyKick {
    fn name(&self) -> &'static str {
        "Jmp HeavyKick"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Jmp HeavyKick enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        common_jumping_attack_transitions(ctx, buffer, physics);
    }
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Jmp HeavyKick exit", ctx.player);
    }
}

pub struct AttackEnd;
impl State for AttackEnd {
    fn name(&self) -> &'static str {
        "Jmp AttackEnd"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Jmp AttackEnd enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        // Base case
        handle_ground_collision(ctx, buffer, physics);
    }
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Jmp AttackEnd exit", ctx.player);
    }
}
