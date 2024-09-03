use crate::prelude::*;

pub struct Idle;
impl State for Idle {
    fn name(&self) -> &'static str {
        "St Idle"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> St Idle enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        // Apply physics
        physics.velocity.x = 0;
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
        if crouch_transition(ctx, buffer, physics) {
            return;
        }
        if dash_transitions(ctx, buffer, physics) {
            return;
        }
        walk_transition(ctx, buffer, physics);
    }
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> St Idle exit", ctx.player);
    }
}

pub struct Turn;
impl State for Turn {
    fn name(&self) -> &'static str {
        "St Turn"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> St Turn enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        // Transitions
        if ctx.elapsed > ctx.total {
            if jump_transitions(ctx, buffer, physics) {
                return;
            }
            if specials_transitions(ctx, buffer, physics) {
                return;
            }
            if normals_transitions(ctx, buffer, physics) {
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
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> St Turn exit", ctx.player);
    }
}

pub struct WalkForward;
impl State for WalkForward {
    fn name(&self) -> &'static str {
        "St WalkForward"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, physics: &mut Physics) {
        println!("{} -> St WalkForward enter", ctx.player);
        // FIX: Find a way to move on the first frame
        physics.set_forward_velocity(ctx.data.forward_walk);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        // Special case for walking
        physics.set_forward_velocity(ctx.data.forward_walk);
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
        if crouch_transition(ctx, buffer, physics) {
            return;
        }
        if dash_transitions(ctx, buffer, physics) {
            return;
        }
        // Base case & return to idle
        if !buffer.forward() {
            ctx.next = Some(Box::new(standing::Idle));
        }
    }
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, physics: &mut Physics) {
        // If velocity was applied earlier in the state, remove it
        physics.velocity.x = 0;
        println!("{} -> St WalkForward exit", ctx.player);
    }
}

pub struct WalkBackward;
impl State for WalkBackward {
    fn name(&self) -> &'static str {
        "St WalkBackward"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, physics: &mut Physics) {
        println!("{} -> St WalkBackward enter", ctx.player);
        physics.set_backward_velocity(ctx.data.backward_walk);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        // Special case for walking
        physics.set_backward_velocity(ctx.data.backward_walk);
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
        if crouch_transition(ctx, buffer, physics) {
            return;
        }
        if dash_transitions(ctx, buffer, physics) {
            return;
        }
        // Base case & return to idle
        if !buffer.backward() {
            ctx.next = Some(Box::new(standing::Idle));
        }
    }
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, physics: &mut Physics) {
        // If velocity was applied earlier in the state, remove it
        physics.velocity.x = 0;
        println!("{} -> St WalkBackward exit", ctx.player);
    }
}

pub struct DashForward;
impl State for DashForward {
    fn name(&self) -> &'static str {
        "St DashForward"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> St DashForward enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        // Transitions
        if ctx.elapsed > ctx.total {
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
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> St DashForward exit", ctx.player);
    }
}

pub struct DashBackward;
impl State for DashBackward {
    fn name(&self) -> &'static str {
        "St DashBackward"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> St DashBackward enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        // Transitions
        if ctx.elapsed > ctx.total {
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
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> St DashBackward exit", ctx.player);
    }
}

pub struct LightPunch;
impl State for LightPunch {
    fn name(&self) -> &'static str {
        "St LightPunch"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> St LightPunch enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        if ctx.elapsed > ctx.total {
            common_standing_attack_transitions(ctx, buffer, physics);
        }
    }
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> St LightPunch exit", ctx.player);
    }
}

pub struct MediumPunch;
impl State for MediumPunch {
    fn name(&self) -> &'static str {
        "St MediumPunch"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> St MediumPunch enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        if ctx.elapsed > ctx.total {
            common_standing_attack_transitions(ctx, buffer, physics);
        }
    }
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> St MediumPunch exit", ctx.player);
    }
}

pub struct HeavyPunch;
impl State for HeavyPunch {
    fn name(&self) -> &'static str {
        "St HeavyPunch"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> St HeavyPunch enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        if ctx.elapsed > ctx.total {
            common_standing_attack_transitions(ctx, buffer, physics);
        }
    }
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> St HeavyPunch exit", ctx.player);
    }
}

pub struct LightKick;
impl State for LightKick {
    fn name(&self) -> &'static str {
        "St LightKick"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> St LightKick enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        if ctx.elapsed > ctx.total {
            common_standing_attack_transitions(ctx, buffer, physics);
        }
    }
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> St LightKick exit", ctx.player);
    }
}

pub struct MediumKick;
impl State for MediumKick {
    fn name(&self) -> &'static str {
        "St MediumKick"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> St MediumKick enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        if ctx.elapsed > ctx.total {
            common_standing_attack_transitions(ctx, buffer, physics);
        }
    }
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> St MediumKick exit", ctx.player);
    }
}

pub struct HeavyKick;
impl State for HeavyKick {
    fn name(&self) -> &'static str {
        "St HeavyKick"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> St HeavyKick enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        if ctx.elapsed > ctx.total {
            common_standing_attack_transitions(ctx, buffer, physics);
        }
    }
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> St HeavyKick exit", ctx.player);
    }
}
