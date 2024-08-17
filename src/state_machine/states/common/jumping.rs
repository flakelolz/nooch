use crate::prelude::*;

pub struct Start;
impl State for Start {
    fn name(&self) -> &'static str {
        "Jmp Start"
    }
    fn on_enter(&mut self, ctx: &mut Context) {
        println!("{} -> Jmp Start enter", ctx.player);
    }
    fn on_update(&mut self, ctx: &mut Context) {
        if specials_transitions(ctx) {
            return;
        }
        // Set jump direction
        if ctx.flags.jump == JumpFlags::Neutral {
            handle_jump_flags(ctx);
        }
        // Base case
        if ctx.elapsed > ctx.total {
            // Transitions
            match ctx.flags.jump {
                JumpFlags::None => (),
                JumpFlags::Neutral => ctx.next = Some(Box::new(jumping::Neutral)),
                JumpFlags::Forward => ctx.next = Some(Box::new(jumping::Forward)),
                JumpFlags::Backward => ctx.next = Some(Box::new(jumping::Backward)),
            }
        }
    }
    fn on_exit(&mut self, ctx: &mut Context) {
        println!("{} -> Jmp Start exit", ctx.player);
    }
}

pub struct Neutral;
impl State for Neutral {
    fn name(&self) -> &'static str {
        "Jmp Neutral"
    }

    fn on_enter(&mut self, ctx: &mut Context) {
        println!("{} -> Jmp Neutral enter", ctx.player);
        if ctx.physics.position.y <= 0 {
            ctx.physics.velocity.y = 9500;
            ctx.physics.acceleration.y = -604;
            ctx.physics.airborne = true;
        }
    }

    fn on_update(&mut self, ctx: &mut Context) {
        // Base case
        if handle_ground_collision(ctx) {
            return;
        }
        // Transitions
        if specials_transitions(ctx) {
            return;
        }
        normals_transitions(ctx);
    }

    fn on_exit(&mut self, ctx: &mut Context) {
        println!("{} -> Jmp Neutral exit", ctx.player);
    }
}

pub struct Forward;
impl State for Forward {
    fn name(&self) -> &'static str {
        "Jmp Forward"
    }

    fn on_enter(&mut self, ctx: &mut Context) {
        println!("{} -> Jmp Forward enter", ctx.player);
        if ctx.physics.position.y <= 0 {
            ctx.physics.velocity.y = 9500;
            ctx.physics.acceleration.y = -604;
            ctx.physics.set_forward_velocity(4000);
            ctx.physics.airborne = true;
        }
    }

    fn on_update(&mut self, ctx: &mut Context) {
        // Base case
        if handle_ground_collision(ctx) {
            return;
        }
        // Transitions
        if specials_transitions(ctx) {
            return;
        }
        normals_transitions(ctx);
    }

    fn on_exit(&mut self, ctx: &mut Context) {
        println!("{} -> Jmp Forward exit", ctx.player);
    }
}

pub struct Backward;
impl State for Backward {
    fn name(&self) -> &'static str {
        "Jmp Backward"
    }

    fn on_enter(&mut self, ctx: &mut Context) {
        println!("{} -> Jmp Backward enter", ctx.player);
        if ctx.physics.position.y <= 0 {
            ctx.physics.velocity.y = 9500;
            ctx.physics.acceleration.y = -604;
            ctx.physics.set_backward_velocity(4000);
            ctx.physics.airborne = true;
        }
    }

    fn on_update(&mut self, ctx: &mut Context) {
        // Base case
        if handle_ground_collision(ctx) {
            return;
        }
        // Transitions
        if specials_transitions(ctx) {
            return;
        }
        normals_transitions(ctx);
    }

    fn on_exit(&mut self, ctx: &mut Context) {
        println!("{} -> Jmp Backward exit", ctx.player);
    }
}

pub struct End;
impl State for End {
    fn name(&self) -> &'static str {
        "Jmp End"
    }
    fn on_enter(&mut self, ctx: &mut Context) {
        println!("{} -> Jmp End enter", ctx.player);
    }
    fn on_update(&mut self, ctx: &mut Context) {
        // Base case
        if ctx.elapsed > ctx.total {
            // Transitions
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
            if walk_transition(ctx) {
                return;
            }
            // return to idle
            ctx.next = Some(Box::new(standing::Idle));
        }
    }
    fn on_exit(&mut self, ctx: &mut Context) {
        println!("{} -> Jmp End exit", ctx.player);
    }
}

pub struct LightPunch;
impl State for LightPunch {
    fn name(&self) -> &'static str {
        "Jmp LightPunch"
    }
    fn on_enter(&mut self, ctx: &mut Context) {
        println!("{} -> Jmp LightPunch enter", ctx.player);
    }
    fn on_update(&mut self, ctx: &mut Context) {
        if ctx.elapsed > ctx.total {
            ctx.next = Some(Box::new(standing::Idle));
        }
    }
    fn on_exit(&mut self, ctx: &mut Context) {
        println!("{} -> Jmp LightPunch exit", ctx.player);
    }
}

pub struct MediumPunch;
impl State for MediumPunch {
    fn name(&self) -> &'static str {
        "Jmp MediumPunch"
    }
    fn on_enter(&mut self, ctx: &mut Context) {
        println!("{} -> Jmp MediumPunch enter", ctx.player);
    }
    fn on_update(&mut self, ctx: &mut Context) {
        if ctx.elapsed > ctx.total {
            ctx.next = Some(Box::new(standing::Idle));
        }
    }
    fn on_exit(&mut self, ctx: &mut Context) {
        println!("{} -> Jmp MediumPunch exit", ctx.player);
    }
}

pub struct HeavyPunch;
impl State for HeavyPunch {
    fn name(&self) -> &'static str {
        "Jmp HeavyPunch"
    }
    fn on_enter(&mut self, ctx: &mut Context) {
        println!("{} -> Jmp HeavyPunch enter", ctx.player);
    }
    fn on_update(&mut self, ctx: &mut Context) {
        if ctx.elapsed > ctx.total {
            ctx.next = Some(Box::new(standing::Idle));
        }
    }
    fn on_exit(&mut self, ctx: &mut Context) {
        println!("{} -> Jmp HeavyPunch exit", ctx.player);
    }
}

pub struct LightKick;
impl State for LightKick {
    fn name(&self) -> &'static str {
        "Jmp LightKick"
    }
    fn on_enter(&mut self, ctx: &mut Context) {
        println!("{} -> Jmp LightKick enter", ctx.player);
    }
    fn on_update(&mut self, ctx: &mut Context) {
        if ctx.elapsed > ctx.total {
            ctx.next = Some(Box::new(standing::Idle));
        }
    }
    fn on_exit(&mut self, ctx: &mut Context) {
        println!("{} -> Jmp LightKick exit", ctx.player);
    }
}

pub struct MediumKick;
impl State for MediumKick {
    fn name(&self) -> &'static str {
        "Jmp MediumKick"
    }
    fn on_enter(&mut self, ctx: &mut Context) {
        println!("{} -> Jmp MediumKick enter", ctx.player);
    }
    fn on_update(&mut self, ctx: &mut Context) {
        if ctx.elapsed > ctx.total {
            ctx.next = Some(Box::new(standing::Idle));
        }
    }
    fn on_exit(&mut self, ctx: &mut Context) {
        println!("{} -> Jmp MediumKick exit", ctx.player);
    }
}

pub struct HeavyKick;
impl State for HeavyKick {
    fn name(&self) -> &'static str {
        "Jmp HeavyKick"
    }
    fn on_enter(&mut self, ctx: &mut Context) {
        println!("{} -> Jmp HeavyKick enter", ctx.player);
    }
    fn on_update(&mut self, ctx: &mut Context) {
        if ctx.elapsed > ctx.total {
            ctx.next = Some(Box::new(standing::Idle));
        }
    }
    fn on_exit(&mut self, ctx: &mut Context) {
        println!("{} -> Jmp HeavyKick exit", ctx.player);
    }
}
