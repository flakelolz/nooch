use crate::prelude::*;

pub struct Idle;
impl State for Idle {
    fn name(&self) -> &'static str {
        "St Idle"
    }
    fn enter(&mut self, context: &mut Context) {
        println!("{} -> St Idle enter", context.player);
    }
    fn update(&mut self, context: &mut Context) {
        context.physics.velocity.x = 0;
        if context.input.pressed(&Buttons::Hk) {
            context.next = Some(Box::new(standing::HeavyKick));
            return;
        }

        if context.input.pressed(&Buttons::Mp) {
            context.next = Some(Box::new(standing::MediumPunch));
            return;
        }

        if context.input.pressed(&Buttons::Right) {
            context.next = Some(Box::new(standing::WalkForward));
            return;
        }

        if context.input.pressed(&Buttons::Left) {
            context.next = Some(Box::new(standing::WalkBackward));
        }
    }
    fn exit(&mut self, context: &mut Context) {
        println!("{} -> St Idle exit", context.player);
    }
}

pub struct WalkForward;
impl State for WalkForward {
    fn name(&self) -> &'static str {
        "St WalkForward"
    }
    fn enter(&mut self, context: &mut Context) {
        println!("{} -> St WalkForward enter", context.player);
    }
    fn update(&mut self, context: &mut Context) {
        context.physics.velocity.x = 3000;
        if !context.input.pressed(&Buttons::Right) {
            context.next = Some(Box::new(Idle));
        }
    }
    fn exit(&mut self, context: &mut Context) {
        println!("{} -> St WalkForward exit", context.player);
    }
}

pub struct WalkBackward;
impl State for WalkBackward {
    fn name(&self) -> &'static str {
        "St WalkBackward"
    }
    fn enter(&mut self, context: &mut Context) {
        println!("{} -> St WalkBackward enter", context.player);
    }
    fn update(&mut self, context: &mut Context) {
        context.physics.velocity.x = -3000;
        if !context.input.pressed(&Buttons::Left) {
            context.next = Some(Box::new(Idle));
        }
    }
    fn exit(&mut self, context: &mut Context) {
        println!("{} -> St WalkBackward exit", context.player);
    }
}

pub struct MediumPunch;
impl State for MediumPunch {
    fn name(&self) -> &'static str {
        "St MediumPunch"
    }
    fn enter(&mut self, context: &mut Context) {
        println!("{} -> St MediumPunch enter", context.player);
    }
    fn update(&mut self, context: &mut Context) {
        if context.elapsed >= context.total {
            context.next = Some(Box::new(Idle));
        }
    }
    fn exit(&mut self, context: &mut Context) {
        println!("{} -> St MediumPunch exit", context.player);
    }
}

pub struct HeavyKick;
impl State for HeavyKick {
    fn name(&self) -> &'static str {
        "St HeavyKick"
    }
    fn enter(&mut self, context: &mut Context) {
        println!("{} -> St HeavyKick enter", context.player);
    }
    fn update(&mut self, context: &mut Context) {
        if context.elapsed >= context.total {
            context.next = Some(Box::new(Idle));
        }
    }
    fn exit(&mut self, context: &mut Context) {
        println!("{} -> St HeavyKick exit", context.player);
    }
}
