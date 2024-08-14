use crate::prelude::*;

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct Physics {
    pub position: IVec2,
    pub velocity: IVec2,
    pub acceleration: IVec2,
    pub facing_left: bool,
}

impl Physics {
    pub fn new((x, y): (i32, i32), flipped: bool) -> Self {
        Self {
            position: IVec2::new(x, y),
            velocity: IVec2::ZERO,
            acceleration: IVec2::ZERO,
            facing_left: flipped,
        }
    }

    pub fn set_forward_position(&mut self, pos: i32) {
        self.position.x += if self.facing_left { -pos } else { pos };
    }

    pub fn set_forward_velocity(&mut self, speed: i32) {
        self.velocity.x = if self.facing_left { -speed } else { speed };
    }
}

pub fn update_physics(world: &mut World) {
    let query = world
        .query_named::<&mut Physics>("Update physics")
        .set_cached()
        .build();
    query.each(|physics| {
        physics.position += physics.velocity;
        physics.velocity += physics.acceleration;
    });
}
