use crate::prelude::*;

pub struct Idle;
impl State for Idle {
    fn name(&self) -> &'static str {
        "St Idle"
    }
    fn on_enter(&mut self, ctx: &mut Context) {
        println!("{} -> St Idle enter", ctx.player);
    }
    fn on_update(&mut self, ctx: &mut Context) {
        ctx.physics.velocity.x = 0;
        if ctx.input.pressed(&Buttons::Hk) {
            ctx.next = Some(Box::new(standing::HeavyKick));
            return;
        }

        if ctx.input.pressed(&Buttons::Mp) {
            ctx.next = Some(Box::new(standing::MediumPunch));
            return;
        }

        if ctx.input.pressed(&Buttons::Right) {
            ctx.next = Some(Box::new(standing::WalkForward));
            return;
        }

        if ctx.input.pressed(&Buttons::Left) {
            ctx.next = Some(Box::new(standing::WalkBackward));
        }
    }
    fn on_exit(&mut self, ctx: &mut Context) {
        println!("{} -> St Idle exit", ctx.player);
    }
}

pub struct WalkForward;
impl State for WalkForward {
    fn name(&self) -> &'static str {
        "St WalkForward"
    }
    fn on_enter(&mut self, ctx: &mut Context) {
        println!("{} -> St WalkForward enter", ctx.player);
    }
    fn on_update(&mut self, ctx: &mut Context) {
        ctx.physics.velocity.x = 3000;
        if !ctx.input.pressed(&Buttons::Right) {
            ctx.next = Some(Box::new(Idle));
        }
    }
    fn on_exit(&mut self, ctx: &mut Context) {
        println!("{} -> St WalkForward exit", ctx.player);
    }
}

pub struct WalkBackward;
impl State for WalkBackward {
    fn name(&self) -> &'static str {
        "St WalkBackward"
    }
    fn on_enter(&mut self, ctx: &mut Context) {
        println!("{} -> St WalkBackward enter", ctx.player);
    }
    fn on_update(&mut self, context: &mut Context) {
        context.physics.velocity.x = -3000;
        if !context.input.pressed(&Buttons::Left) {
            context.next = Some(Box::new(Idle));
        }
    }
    fn on_exit(&mut self, context: &mut Context) {
        println!("{} -> St WalkBackward exit", context.player);
    }
}

pub struct MediumPunch;
impl State for MediumPunch {
    fn name(&self) -> &'static str {
        "St MediumPunch"
    }
    fn on_enter(&mut self, ctx: &mut Context) {
        println!("{} -> St MediumPunch enter", ctx.player);
    }
    fn on_update(&mut self, ctx: &mut Context) {
        if ctx.elapsed > ctx.total {
            ctx.next = Some(Box::new(Idle));
        }
    }
    fn on_exit(&mut self, ctx: &mut Context) {
        println!("{} -> St MediumPunch exit", ctx.player);
    }
}

pub struct HeavyKick;
impl State for HeavyKick {
    fn name(&self) -> &'static str {
        "St HeavyKick"
    }
    fn on_enter(&mut self, ctx: &mut Context) {
        println!("{} -> St HeavyKick enter", ctx.player);
    }
    fn on_update(&mut self, ctx: &mut Context) {
        if ctx.elapsed > ctx.total {
            ctx.next = Some(Box::new(Idle));
        }
    }
    fn on_exit(&mut self, ctx: &mut Context) {
        println!("{} -> St HeavyKick exit", ctx.player);
    }
}
