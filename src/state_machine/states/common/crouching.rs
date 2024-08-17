use crate::prelude::*;

pub struct Start;
impl State for Start {
    fn name(&self) -> &'static str {
        "Cr Start"
    }
    fn on_enter(&mut self, ctx: &mut Context) {
        println!("{} -> Cr Start enter", ctx.player);
    }
    fn on_update(&mut self, ctx: &mut Context) {
        // Transitions
        if turn_transition(ctx) {
            return;
        }
        if specials_transitions(ctx) {
            return;
        }
        if normals_transitions(ctx) {
            return;
        }
        if jump_transitions(ctx) {
            return;
        }
        // Special case for releasing down on crouch start
        if ctx.elapsed > ctx.total && ctx.buffer.down() {
            ctx.next = Some(Box::new(crouching::Idle));
        }
        // Base case
        if !ctx.buffer.down() {
            ctx.next = Some(Box::new(crouching::End));
        }
    }
    fn on_exit(&mut self, ctx: &mut Context) {
        println!("{} -> Cr Start exit", ctx.player);
    }
}

pub struct Idle;
impl State for Idle {
    fn name(&self) -> &'static str {
        "Cr Idle"
    }
    fn on_enter(&mut self, ctx: &mut Context) {
        println!("{} -> Cr Idle enter", ctx.player);
    }
    fn on_update(&mut self, ctx: &mut Context) {
        // Transitions
        if turn_transition(ctx) {
            return;
        }
        if specials_transitions(ctx) {
            return;
        }
        if normals_transitions(ctx) {
            return;
        }
        // Base case
        if !ctx.buffer.down() {
            ctx.next = Some(Box::new(crouching::End));
        }
    }
    fn on_exit(&mut self, ctx: &mut Context) {
        println!("{} -> Cr Idle exit", ctx.player);
    }
}

pub struct End;
impl State for End {
    fn name(&self) -> &'static str {
        "Cr End"
    }
    fn on_enter(&mut self, ctx: &mut Context) {
        println!("{} -> Cr End enter", ctx.player);
    }
    fn on_update(&mut self, ctx: &mut Context) {
        // Transitions
        // if context.ctx.reaction.blockstun == 0 {
        if jump_transitions(ctx) {
            return;
        }
        if specials_transitions(ctx) {
            return;
        }
        if normals_transitions(ctx) {
            return;
        }
        if crouch_transition(ctx) {
            return;
        }
        if dash_transitions(ctx) {
            return;
        }
        if walk_transition(ctx) {
            return;
        }
        // }
        // Base case & return to idle
        if ctx.elapsed > ctx.total {
            ctx.next = Some(Box::new(standing::Idle));
        }
    }
    fn on_exit(&mut self, ctx: &mut Context) {
        println!("{} -> Cr End exit", ctx.player);
    }
}

pub struct Turn;
impl State for Turn {
    fn name(&self) -> &'static str {
        "Cr Turn"
    }
    fn on_enter(&mut self, ctx: &mut Context) {
        println!("{} -> Cr Turn enter", ctx.player);
    }
    fn on_update(&mut self, ctx: &mut Context) {
        // Transitions
        // if context.ctx.reaction.blockstun == 0 {
        if jump_transitions(ctx) {
            return;
        }
        if specials_transitions(ctx) {
            return;
        }
        if normals_transitions(ctx) {
            return;
        }
        if crouch_transition(ctx) {
            return;
        }
        if dash_transitions(ctx) {
            return;
        }
        if walk_transition(ctx) {
            return;
        }
        // }
        // Base case & return to idle
        if ctx.elapsed > ctx.total {
            ctx.next = Some(Box::new(standing::Idle));
        }
    }
    fn on_exit(&mut self, ctx: &mut Context) {
        println!("{} -> Cr Turn exit", ctx.player);
    }
}

pub struct LightPunch;
impl State for LightPunch {
    fn name(&self) -> &'static str {
        "Cr LightPunch"
    }
    fn on_enter(&mut self, ctx: &mut Context) {
        println!("{} -> Cr LightPunch enter", ctx.player);
    }
    fn on_update(&mut self, ctx: &mut Context) {
        common_crouching_attack_transitions(ctx);
    }
    fn on_exit(&mut self, ctx: &mut Context) {
        println!("{} -> Cr LightPunch exit", ctx.player);
    }
}

pub struct MediumPunch;
impl State for MediumPunch {
    fn name(&self) -> &'static str {
        "Cr MediumPunch"
    }
    fn on_enter(&mut self, ctx: &mut Context) {
        println!("{} -> Cr MediumPunch enter", ctx.player);
    }
    fn on_update(&mut self, ctx: &mut Context) {
        common_crouching_attack_transitions(ctx);
    }
    fn on_exit(&mut self, ctx: &mut Context) {
        println!("{} -> Cr MediumPunch exit", ctx.player);
    }
}

pub struct HeavyPunch;
impl State for HeavyPunch {
    fn name(&self) -> &'static str {
        "Cr HeavyPunch"
    }
    fn on_enter(&mut self, ctx: &mut Context) {
        println!("{} -> Cr HeavyPunch enter", ctx.player);
    }
    fn on_update(&mut self, ctx: &mut Context) {
        common_crouching_attack_transitions(ctx);
    }
    fn on_exit(&mut self, ctx: &mut Context) {
        println!("{} -> Cr HeavyPunch exit", ctx.player);
    }
}

pub struct LightKick;
impl State for LightKick {
    fn name(&self) -> &'static str {
        "Cr LightKick"
    }
    fn on_enter(&mut self, ctx: &mut Context) {
        println!("{} -> Cr LightKick enter", ctx.player);
    }
    fn on_update(&mut self, ctx: &mut Context) {
        common_crouching_attack_transitions(ctx);
    }
    fn on_exit(&mut self, ctx: &mut Context) {
        println!("{} -> Cr LightKick exit", ctx.player);
    }
}

pub struct MediumKick;
impl State for MediumKick {
    fn name(&self) -> &'static str {
        "Cr MediumKick"
    }
    fn on_enter(&mut self, ctx: &mut Context) {
        println!("{} -> Cr MediumKick enter", ctx.player);
    }
    fn on_update(&mut self, ctx: &mut Context) {
        common_crouching_attack_transitions(ctx);
    }
    fn on_exit(&mut self, ctx: &mut Context) {
        println!("{} -> Cr MediumKick exit", ctx.player);
    }
}

pub struct HeavyKick;
impl State for HeavyKick {
    fn name(&self) -> &'static str {
        "Cr HeavyKick"
    }
    fn on_enter(&mut self, ctx: &mut Context) {
        println!("{} -> Cr HeavyKick enter", ctx.player);
    }
    fn on_update(&mut self, ctx: &mut Context) {
        common_crouching_attack_transitions(ctx);
    }
    fn on_exit(&mut self, ctx: &mut Context) {
        println!("{} -> Cr HeavyKick exit", ctx.player);
    }
}
