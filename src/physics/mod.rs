use crate::prelude::*;

const DECELERATION: i32 = 1000;
const THRESHOLD: i32 = 1100;

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct Physics {
    pub position: IVec2,
    pub velocity: IVec2,
    pub acceleration: IVec2,
    pub facing_left: bool,
    pub facing_opponent: bool,
    pub airborne: bool,
    /// Pushbox left
    pub left: i32,
    /// Pushbox right
    pub right: i32,
}

impl Physics {
    pub fn new((x, y): (i32, i32), left: bool) -> Self {
        Self {
            position: IVec2::new(x, y),
            velocity: IVec2::ZERO,
            acceleration: IVec2::ZERO,
            facing_left: left,
            facing_opponent: true,
            ..Default::default()
        }
    }

    pub fn set_forward_position(&mut self, pos: i32) {
        if self.facing_left {
            self.try_add_x_position(-pos);
        } else {
            self.try_add_x_position(pos);
        }
    }

    pub fn set_forward_velocity(&mut self, speed: i32) {
        self.velocity.x = if self.facing_left { -speed } else { speed };
    }

    pub fn set_backward_velocity(&mut self, speed: i32) {
        self.velocity.x = if self.facing_left { speed } else { -speed };
    }

    /// Tries to add x to the position. Returns true if it was successful.
    pub fn try_add_x_position(&mut self, add: i32) -> bool {
        if add == 0 {
            return true;
        }
        let amount = self.position.x + add;
        let middle = (RIGHT_WALL - LEFT_WALL) / 2;
        let left = self.position.x - self.left;
        let right = -(self.position.x - self.right);

        if self.position.x <= middle {
            // Add position as long as the addition is within the left wall
            if amount - left > LEFT_WALL {
                self.position.x = amount;
                true
            } else {
                self.position.x = LEFT_WALL + left;
                false
            }
        // Add position as long as the addition is within the right wall
        } else if amount + right < RIGHT_WALL {
            self.position.x = amount;
            true
        } else {
            self.position.x = RIGHT_WALL - right;
            false
        }
    }

    pub fn can_add_x_position(&mut self, add: i32) -> bool {
        let amount = self.position.x + add;
        let middle = (RIGHT_WALL - LEFT_WALL) / 2;
        let left = self.position.x - self.left;
        let right = -(self.position.x - self.right);

        if self.position.x <= middle {
            // Add position as long as the addition is within the left wall
            amount - left > LEFT_WALL
        // Add position as long as the addition is within the right wall
        } else {
            amount + right < RIGHT_WALL
        }
    }
}

pub fn update_physics(world: &mut World) {
    let facing_q = world
        .query_named::<(&mut Physics, &mut Buffer, &Player)>("Update facing direction")
        .set_cached()
        .build();
    facing_q.run(|mut it| {
        while it.next() {
            let mut physics = it.field::<Physics>(0).unwrap();
            let mut buffer = it.field::<Buffer>(1).unwrap();
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
        .query_named::<&mut Physics>("Update position with velocity")
        .set_cached()
        .build();
    query.each(|physics| {
        // Checks if it's within the bounds of the walls
        physics.try_add_x_position(physics.velocity.x);
        physics.position.y += physics.velocity.y;
        physics.velocity += physics.acceleration;
    });

    let facing_q = world
        .query_named::<(&mut Physics, &mut StateMachine, &Player)>("Apply knockback")
        .set_cached()
        .build();
    facing_q.run(|mut it| {
        while it.next() {
            let mut physics = it.field::<Physics>(0).unwrap();
            let mut state = it.field::<StateMachine>(1).unwrap();

            for i in 0..state.len() {
                let reaction = &mut state[i].ctx.reaction;

                if reaction.hitstop == 0 && reaction.knockback != 0 {
                    if !physics[i].try_add_x_position(reaction.knockback) {
                        let j = if i == 0 { 1 } else { 0 };
                        physics[j].try_add_x_position(-reaction.knockback);
                    }

                    // Decelerate
                    if reaction.knockback > 0 {
                        reaction.knockback -= DECELERATION;
                    }

                    if reaction.knockback < 0 {
                        reaction.knockback += DECELERATION;
                    }

                    if reaction.knockback.abs() < THRESHOLD {
                        reaction.knockback = 0;
                    }
                }
            }
        }
    });
}

/// Conditionally flip the character to face the opponent if not already facing them.
// NOTE: This change can and will happen in the middle of a state, before the physics update
pub fn face_opponent(physics: &mut Physics, buffer: &mut Buffer) -> bool {
    if !physics.facing_opponent {
        physics.facing_left = !physics.facing_left;
        *buffer.current_mut() ^= Buttons::FacingLeft;
        physics.facing_opponent = true;
        *buffer.current_mut() |= Buttons::FacingOpponent;
        return true;
    }
    false
}
