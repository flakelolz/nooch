mod common;
mod custom;

use crate::prelude::*;

pub use self::common::*;
pub use self::custom::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum States {
    Group(Group),
    Standing(Standing),
    Crouching(Crouching),
    Jumping(Jumping),
    Ken(ken::Ken),
}

impl States {
    pub fn set(&self, ctx: &mut Context) -> bool {
        match self {
            States::Group(states) => states.set(ctx),
            States::Standing(states) => states.set(ctx),
            States::Crouching(states) => states.set(ctx),
            States::Jumping(states) => states.set(ctx),
            States::Ken(states) => states.set(ctx),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Group {
    /// All states
    All,
    /// Every normal attack
    Normals,
    /// Standing normals
    StNormals,
    /// Crouching normals
    CrNormals,
    /// Air normals
    AirNormals,
    /// Custom character normals
    CusNormals,
    /// Special attacks
    Specials,
    /// Every movement option
    Movement,
    /// Dahs forward or backward
    Dashes,
    /// Walk forward or backward
    Walks,
    /// Jumps
    Jumps,
}

impl Group {
    pub fn set(&self, ctx: &mut Context) -> bool {
        match self {
            Group::All => {
                if Group::Specials.set(ctx) {
                    return true;
                }
                if Group::CusNormals.set(ctx) {
                    return true;
                }
                if Group::Normals.set(ctx) {
                    return true;
                }
                if Group::Jumps.set(ctx) {
                    return true;
                }
                if Group::Movement.set(ctx) {
                    return true;
                }
                false
            }
            Group::Specials => {
                match ctx.name {
                    Name::Ken => {
                        if States::Ken(ken::Ken::Specials).set(ctx) {
                            return true;
                        }
                    }
                    Name::Ryu => todo!(),
                }
                false
            }
            Group::CusNormals => {
                match ctx.name {
                    Name::Ken => {
                        if States::Ken(ken::Ken::Normals).set(ctx) {
                            return true;
                        }
                    }
                    Name::Ryu => todo!(),
                }
                false
            }
            Group::Normals => {
                if Group::CrNormals.set(ctx) {
                    return true;
                }
                if Group::CusNormals.set(ctx) {
                    return true;
                }
                if Group::StNormals.set(ctx) {
                    return true;
                }
                false
            }

            Group::StNormals => {
                if Standing::HeavyKick.set(ctx) {
                    return true;
                }
                if Standing::HeavyPunch.set(ctx) {
                    return true;
                }
                if Standing::MediumKick.set(ctx) {
                    return true;
                }
                if Standing::MediumPunch.set(ctx) {
                    return true;
                }
                if Standing::LightKick.set(ctx) {
                    return true;
                }
                if Standing::LightPunch.set(ctx) {
                    return true;
                }
                false
            }

            Group::CrNormals => {
                if Crouching::HeavyKick.set(ctx) {
                    return true;
                }
                if Crouching::HeavyPunch.set(ctx) {
                    return true;
                }
                if Crouching::MediumKick.set(ctx) {
                    return true;
                }
                if Crouching::MediumPunch.set(ctx) {
                    return true;
                }
                if Crouching::LightKick.set(ctx) {
                    return true;
                }
                if Crouching::LightPunch.set(ctx) {
                    return true;
                }
                false
            }
            Group::AirNormals => {
                if Jumping::HeavyKick.set(ctx) {
                    return true;
                }
                if Jumping::HeavyPunch.set(ctx) {
                    return true;
                }
                if Jumping::MediumKick.set(ctx) {
                    return true;
                }
                if Jumping::MediumPunch.set(ctx) {
                    return true;
                }
                if Jumping::LightKick.set(ctx) {
                    return true;
                }
                if Jumping::LightPunch.set(ctx) {
                    return true;
                }

                false
            }
            Group::Movement => {
                if Standing::DashForward.set(ctx) {
                    return true;
                }
                if Standing::DashBackward.set(ctx) {
                    return true;
                }
                if Standing::WalkForward.set(ctx) {
                    return true;
                }
                if Standing::WalkBackward.set(ctx) {
                    return true;
                }
                false
            }
            Group::Dashes => {
                if Standing::DashForward.set(ctx) {
                    return true;
                }
                if Standing::DashBackward.set(ctx) {
                    return true;
                }
                false
            }
            Group::Walks => {
                if Standing::WalkForward.set(ctx) {
                    return true;
                }
                if Standing::WalkBackward.set(ctx) {
                    return true;
                }
                false
            }
            Group::Jumps => {
                if Jumping::Start.set(ctx) {
                    return true;
                }
                if Jumping::Forward.set(ctx) {
                    return true;
                }
                if Jumping::Backward.set(ctx) {
                    return true;
                }
                if Jumping::Neutral.set(ctx) {
                    return true;
                }
                false
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Standing {
    LightPunch,
    MediumPunch,
    HeavyPunch,
    LightKick,
    MediumKick,
    HeavyKick,
    DashForward,
    DashBackward,
    WalkForward,
    WalkBackward,
}

impl Standing {
    pub fn set(&self, ctx: &mut Context) -> bool {
        match self {
            Standing::LightPunch => {
                if ctx.buffer.buffered(Buttons::Lp, ctx.buffer.cancels) && !ctx.buffer.down() {
                    ctx.next.replace(Box::new(standing::LightPunch));
                    return true;
                }
                false
            }
            Standing::MediumPunch => {
                if ctx.buffer.buffered(Buttons::Mp, ctx.buffer.attack) && !ctx.buffer.down() {
                    ctx.next.replace(Box::new(standing::MediumPunch));
                    return true;
                }
                false
            }
            Standing::HeavyPunch => {
                if ctx.buffer.buffered(Buttons::Hp, ctx.buffer.attack) && !ctx.buffer.down() {
                    ctx.next.replace(Box::new(standing::HeavyPunch));
                    return true;
                }
                false
            }
            Standing::LightKick => {
                if ctx.buffer.buffered(Buttons::Lk, ctx.buffer.cancels) && !ctx.buffer.down() {
                    ctx.next.replace(Box::new(standing::LightKick));
                    return true;
                }
                false
            }
            Standing::MediumKick => {
                if ctx.buffer.buffered(Buttons::Mk, ctx.buffer.attack) && !ctx.buffer.down() {
                    ctx.next.replace(Box::new(standing::MediumKick));
                    return true;
                }
                false
            }
            Standing::HeavyKick => {
                if ctx.buffer.buffered(Buttons::Hk, ctx.buffer.attack) && !ctx.buffer.down() {
                    ctx.next.replace(Box::new(standing::HeavyKick));
                    return true;
                }
                false
            }
            Standing::DashForward => {
                if ctx.buffer.dashed(Dashes::Forward, ctx.buffer.dash) && ctx.locks.dash_f {
                    ctx.next.replace(Box::new(standing::DashForward));
                    return true;
                }
                false
            }
            Standing::DashBackward => {
                if ctx.buffer.dashed(Dashes::Backward, ctx.buffer.dash) && ctx.locks.dash_b {
                    ctx.next.replace(Box::new(standing::DashBackward));
                    return true;
                }
                false
            }
            Standing::WalkForward => {
                if ctx.buffer.forward() {
                    ctx.next.replace(Box::new(standing::WalkForward));
                    return true;
                }
                false
            }
            Standing::WalkBackward => {
                if ctx.buffer.backward() {
                    ctx.next.replace(Box::new(standing::WalkBackward));
                    return true;
                }
                false
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub enum Crouching {
    Start,
    End,
    LightPunch,
    MediumPunch,
    HeavyPunch,
    LightKick,
    MediumKick,
    HeavyKick,
}

impl Crouching {
    pub fn set(&self, ctx: &mut Context) -> bool {
        match self {
            Crouching::Start => {
                if ctx.buffer.down() {
                    ctx.next.replace(Box::new(crouching::Start));
                    return true;
                }
                false
            }
            Crouching::End => {
                if !ctx.buffer.down() {
                    ctx.next.replace(Box::new(crouching::End));
                    return true;
                }
                false
            }
            Crouching::LightPunch => {
                if ctx.buffer.buffered(Buttons::Lp, ctx.buffer.cancels) && ctx.buffer.down() {
                    ctx.next.replace(Box::new(crouching::LightPunch));
                    return true;
                }
                false
            }
            Crouching::MediumPunch => {
                if ctx.buffer.buffered(Buttons::Mp, ctx.buffer.attack) && ctx.buffer.down() {
                    ctx.next.replace(Box::new(crouching::MediumPunch));
                    return true;
                }
                false
            }
            Crouching::HeavyPunch => {
                if ctx.buffer.buffered(Buttons::Hp, ctx.buffer.attack) && ctx.buffer.down() {
                    ctx.next.replace(Box::new(crouching::HeavyPunch));
                    return true;
                }
                false
            }
            Crouching::LightKick => {
                if ctx.buffer.buffered(Buttons::Lk, ctx.buffer.cancels) && ctx.buffer.down() {
                    ctx.next.replace(Box::new(crouching::LightKick));
                    return true;
                }
                false
            }
            Crouching::MediumKick => {
                if ctx.buffer.buffered(Buttons::Mk, ctx.buffer.attack) && ctx.buffer.down() {
                    ctx.next.replace(Box::new(crouching::MediumKick));
                    return true;
                }
                false
            }
            Crouching::HeavyKick => {
                if ctx.buffer.buffered(Buttons::Hk, ctx.buffer.attack) && ctx.buffer.down() {
                    ctx.next.replace(Box::new(crouching::HeavyKick));
                    return true;
                }
                false
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Jumping {
    Start,
    Neutral,
    Forward,
    Backward,
    End,
    LightPunch,
    MediumPunch,
    HeavyPunch,
    LightKick,
    MediumKick,
    HeavyKick,
}

impl Jumping {
    pub fn set(&self, ctx: &mut Context) -> bool {
        match self {
            Jumping::Start => {
                if ctx.buffer.up() && !ctx.physics.airborne {
                    handle_jump_flags(ctx);
                    ctx.next.replace(Box::new(jumping::Start));
                    return true;
                }
            }
            Jumping::Neutral => {
                if ctx.buffer.up() {
                    ctx.next.replace(Box::new(jumping::Neutral));
                    return true;
                }
            }
            Jumping::Forward => {
                if ctx.buffer.up_forward() {
                    ctx.next.replace(Box::new(jumping::Forward));
                    return true;
                }
            }
            Jumping::Backward => {
                if ctx.buffer.up_backward() {
                    ctx.next.replace(Box::new(jumping::Backward));
                    return true;
                }
            }
            Jumping::End => {
                if !ctx.buffer.up() && ctx.physics.airborne {
                    ctx.next.replace(Box::new(jumping::End));
                    return true;
                }
            }
            Jumping::LightPunch => {
                if ctx.buffer.buffered(Buttons::Lp, ctx.buffer.attack) && ctx.physics.airborne {
                    ctx.next.replace(Box::new(jumping::LightPunch));
                    return true;
                }
            }
            Jumping::MediumPunch => {
                if ctx.buffer.buffered(Buttons::Mp, ctx.buffer.attack) && ctx.physics.airborne {
                    ctx.next.replace(Box::new(jumping::MediumPunch));
                    return true;
                }
            }
            Jumping::HeavyPunch => {
                if ctx.buffer.buffered(Buttons::Hp, ctx.buffer.attack) && ctx.physics.airborne {
                    ctx.next.replace(Box::new(jumping::HeavyPunch));
                    return true;
                }
            }
            Jumping::LightKick => {
                if ctx.buffer.buffered(Buttons::Lk, ctx.buffer.attack) && ctx.physics.airborne {
                    ctx.next.replace(Box::new(jumping::LightKick));
                    return true;
                }
            }
            Jumping::MediumKick => {
                if ctx.buffer.buffered(Buttons::Mk, ctx.buffer.attack) && ctx.physics.airborne {
                    ctx.next.replace(Box::new(jumping::MediumKick));
                    return true;
                }
            }
            Jumping::HeavyKick => {
                if ctx.buffer.buffered(Buttons::Hk, ctx.buffer.attack) && ctx.physics.airborne {
                    ctx.next.replace(Box::new(jumping::HeavyKick));
                    return true;
                }
            }
        }
        false
    }
}
