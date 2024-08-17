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
    let facing_q = world
        .query_named::<(&mut Physics, &mut InputBuffer, &Player)>("Update facing direction")
        .set_cached()
        .build();
    facing_q.run(|mut it| {
        while it.next() {
            let mut physics = it.field::<Physics>(0).unwrap();
            let mut buffer = it.field::<InputBuffer>(1).unwrap();
            let (p1, p2) = physics.split_at_mut(1);
            if let Some(p1) = p1.get_mut(0) {
                if let Some(p2) = p2.get_mut(0) {
                    p1.facing_opponent = ((p2.position.x <= p1.position.x) && p1.facing_left)
                        || ((p2.position.x >= p1.position.x) && !p1.facing_left);

                    p2.facing_opponent = ((p1.position.x <= p2.position.x) && p2.facing_left)
                        || ((p1.position.x >= p2.position.x) && !p2.facing_left);
                }
            }

            for i in 0..buffer.len() {
                if physics[i].facing_left {
                    *buffer[i].current_mut() |= Buttons::FacingLeft;
                }
                if physics[i].facing_opponent {
                    *buffer[i].current_mut() |= Buttons::FacingOpponent;
                }
            }
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
