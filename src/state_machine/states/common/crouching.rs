use crate::prelude::*;

pub struct Start;
impl State for Start {
    fn name(&self) -> &'static str {
        "Cr Start"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Cr Start enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
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
        // Special case for releasing down on crouch start
        if ctx.elapsed > ctx.total && buffer.down() {
            ctx.next = Some(Box::new(crouching::Idle));
        }
        // Base case
        if !buffer.down() {
            ctx.next = Some(Box::new(crouching::End));
        }
    }
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Cr Start exit", ctx.player);
    }
}

pub struct Idle;
impl State for Idle {
    fn name(&self) -> &'static str {
        "Cr Idle"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Cr Idle enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
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
        // Base case
        if !buffer.down() {
            ctx.next = Some(Box::new(crouching::End));
        }
    }
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Cr Idle exit", ctx.player);
    }
}

pub struct End;
impl State for End {
    fn name(&self) -> &'static str {
        "Cr End"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Cr End enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        // Transitions
        // if context.ctx.reaction.blockstun == 0 {
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
        // }
        // Base case & return to idle
        if ctx.elapsed > ctx.total {
            ctx.next = Some(Box::new(standing::Idle));
        }
    }
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Cr End exit", ctx.player);
    }
}

pub struct Turn;
impl State for Turn {
    fn name(&self) -> &'static str {
        "Cr Turn"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Cr Turn enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        // Transitions
        // if context.ctx.reaction.blockstun == 0 {
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
        }

        if ctx.elapsed > ctx.total {
            if !buffer.down() {
                // Return to idle
                ctx.next = Some(Box::new(crouching::End));
            } else {
                ctx.next = Some(Box::new(crouching::Idle));
            }
        }
    }
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Cr Turn exit", ctx.player);
    }
}

pub struct LightPunch;
impl State for LightPunch {
    fn name(&self) -> &'static str {
        "Cr LightPunch"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Cr LightPunch enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        common_crouching_attack_transitions(ctx, buffer, physics);
    }
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Cr LightPunch exit", ctx.player);
    }
}

pub struct MediumPunch;
impl State for MediumPunch {
    fn name(&self) -> &'static str {
        "Cr MediumPunch"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Cr MediumPunch enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        common_crouching_attack_transitions(ctx, buffer, physics);
    }
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Cr MediumPunch exit", ctx.player);
    }
}

pub struct HeavyPunch;
impl State for HeavyPunch {
    fn name(&self) -> &'static str {
        "Cr HeavyPunch"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Cr HeavyPunch enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        common_crouching_attack_transitions(ctx, buffer, physics);
    }
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Cr HeavyPunch exit", ctx.player);
    }
}

pub struct LightKick;
impl State for LightKick {
    fn name(&self) -> &'static str {
        "Cr LightKick"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Cr LightKick enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        common_crouching_attack_transitions(ctx, buffer, physics);
    }
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Cr LightKick exit", ctx.player);
    }
}

pub struct MediumKick;
impl State for MediumKick {
    fn name(&self) -> &'static str {
        "Cr MediumKick"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Cr MediumKick enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        common_crouching_attack_transitions(ctx, buffer, physics);
    }
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Cr MediumKick exit", ctx.player);
    }
}

pub struct HeavyKick;
impl State for HeavyKick {
    fn name(&self) -> &'static str {
        "Cr HeavyKick"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Cr HeavyKick enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        common_crouching_attack_transitions(ctx, buffer, physics);
    }
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Cr HeavyKick exit", ctx.player);
    }
}
