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

        face_opponent(&mut ctx.physics);
        if ctx.buffer.held.lk == 30 {
            ctx.next = Some(Box::new(standing::HeavyKick));
            return;
        }
        if ctx.buffer.buffered(Buttons::Lp, 2) && ctx.buffer.motion(Motions::Dpf, Buttons::Lp, 9) {
            println!("Dpf");
            ctx.next = Some(Box::new(standing::MediumPunch));
            return;
        }
        if ctx.buffer.buffered(Buttons::Lp, 2) && ctx.buffer.motion(Motions::Qcf, Buttons::Lp, 9) {
            println!("Qcf");
            ctx.next = Some(Box::new(standing::MediumPunch));
            return;
        }

        if ctx.buffer.just_pressed(Buttons::Hk) {
            ctx.next = Some(Box::new(standing::HeavyKick));
            return;
        }

        if ctx.buffer.just_pressed(Buttons::Mp) {
            ctx.next = Some(Box::new(standing::MediumPunch));
            return;
        }

        if ctx.buffer.forward() {
            ctx.next = Some(Box::new(standing::WalkForward));
            return;
        }

        if ctx.buffer.backward() {
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
        face_opponent(&mut ctx.physics);

        if ctx.buffer.buffered(Buttons::Lp, 2) && ctx.buffer.motion(Motions::Dpf, Buttons::Lp, 9) {
            println!("Dpf");
            ctx.next = Some(Box::new(standing::MediumPunch));
            return;
        }
        if ctx.buffer.buffered(Buttons::Lp, 2) && ctx.buffer.motion(Motions::Qcf, Buttons::Lp, 9) {
            println!("Qcf");
            ctx.next = Some(Box::new(standing::MediumPunch));
            return;
        }
        ctx.physics.set_forward_velocity(3000);
        if !ctx.buffer.forward() {
            ctx.next = Some(Box::new(standing::Idle));
        }
    }
    fn on_exit(&mut self, ctx: &mut Context) {
        ctx.physics.velocity.x = 0;
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
    fn on_update(&mut self, ctx: &mut Context) {
        face_opponent(&mut ctx.physics);
        ctx.physics.set_backward_velocity(3000);
        if !ctx.buffer.backward() {
            ctx.next = Some(Box::new(standing::Idle));
        }
    }
    fn on_exit(&mut self, ctx: &mut Context) {
        ctx.physics.velocity.x = 0;
        println!("{} -> St WalkBackward exit", ctx.player);
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
            face_opponent(&mut ctx.physics);
            if ctx.buffer.buffered(Buttons::Mp, 1) {
                ctx.next = Some(Box::new(standing::MediumPunch));
                return;
            }
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
            face_opponent(&mut ctx.physics);
            ctx.next = Some(Box::new(Idle));
        }
    }
    fn on_exit(&mut self, ctx: &mut Context) {
        println!("{} -> St HeavyKick exit", ctx.player);
    }
}

/// Conditionally flip the character to face the opponent if not already facing them.
pub fn face_opponent(physics: &mut Physics) -> bool {
    if !physics.facing_opponent {
        physics.facing_left = !physics.facing_left;
        physics.facing_opponent = true;
        return true;
    }
    false
}
