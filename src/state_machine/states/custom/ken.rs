use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Ken {
    LightPunch,
    MediumPunch,
    HeavyPunch,
    Normals,
    Specials,
}

impl Ken {
    pub fn set(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) -> bool {
        match self {
            Ken::Normals => {
                // if Normals::HeavyPunch.set(ctx, buffer, physics) {
                //     return true;
                // }
                // if Normals::MediumPunch.set(ctx, buffer, physics) {
                //     return true;
                // }
                // if Normals::LightPunch.set(ctx, buffer, physics) {
                //     return true;
                // }
                false
            }
            Ken::HeavyPunch => {
                if Normals::HeavyPunch.set(ctx, buffer, physics) {
                    return true;
                }
                false
            }
            Ken::MediumPunch => {
                if Normals::MediumPunch.set(ctx, buffer, physics) {
                    return true;
                }
                false
            }
            Ken::LightPunch => {
                if Normals::LightPunch.set(ctx, buffer, physics) {
                    return true;
                }
                false
            }
            Ken::Specials => {
                // Priority Hadouken with half-circle motion
                {
                    let hcf = [4, 1, 2, 3, 6];
                    if buffer.buffered(Buttons::Punches, buffer.cancels)
                        && buffer.motion_custom(&hcf, Buttons::Punches, 9)
                    {
                        println!("Priority!!!");
                        ctx.next.replace(Box::new(ken::Hadouken));
                        return true;
                    }
                }
                if Specials::Shoryuken.set(ctx, buffer, physics) {
                    return true;
                }
                if Specials::Hadouken.set(ctx, buffer, physics) {
                    return true;
                }
                false
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Normals {
    LightPunch,
    MediumPunch,
    HeavyPunch,
    BckMediumKick,
    FwdMediumKick,
    FwdHeavyKick,
}

impl Normals {
    pub fn set(&self, ctx: &mut Context, buffer: &mut Buffer, _physics: &mut Physics) -> bool {
        // let distance = world_to_screen_num(physics.distance as i32);
        match self {
            Normals::LightPunch => {
                if buffer.buffered(Buttons::Lp, buffer.attack)
                // && distance < 35
                {
                    ctx.next.replace(Box::new(ken::LightPunch));
                    return true;
                }
                false
            }
            Normals::MediumPunch => {
                if buffer.buffered(Buttons::Mp, buffer.attack)
                // && distance < 40
                {
                    ctx.next.replace(Box::new(ken::MediumPunch));
                    return true;
                }
                false
            }
            Normals::HeavyPunch => {
                if buffer.buffered(Buttons::Hp, buffer.attack)
                // && distance < 48
                {
                    ctx.next.replace(Box::new(ken::HeavyPunch));
                    return true;
                }
                false
            }
            Normals::BckMediumKick => todo!(),
            Normals::FwdMediumKick => todo!(),
            Normals::FwdHeavyKick => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Specials {
    Hadouken,
    Shoryuken,
}

impl Specials {
    pub fn set(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) -> bool {
        match self {
            Specials::Hadouken => {
                if buffer.buffered(Buttons::Punches, buffer.cancels)
                    && buffer.motion(Motions::Qcf, Buttons::Punches, 9)
                    && !physics.airborne
                {
                    ctx.next.replace(Box::new(ken::Hadouken));
                    return true;
                }
                false
            }
            Specials::Shoryuken => {
                if buffer.motion(Motions::Dpf, Buttons::Lp, 9)
                    && buffer.buffered(Buttons::Lp, 20)
                    && !physics.airborne
                {
                    ctx.next.replace(Box::new(ShoryukenL));
                    return true;
                }
                false
            }
        }
    }
}

pub struct LightPunch;
impl State for LightPunch {
    fn name(&self) -> &'static str {
        "Cl LightPunch"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Cl LightPunch enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        if ctx.elapsed > ctx.total {
            common_standing_attack_transitions(ctx, buffer, physics);
        }
    }
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Cl LightPunch exit", ctx.player);
    }
}

pub struct MediumPunch;
impl State for MediumPunch {
    fn name(&self) -> &'static str {
        "Cl HeavyPunch"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Cl HeavyPunch enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        if ctx.elapsed > ctx.total {
            common_standing_attack_transitions(ctx, buffer, physics);
        }
    }
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Cl HeavyPunch exit", ctx.player);
    }
}

pub struct HeavyPunch;
impl State for HeavyPunch {
    fn name(&self) -> &'static str {
        "Cl HeavyPunch"
    }
    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Cl HeavyPunch enter", ctx.player);
    }
    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        if ctx.elapsed > ctx.total {
            common_standing_attack_transitions(ctx, buffer, physics);
        }
    }
    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Cl HeavyPunch exit", ctx.player);
    }
}

pub struct Hadouken;
impl State for Hadouken {
    fn name(&self) -> &'static str {
        "Ken Hadouken"
    }

    fn on_enter(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        println!("{} -> Ken Hadouken enter", ctx.player);
        // TODO: Set how fast a fireball is going to move based on the button currently pressed
        println!(
            "lp: {}, mp: {}, hp: {}",
            buffer.pressed(Buttons::Lp),
            buffer.pressed(Buttons::Mp),
            buffer.pressed(Buttons::Hp)
        );
        let offset = IVec2 {
            x: {
                if physics.facing_left {
                    -70000
                } else {
                    70000
                }
            },
            y: 0,
        };
        let mut physics = Physics {
            position: physics.position + offset,
            facing_left: physics.facing_left,
            facing_opponent: physics.facing_opponent,
            ..Default::default()
        };

        if buffer.pressed(Buttons::Lp) {
            physics.set_forward_velocity(3000);
        }
        if buffer.pressed(Buttons::Mp) {
            physics.set_forward_velocity(4000);
        }
        if buffer.pressed(Buttons::Hp) {
            physics.set_forward_velocity(5000);
        }
        // _context.spawn.replace(Projectile {
        //     owner: None,
        //     name: "Obj Fireball".into(),
        //     physics,
        //     timing: 13,
        //     duration: 100,
        // });
    }

    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        if ctx.elapsed > ctx.total {
            common_standing_attack_transitions(ctx, buffer, physics);
        }
    }

    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Ken Hadouken exit", ctx.player);
    }
}

pub struct ShoryukenL;
impl State for ShoryukenL {
    fn name(&self) -> &'static str {
        "Ken ShoryukenL"
    }

    fn on_enter(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Ken ShoryukenL enter", ctx.player);
    }

    fn on_update(&self, ctx: &mut Context, buffer: &mut Buffer, physics: &mut Physics) {
        if ctx.elapsed == 3
        // && ctx.reaction.hitstop == 0
        {
            physics.position.x += 5000;
        }
        if ctx.elapsed == 5 {
            physics.velocity.y = 9000;
            physics.acceleration.y = -750;
            physics.set_forward_velocity(1000);
            physics.airborne = true;
        }

        if ctx.elapsed > 14 {
            physics.velocity.x = 0;
        }

        if ctx.elapsed > 29 {
            physics.position.y = 0;
            physics.velocity = IVec2::ZERO;
            physics.acceleration.y = 0;
            physics.airborne = false;
        }
        if ctx.elapsed == 30 {
            physics.position.x += 4000;
        }

        if ctx.elapsed > ctx.total {
            common_standing_attack_transitions(ctx, buffer, physics);
        }
    }

    fn on_exit(&self, ctx: &mut Context, _buffer: &mut Buffer, _physics: &mut Physics) {
        println!("{} -> Ken ShoryukenL exit", ctx.player);
    }
}
