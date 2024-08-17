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
    pub fn set(&self, ctx: &mut Context) -> bool {
        match self {
            Ken::Normals => {
                // if Normals::HeavyPunch.set(ctx) {
                //     return true;
                // }
                // if Normals::MediumPunch.set(ctx) {
                //     return true;
                // }
                // if Normals::LightPunch.set(ctx) {
                //     return true;
                // }
                false
            }
            Ken::HeavyPunch => {
                if Normals::HeavyPunch.set(ctx) {
                    return true;
                }
                false
            }
            Ken::MediumPunch => {
                if Normals::MediumPunch.set(ctx) {
                    return true;
                }
                false
            }
            Ken::LightPunch => {
                if Normals::LightPunch.set(ctx) {
                    return true;
                }
                false
            }
            Ken::Specials => {
                // Priority Hadouken with half-circle motion
                {
                    let hcf = [4, 1, 2, 3, 6];
                    if ctx.buffer.buffered(Buttons::Punches, ctx.buffer.cancels)
                        && ctx.buffer.motion_custom(&hcf, Buttons::Punches, 9)
                    {
                        println!("Priority!!!");
                        ctx.next.replace(Box::new(ken::Hadouken));
                        return true;
                    }
                }
                if Specials::Shoryuken.set(ctx) {
                    return true;
                }
                if Specials::Hadouken.set(ctx) {
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
    pub fn set(&self, ctx: &mut Context) -> bool {
        // let distance = world_to_screen_num(ctx.physics.distance as i32);
        match self {
            Normals::LightPunch => {
                if ctx.buffer.buffered(Buttons::Lp, ctx.buffer.attack)
                // && distance < 35
                {
                    ctx.next.replace(Box::new(ken::LightPunch));
                    return true;
                }
                false
            }
            Normals::MediumPunch => {
                if ctx.buffer.buffered(Buttons::Mp, ctx.buffer.attack)
                // && distance < 40
                {
                    ctx.next.replace(Box::new(ken::MediumPunch));
                    return true;
                }
                false
            }
            Normals::HeavyPunch => {
                if ctx.buffer.buffered(Buttons::Hp, ctx.buffer.attack)
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
    pub fn set(&self, ctx: &mut Context) -> bool {
        match self {
            Specials::Hadouken => {
                if ctx.buffer.buffered(Buttons::Punches, ctx.buffer.cancels)
                    && ctx.buffer.motion(Motions::Qcf, Buttons::Punches, 9)
                    && !ctx.physics.airborne
                {
                    ctx.next.replace(Box::new(ken::Hadouken));
                    return true;
                }
                false
            }
            Specials::Shoryuken => {
                if ctx.buffer.motion(Motions::Dpf, Buttons::Lp, 9)
                    && ctx.buffer.buffered(Buttons::Lp, 20)
                    && !ctx.physics.airborne
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
    fn on_enter(&mut self, ctx: &mut Context) {
        println!("{} -> Cl LightPunch enter", ctx.player);
    }
    fn on_update(&mut self, ctx: &mut Context) {
        if ctx.elapsed > ctx.total {
            common_standing_attack_transitions(ctx);
        }
    }
    fn on_exit(&mut self, ctx: &mut Context) {
        println!("{} -> Cl LightPunch exit", ctx.player);
    }
}

pub struct MediumPunch;
impl State for MediumPunch {
    fn name(&self) -> &'static str {
        "Cl HeavyPunch"
    }
    fn on_enter(&mut self, ctx: &mut Context) {
        println!("{} -> Cl HeavyPunch enter", ctx.player);
    }
    fn on_update(&mut self, ctx: &mut Context) {
        if ctx.elapsed > ctx.total {
            common_standing_attack_transitions(ctx);
        }
    }
    fn on_exit(&mut self, ctx: &mut Context) {
        println!("{} -> Cl HeavyPunch exit", ctx.player);
    }
}

pub struct HeavyPunch;
impl State for HeavyPunch {
    fn name(&self) -> &'static str {
        "Cl HeavyPunch"
    }
    fn on_enter(&mut self, ctx: &mut Context) {
        println!("{} -> Cl HeavyPunch enter", ctx.player);
    }
    fn on_update(&mut self, ctx: &mut Context) {
        if ctx.elapsed > ctx.total {
            common_standing_attack_transitions(ctx);
        }
    }
    fn on_exit(&mut self, ctx: &mut Context) {
        println!("{} -> Cl HeavyPunch exit", ctx.player);
    }
}

pub struct Hadouken;
impl State for Hadouken {
    fn name(&self) -> &'static str {
        "Ken Hadouken"
    }

    fn on_enter(&mut self, ctx: &mut Context) {
        println!("{} -> Ken Hadouken enter", ctx.player);
        // TODO: Set how fast a fireball is going to move based on the button currently pressed
        println!(
            "lp: {}, mp: {}, hp: {}",
            ctx.buffer.pressed(Buttons::Lp),
            ctx.buffer.pressed(Buttons::Mp),
            ctx.buffer.pressed(Buttons::Hp)
        );
        let offset = IVec2 {
            x: {
                if ctx.physics.facing_left {
                    -70000
                } else {
                    70000
                }
            },
            y: 0,
        };
        let mut physics = Physics {
            position: ctx.physics.position + offset,
            facing_left: ctx.physics.facing_left,
            facing_opponent: ctx.physics.facing_opponent,
            ..Default::default()
        };

        if ctx.buffer.pressed(Buttons::Lp) {
            physics.set_forward_velocity(3000);
        }
        if ctx.buffer.pressed(Buttons::Mp) {
            physics.set_forward_velocity(4000);
        }
        if ctx.buffer.pressed(Buttons::Hp) {
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

    fn on_update(&mut self, ctx: &mut Context) {
        if ctx.elapsed > ctx.total {
            common_standing_attack_transitions(ctx);
        }
    }

    fn on_exit(&mut self, ctx: &mut Context) {
        println!("{} -> Ken Hadouken exit", ctx.player);
    }
}

pub struct ShoryukenL;
impl State for ShoryukenL {
    fn name(&self) -> &'static str {
        "Ken ShoryukenL"
    }

    fn on_enter(&mut self, ctx: &mut Context) {
        println!("{} -> Ken ShoryukenL enter", ctx.player);
    }

    fn on_update(&mut self, ctx: &mut Context) {
        if ctx.elapsed == 3
        // && ctx.reaction.hitstop == 0
        {
            ctx.physics.position.x += 5000;
        }
        if ctx.elapsed == 5 {
            ctx.physics.velocity.y = 9000;
            ctx.physics.acceleration.y = -750;
            ctx.physics.set_forward_velocity(1000);
            ctx.physics.airborne = true;
        }

        if ctx.elapsed > 14 {
            ctx.physics.velocity.x = 0;
        }

        if ctx.elapsed > 29 {
            ctx.physics.position.y = 0;
            ctx.physics.velocity = IVec2::ZERO;
            ctx.physics.acceleration.y = 0;
            ctx.physics.airborne = false;
        }
        if ctx.elapsed == 30 {
            ctx.physics.position.x += 4000;
        }

        if ctx.elapsed > ctx.total {
            common_standing_attack_transitions(ctx);
        }
    }

    fn on_exit(&mut self, ctx: &mut Context) {
        println!("{} -> Ken ShoryukenL exit", ctx.player);
    }
}
