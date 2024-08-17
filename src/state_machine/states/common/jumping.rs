use crate::prelude::*;

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
