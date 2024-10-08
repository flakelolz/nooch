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
    pub fn set(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) -> bool {
        match self {
            States::Group(states) => states.set(ctx, buffer, physics),
            States::Standing(states) => states.set(ctx, buffer, physics),
            States::Crouching(states) => states.set(ctx, buffer, physics),
            States::Jumping(states) => states.set(ctx, buffer, physics),
            States::Ken(states) => states.set(ctx, buffer, physics),
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
    pub fn set(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) -> bool {
        match self {
            Group::All => {
                if Group::Specials.set(ctx, buffer, physics) {
                    return true;
                }
                if Group::CusNormals.set(ctx, buffer, physics) {
                    return true;
                }
                if Group::Normals.set(ctx, buffer, physics) {
                    return true;
                }
                if Group::Jumps.set(ctx, buffer, physics) {
                    return true;
                }
                if Group::Movement.set(ctx, buffer, physics) {
                    return true;
                }
                false
            }
            Group::Specials => {
                match ctx.name {
                    Name::Ken => {
                        if States::Ken(ken::Ken::Specials).set(ctx, buffer, physics) {
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
                        if States::Ken(ken::Ken::Normals).set(ctx, buffer, physics) {
                            return true;
                        }
                    }
                    Name::Ryu => todo!(),
                }
                false
            }
            Group::Normals => {
                if Group::CrNormals.set(ctx, buffer, physics) {
                    return true;
                }
                if Group::CusNormals.set(ctx, buffer, physics) {
                    return true;
                }
                if Group::StNormals.set(ctx, buffer, physics) {
                    return true;
                }
                false
            }

            Group::StNormals => {
                if Standing::HeavyKick.set(ctx, buffer, physics) {
                    return true;
                }
                if Standing::HeavyPunch.set(ctx, buffer, physics) {
                    return true;
                }
                if Standing::MediumKick.set(ctx, buffer, physics) {
                    return true;
                }
                if Standing::MediumPunch.set(ctx, buffer, physics) {
                    return true;
                }
                if Standing::LightKick.set(ctx, buffer, physics) {
                    return true;
                }
                if Standing::LightPunch.set(ctx, buffer, physics) {
                    return true;
                }
                false
            }

            Group::CrNormals => {
                if Crouching::HeavyKick.set(ctx, buffer, physics) {
                    return true;
                }
                if Crouching::HeavyPunch.set(ctx, buffer, physics) {
                    return true;
                }
                if Crouching::MediumKick.set(ctx, buffer, physics) {
                    return true;
                }
                if Crouching::MediumPunch.set(ctx, buffer, physics) {
                    return true;
                }
                if Crouching::LightKick.set(ctx, buffer, physics) {
                    return true;
                }
                if Crouching::LightPunch.set(ctx, buffer, physics) {
                    return true;
                }
                false
            }
            Group::AirNormals => {
                if Jumping::HeavyKick.set(ctx, buffer, physics) {
                    return true;
                }
                if Jumping::HeavyPunch.set(ctx, buffer, physics) {
                    return true;
                }
                if Jumping::MediumKick.set(ctx, buffer, physics) {
                    return true;
                }
                if Jumping::MediumPunch.set(ctx, buffer, physics) {
                    return true;
                }
                if Jumping::LightKick.set(ctx, buffer, physics) {
                    return true;
                }
                if Jumping::LightPunch.set(ctx, buffer, physics) {
                    return true;
                }

                false
            }
            Group::Movement => {
                if Standing::DashForward.set(ctx, buffer, physics) {
                    return true;
                }
                if Standing::DashBackward.set(ctx, buffer, physics) {
                    return true;
                }
                if Standing::WalkForward.set(ctx, buffer, physics) {
                    return true;
                }
                if Standing::WalkBackward.set(ctx, buffer, physics) {
                    return true;
                }
                false
            }
            Group::Dashes => {
                if Standing::DashForward.set(ctx, buffer, physics) {
                    return true;
                }
                if Standing::DashBackward.set(ctx, buffer, physics) {
                    return true;
                }
                false
            }
            Group::Walks => {
                if Standing::WalkForward.set(ctx, buffer, physics) {
                    return true;
                }
                if Standing::WalkBackward.set(ctx, buffer, physics) {
                    return true;
                }
                false
            }
            Group::Jumps => {
                if Jumping::Start.set(ctx, buffer, physics) {
                    return true;
                }
                if Jumping::Forward.set(ctx, buffer, physics) {
                    return true;
                }
                if Jumping::Backward.set(ctx, buffer, physics) {
                    return true;
                }
                if Jumping::Neutral.set(ctx, buffer, physics) {
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
    pub fn set(&self, ctx: &mut Context, buffer: &mut Buffer, _physics: &mut Physics) -> bool {
        match self {
            Standing::LightPunch => {
                if buffer.buffered(Buttons::Lp, buffer.cancels) && !buffer.down() {
                    ctx.next.replace(Box::new(standing::LightPunch));
                    return true;
                }
                false
            }
            Standing::MediumPunch => {
                if buffer.buffered(Buttons::Mp, buffer.attack) && !buffer.down() {
                    ctx.next.replace(Box::new(standing::MediumPunch));
                    return true;
                }
                false
            }
            Standing::HeavyPunch => {
                if buffer.buffered(Buttons::Hp, buffer.attack) && !buffer.down() {
                    ctx.next.replace(Box::new(standing::HeavyPunch));
                    return true;
                }
                false
            }
            Standing::LightKick => {
                if buffer.buffered(Buttons::Lk, buffer.cancels) && !buffer.down() {
                    ctx.next.replace(Box::new(standing::LightKick));
                    return true;
                }
                false
            }
            Standing::MediumKick => {
                if buffer.buffered(Buttons::Mk, buffer.attack) && !buffer.down() {
                    ctx.next.replace(Box::new(standing::MediumKick));
                    return true;
                }
                false
            }
            Standing::HeavyKick => {
                if buffer.buffered(Buttons::Hk, buffer.attack) && !buffer.down() {
                    ctx.next.replace(Box::new(standing::HeavyKick));
                    return true;
                }
                false
            }
            Standing::DashForward => {
                if buffer.dashed(Dashes::Forward, buffer.dash) && ctx.locks.dash_f {
                    ctx.next.replace(Box::new(standing::DashForward));
                    return true;
                }
                false
            }
            Standing::DashBackward => {
                if buffer.dashed(Dashes::Backward, buffer.dash) && ctx.locks.dash_b {
                    ctx.next.replace(Box::new(standing::DashBackward));
                    return true;
                }
                false
            }
            Standing::WalkForward => {
                if buffer.forward() {
                    ctx.next.replace(Box::new(standing::WalkForward));
                    return true;
                }
                false
            }
            Standing::WalkBackward => {
                if buffer.backward() {
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
    pub fn set(&self, ctx: &mut Context, buffer: &mut Buffer, _physics: &mut Physics) -> bool {
        match self {
            Crouching::Start => {
                if buffer.down() {
                    ctx.next.replace(Box::new(crouching::Start));
                    return true;
                }
                false
            }
            Crouching::End => {
                if !buffer.down() {
                    ctx.next.replace(Box::new(crouching::End));
                    return true;
                }
                false
            }
            Crouching::LightPunch => {
                if buffer.buffered(Buttons::Lp, buffer.cancels) && buffer.down() {
                    ctx.next.replace(Box::new(crouching::LightPunch));
                    return true;
                }
                false
            }
            Crouching::MediumPunch => {
                if buffer.buffered(Buttons::Mp, buffer.attack) && buffer.down() {
                    ctx.next.replace(Box::new(crouching::MediumPunch));
                    return true;
                }
                false
            }
            Crouching::HeavyPunch => {
                if buffer.buffered(Buttons::Hp, buffer.attack) && buffer.down() {
                    ctx.next.replace(Box::new(crouching::HeavyPunch));
                    return true;
                }
                false
            }
            Crouching::LightKick => {
                if buffer.buffered(Buttons::Lk, buffer.cancels) && buffer.down() {
                    ctx.next.replace(Box::new(crouching::LightKick));
                    return true;
                }
                false
            }
            Crouching::MediumKick => {
                if buffer.buffered(Buttons::Mk, buffer.attack) && buffer.down() {
                    ctx.next.replace(Box::new(crouching::MediumKick));
                    return true;
                }
                false
            }
            Crouching::HeavyKick => {
                if buffer.buffered(Buttons::Hk, buffer.attack) && buffer.down() {
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
    pub fn set(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) -> bool {
        match self {
            Jumping::Start => {
                if buffer.up() && !physics.airborne {
                    handle_jump_flags(ctx, buffer, physics);
                    ctx.next.replace(Box::new(jumping::Start));
                    return true;
                }
            }
            Jumping::Neutral => {
                if buffer.up() {
                    ctx.next.replace(Box::new(jumping::Neutral));
                    return true;
                }
            }
            Jumping::Forward => {
                if buffer.up_forward() {
                    ctx.next.replace(Box::new(jumping::Forward));
                    return true;
                }
            }
            Jumping::Backward => {
                if buffer.up_backward() {
                    ctx.next.replace(Box::new(jumping::Backward));
                    return true;
                }
            }
            Jumping::End => {
                if !buffer.up() && physics.airborne {
                    ctx.next.replace(Box::new(jumping::End));
                    return true;
                }
            }
            Jumping::LightPunch => {
                if buffer.buffered(Buttons::Lp, buffer.attack) && physics.airborne {
                    ctx.next.replace(Box::new(jumping::LightPunch));
                    return true;
                }
            }
            Jumping::MediumPunch => {
                if buffer.buffered(Buttons::Mp, buffer.attack) && physics.airborne {
                    ctx.next.replace(Box::new(jumping::MediumPunch));
                    return true;
                }
            }
            Jumping::HeavyPunch => {
                if buffer.buffered(Buttons::Hp, buffer.attack) && physics.airborne {
                    ctx.next.replace(Box::new(jumping::HeavyPunch));
                    return true;
                }
            }
            Jumping::LightKick => {
                if buffer.buffered(Buttons::Lk, buffer.attack) && physics.airborne {
                    ctx.next.replace(Box::new(jumping::LightKick));
                    return true;
                }
            }
            Jumping::MediumKick => {
                if buffer.buffered(Buttons::Mk, buffer.attack) && physics.airborne {
                    ctx.next.replace(Box::new(jumping::MediumKick));
                    return true;
                }
            }
            Jumping::HeavyKick => {
                if buffer.buffered(Buttons::Hk, buffer.attack) && physics.airborne {
                    ctx.next.replace(Box::new(jumping::HeavyKick));
                    return true;
                }
            }
        }
        false
    }
}
