use crate::prelude::*;

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct Physics {
    pub position: IVec2,
    pub velocity: IVec2,
    pub acceleration: IVec2,
    pub facing_left: bool,
    pub facing_opponent: bool,
    pub airborne: bool,
}

impl Physics {
    pub fn new((x, y): (i32, i32), left: bool) -> Self {
        Self {
            position: IVec2::new(x, y),
            velocity: IVec2::ZERO,
            acceleration: IVec2::ZERO,
            facing_left: left,
            facing_opponent: true,
            airborne: false,
        }
    }

    pub fn set_forward_position(&mut self, pos: i32) {
        self.position.x += if self.facing_left { -pos } else { pos };
    }

    pub fn set_forward_velocity(&mut self, speed: i32) {
        self.velocity.x = if self.facing_left { -speed } else { speed };
    }

    pub fn set_backward_velocity(&mut self, speed: i32) {
        self.velocity.x = if self.facing_left { speed } else { -speed };
    }
}

pub fn update_physics(world: &mut World) {
    let p1 = world.try_lookup("Player 1");
    let p2 = world.try_lookup("Player 2");
    if let Some(p1) = p1 {
        if let Some(p2) = p2 {
            p1.get::<&mut Physics>(|p1| {
                p2.get::<&mut Physics>(|p2| {
                    p1.facing_opponent = ((p2.position.x <= p1.position.x) && p1.facing_left)
                        || ((p2.position.x >= p1.position.x) && !p1.facing_left);

                    p2.facing_opponent = ((p1.position.x <= p2.position.x) && p2.facing_left)
                        || ((p1.position.x >= p2.position.x) && !p2.facing_left);
                })
            })
        }
    }

    // Apply facing direction to the input itself
    let facing_q = world.query::<(&mut InputBuffer, &mut Physics)>().build();
    facing_q.each(|(buffer, physics)| {
        if physics.facing_left {
            *buffer.current_mut() |= Buttons::FacingLeft;
        }
        if physics.facing_opponent {
            *buffer.current_mut() |= Buttons::FacingOpponent;
        }
    });

    let query = world
        .query_named::<&mut Physics>("Update physics")
        .set_cached()
        .build();
    query.each(|physics| {
        physics.position += physics.velocity;
        physics.velocity += physics.acceleration;
    });
}

/// Conditionally flip the character to face the opponent if not already facing them.
// NOTE: This change can and will happen in the middle of a state, before the physics update
pub fn face_opponent(physics: &mut Physics, buffer: &mut InputBuffer) -> bool {
    if !physics.facing_opponent {
        physics.facing_left = !physics.facing_left;
        *buffer.current_mut() ^= Buttons::FacingLeft;
        physics.facing_opponent = true;
        *buffer.current_mut() |= Buttons::FacingOpponent;
        return true;
    }
    false
}
