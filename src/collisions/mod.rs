use crate::prelude::*;

#[derive(Component, Clone)]
pub struct Collisions {
    /// For checking if there's a gap between two hitboxes and allow multi-hit attacks
    // pub count: usize,
    // pub proximity: Vec<(Entity, ProximityBox)>,
    // pub hitboxes: Vec<(Entity, Hitbox)>,
    // pub hurtboxes: Vec<(Entity, Hurtbox)>,
    pub pushboxes: Vec<(Entity, Pushbox)>,
}

impl Default for Collisions {
    fn default() -> Self {
        Self {
            pushboxes: Vec::with_capacity(10),
        }
    }
}

pub fn collisions(world: &mut World) {
    let collision_q = world
        .query_named::<&mut Collisions>("Get Collision Manager")
        .set_cached()
        .singleton()
        .build();
    let query = world
        .query_named::<(&mut ActionData, &mut Physics, &mut StateMachine)>("Read Collisions")
        .set_cached()
        .build();

    collision_q.each(|collisions| {
        query.each_entity(|entity, (data, physics, state)| {
            let offset = physics.position;

            if let Some(action) = data.get(state.current.name()) {
                if let Some(pushboxes) = &action.pushboxes {
                    for pushbox in pushboxes.iter() {
                        let pushbox = pushbox.translated(offset, physics.facing_left);
                        if pushbox.is_active(state.ctx.elapsed) {
                            // For calculating corner bounds
                            physics.left = pushbox.value.left;
                            physics.right = pushbox.value.right;
                            collisions.pushboxes.push((entity.id(), pushbox));
                        }
                    }
                } else {
                    let pushbox = if physics.facing_left {
                        state.ctx.data.pushbox.translate_flipped(offset)
                    } else {
                        state.ctx.data.pushbox.translate(offset)
                    };

                    // For calculating corner bounds
                    physics.left = pushbox.left;
                    physics.right = pushbox.right;
                    collisions.pushboxes.push((
                        entity.id(),
                        Pushbox {
                            start_frame: 0,
                            duration: 1,
                            value: pushbox,
                        },
                    ));
                }
            }
        });

        let mut overlap;
        for (attacker, a_pushbox) in collisions.pushboxes.iter() {
            for (defender, b_pushbox) in collisions.pushboxes.iter() {
                if attacker != defender && boxes_overlap(&a_pushbox.value, &b_pushbox.value) {
                    let left = a_pushbox.value.left.max(b_pushbox.value.left);
                    let right = a_pushbox.value.right.min(b_pushbox.value.right);
                    overlap = right - left;
                    let overlap = overlap / 2;

                    attacker
                        .entity_view(&*world)
                        .get::<&mut Physics>(|a_physics| {
                            defender
                                .entity_view(&*world)
                                .get::<&mut Physics>(|b_physics| {
                                    // I don't know why this works, but this produces the behevior I want
                                    if a_physics.facing_left.cmp(&b_physics.facing_left)
                                        == std::cmp::Ordering::Less
                                    {
                                        a_physics.try_add_x_position(-overlap);
                                        b_physics.try_add_x_position(overlap);
                                    }
                                });
                        });
                }
            }
        }

        collisions.pushboxes.clear();
    });
}

impl Pushbox {
    pub fn is_active(&self, frame: u32) -> bool {
        (frame >= self.start_frame) && (frame < (self.start_frame + self.duration))
    }
    pub fn translated(&self, offset: IVec2, flipped: bool) -> Self {
        Self {
            value: if flipped {
                self.value.translate_flipped(offset)
            } else {
                self.value.translate(offset)
            },
            ..*self
        }
    }
}

impl Boxes {
    /// Translate the hitbox by an offset
    pub fn translate(&self, offset: IVec2) -> Self {
        Self {
            left: self.left + offset.x,
            right: self.right + offset.x + 1000,
            top: self.top + offset.y,
            bottom: self.bottom + offset.y,
        }
    }
    /// Translate the hitbox by an offset when facing left
    pub fn translate_flipped(&self, offset: IVec2) -> Self {
        Self {
            left: -self.right + offset.x,
            right: -self.left + offset.x,
            top: self.top + offset.y,
            bottom: self.bottom + offset.y,
        }
    }
}

/// Check if two boxes overlap
fn boxes_overlap(a: &Boxes, b: &Boxes) -> bool {
    !((a.left > b.right) || (b.left > a.right) || (a.bottom > b.top) || (b.bottom > a.top))
}

// fn cornered(pushbox: &Pushbox, physics: &mut Physics) {
//     let left = pushbox.value.left;
//     let right = pushbox.value.right;
//
//     physics.cornered = false;
//     if physics.facing_left {
//         if right >= RIGHT_WALL - 1000 {
//             physics.cornered = true;
//         }
//     } else if left <= 2500 {
//         physics.cornered = true;
//     }
// }
