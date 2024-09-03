use crate::prelude::*;

#[derive(Default)]
pub struct Context {
    pub player: Player,
    pub name: Name,
    pub data: CharacterData,
    pub next: Option<Box<dyn State>>,
    pub elapsed: u32,
    pub total: u32,
    pub reaction: Reaction,
    pub flags: Flags,
    pub locks: Locks,
}

impl Context {
    pub fn new(player: Player, name: Name, data: CharacterData) -> Self {
        Self {
            player,
            name,
            data,
            next: None,
            elapsed: 1,
            total: 1,
            ..Default::default()
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Reaction {
    /// Attacker's attack has made contact with hurtbox
    pub has_hit: bool,
    /// Attacker's attack was blocked
    pub _blocked: bool,
    /// Attacker's attack can be canceled out of
    pub _can_cancel: bool,
    /// Everyone's hitstop
    pub hitstop: u32,
    /// Everyone's hitstun
    pub hitstun: u32,
    /// Everyone's blockstun
    pub blockstun: u32,
    /// Everyone's knockback
    pub knockback: i32,
}

#[derive(Debug, Default)]
pub struct Flags {
    pub jump: JumpFlags,
}

#[derive(Debug, Default, PartialEq)]
pub enum JumpFlags {
    #[default]
    None,
    Neutral,
    Forward,
    Backward,
}

#[derive(Debug)]
pub struct Locks {
    pub dash_f: bool,
    pub dash_b: bool,
}

impl Default for Locks {
    fn default() -> Self {
        Self {
            dash_f: true,
            dash_b: true,
        }
    }
}

impl Locks {
    pub fn dash_lockout(&mut self, buffer: &Buffer, time: u8) {
        let forward = if buffer.current().facing_left() {
            buffer.held.l
        } else {
            buffer.held.r
        };
        let backward = if buffer.current().facing_left() {
            buffer.held.r
        } else {
            buffer.held.l
        };
        let down = buffer.held.d;
        let up = buffer.held.u;
        let n = buffer.held.n;

        if forward > time && down == 0 && up == 0 {
            self.dash_f = false;
        } else if backward > time || down > time || up > time || n > time {
            self.dash_f = true;
        }
        if backward > time && down == 0 && up == 0 {
            self.dash_b = false;
        } else if forward > time || down > time || up > time || n > time {
            self.dash_b = true;
        }

        // Unlock on forced input
        if buffer.dashed(Dashes::ForcedForward, buffer.dash + 5) {
            self.dash_f = true;
        }
        if buffer.dashed(Dashes::ForcedBackward, buffer.dash + 5) {
            self.dash_b = true;
        }
    }
}

pub fn handle_modifiers(world: &mut World) {
    let query = world
        .query_named::<(&mut StateMachine, &mut Buffer, &mut Physics)>("Handle state modifiers")
        .set_cached()
        .build();
    query.each(|(state, buffer, physics)| {
        if let Some(command) = &state.modifiers.commands {
            if let Some(positions) = &command.positions {
                if let Some(position) = positions.get(state.modifiers.index) {
                    if position.on_frame == state.ctx.elapsed {
                        physics.set_forward_position(position.value.x);
                        physics.position.y = position.value.y;
                        state.modifiers.index += 1;
                    }
                }
            }
            if let Some(cancels) = &command.cancels {
                for action in cancels {
                    if state.ctx.elapsed >= action.after_frame
                        && state.ctx.elapsed <= action.until_frame.unwrap_or(u32::MAX)
                    {
                        for transition in &action.states {
                            match &action.on {
                                Some(collisions) => {
                                    for kind in collisions {
                                        match kind {
                                            CollisionType::Whiff => {
                                                if transition.set(&mut state.ctx, buffer, physics) {
                                                    turn_transition(
                                                        &mut state.ctx,
                                                        buffer,
                                                        physics,
                                                    );
                                                    return;
                                                }
                                            }
                                            CollisionType::Hit => todo!(),
                                            CollisionType::Block => todo!(),
                                            CollisionType::Parry => todo!(),
                                        }
                                    }
                                }
                                None => {
                                    if transition.set(&mut state.ctx, buffer, physics) {
                                        println!("{} -> {:?}", state.ctx.player, transition);
                                        turn_transition(&mut state.ctx, buffer, physics);
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });
}
