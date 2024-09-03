use crate::prelude::*;

pub struct UpperWeak;
impl State for UpperWeak {
    fn name(&self) -> &'static str {
        "Rxn UpperWeak"
    }

    fn on_enter(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn UpperWeak on_enter");
    }

    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        common_standing_reaction_transitions(ctx, buffer, physics);
    }

    fn on_exit(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn UpperWeak on_exit");
    }
}

pub struct UpperMid;
impl State for UpperMid {
    fn name(&self) -> &'static str {
        "Rxn UpperMid"
    }

    fn on_enter(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn UpperMid on_enter");
    }

    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        common_standing_reaction_transitions(ctx, buffer, physics);
    }

    fn on_exit(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn UpperMid on_exit");
    }
}

pub struct UpperStrong;
impl State for UpperStrong {
    fn name(&self) -> &'static str {
        "Rxn UpperStrong"
    }

    fn on_enter(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn UpperStrong on_enter");
    }

    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        common_standing_reaction_transitions(ctx, buffer, physics);
    }

    fn on_exit(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn UpperStrong on_exit");
    }
}

pub struct UpperRising;
impl State for UpperRising {
    fn name(&self) -> &'static str {
        "Rxn UpperRising"
    }

    fn on_enter(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn UpperRising on_enter");
    }

    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        common_standing_reaction_transitions(ctx, buffer, physics);
    }

    fn on_exit(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn UpperRising on_exit");
    }
}

pub struct FrontSpin;
impl State for FrontSpin {
    fn name(&self) -> &'static str {
        "Rxn FrontSpin"
    }

    fn on_enter(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn FrontSpin on_enter");
    }

    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        common_standing_reaction_transitions(ctx, buffer, physics);
    }

    fn on_exit(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn FrontSpin on_exit");
    }
}

pub struct BackSpin;
impl State for BackSpin {
    fn name(&self) -> &'static str {
        "Rxn BackSpin"
    }

    fn on_enter(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn BackSpin on_enter");
    }

    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        common_standing_reaction_transitions(ctx, buffer, physics);
    }

    fn on_exit(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn BackSpin on_exit");
    }
}

pub struct LowerWeak;
impl State for LowerWeak {
    fn name(&self) -> &'static str {
        "Rxn LowerWeak"
    }

    fn on_enter(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn LowerWeak on_enter");
    }

    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        common_standing_reaction_transitions(ctx, buffer, physics);
    }

    fn on_exit(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn LowerWeak on_exit");
    }
}

pub struct LowerMid;
impl State for LowerMid {
    fn name(&self) -> &'static str {
        "Rxn LowerMid"
    }

    fn on_enter(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn LowerMid on_enter");
    }

    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        common_standing_reaction_transitions(ctx, buffer, physics);
    }

    fn on_exit(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn LowerMid on_exit");
    }
}

pub struct LowerStrong;
impl State for LowerStrong {
    fn name(&self) -> &'static str {
        "Rxn LowerStrong"
    }

    fn on_enter(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn LowerStrong on_enter");
    }

    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        common_standing_reaction_transitions(ctx, buffer, physics);
    }

    fn on_exit(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn LowerStrong on_exit");
    }
}

pub struct LowerRising;
impl State for LowerRising {
    fn name(&self) -> &'static str {
        "Rxn LowerRising"
    }

    fn on_enter(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn LowerRising on_enter");
    }

    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        common_standing_reaction_transitions(ctx, buffer, physics);
    }

    fn on_exit(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn LowerRising on_exit");
    }
}

pub struct CrouchWeak;
impl State for CrouchWeak {
    fn name(&self) -> &'static str {
        "Rxn CrouchWeak"
    }

    fn on_enter(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn CrouchWeak on_enter");
    }

    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        common_crouching_reaction_transitions(ctx, buffer, physics);
    }

    fn on_exit(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn CrouchWeak on_exit");
    }
}

pub struct CrouchMid;
impl State for CrouchMid {
    fn name(&self) -> &'static str {
        "Rxn CrouchMid"
    }

    fn on_enter(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn CrouchMid on_enter");
    }

    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        common_crouching_reaction_transitions(ctx, buffer, physics);
    }

    fn on_exit(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn CrouchMid on_exit");
    }
}

pub struct CrouchStrong;
impl State for CrouchStrong {
    fn name(&self) -> &'static str {
        "Rxn CrouchStrong"
    }

    fn on_enter(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn CrouchStrong on_enter");
    }

    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        common_crouching_reaction_transitions(ctx, buffer, physics);
    }

    fn on_exit(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn CrouchStrong on_exit");
    }
}

pub struct GrdStandPre;
impl State for GrdStandPre {
    fn name(&self) -> &'static str {
        "Rxn GrdStandPre"
    }

    fn on_enter(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn GrdStandPre on_enter");
    }

    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        common_standing_reaction_transitions(ctx, buffer, physics);
    }

    fn on_exit(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn GrdStandPre on_exit");
    }
}

pub struct GrdStandEnd;
impl State for GrdStandEnd {
    fn name(&self) -> &'static str {
        "Rxn GrdStandEnd"
    }

    fn on_enter(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn GrdStandEnd on_enter");
        // context.ctx.reaction.blocking = true;
    }

    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        common_standing_reaction_transitions(ctx, buffer, physics);
    }

    fn on_exit(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn GrdStandEnd on_exit");
        // context.ctx.reaction.blocking = false;
    }
}

pub struct GrdCrouchPre;
impl State for GrdCrouchPre {
    fn name(&self) -> &'static str {
        "Rxn GrdCrouchPre"
    }

    fn on_enter(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn GrdCrouchPre on_enter");
    }

    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        common_crouching_reaction_transitions(ctx, buffer, physics);
    }

    fn on_exit(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn GrdCrouchPre on_exit");
    }
}

pub struct GrdCrouchEnd;
impl State for GrdCrouchEnd {
    fn name(&self) -> &'static str {
        "Rxn GrdCrouchEnd"
    }

    fn on_enter(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn GrdCrouchEnd on_enter");
        // context.ctx.reaction.blocking = true;
    }

    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        common_crouching_reaction_transitions(ctx, buffer, physics);
    }

    fn on_exit(&self, _ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("Rxn GrdCrouchEnd on_exit");
        // context.ctx.reaction.blocking = false;
    }
}
