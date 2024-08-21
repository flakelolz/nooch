use crate::prelude::*;

#[derive(Component, Clone)]
pub struct Collisions {
    /// For checking if there's a gap between two hitboxes and allow multi-hit attacks
    pub count: usize,
    // pub proximity: Vec<(Entity, ProximityBox)>,
    pub hitboxes: Vec<(Entity, Hitbox)>,
    pub hurtboxes: Vec<(Entity, Hurtbox)>,
    pub pushboxes: Vec<(Entity, Pushbox)>,
}

impl Default for Collisions {
    fn default() -> Self {
        Self {
            count: 0,
            hitboxes: Vec::with_capacity(20),
            pushboxes: Vec::with_capacity(10),
            hurtboxes: Vec::with_capacity(10),
        }
    }
}

#[derive(Component, Default)]
pub struct HitEvents(pub Vec<HitEvent>);

impl std::ops::Deref for HitEvents {
    type Target = Vec<HitEvent>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for HitEvents {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HitEvent {
    pub attacker: Entity,
    pub defender: Entity,
    /// Height of the hurtbox that was hit, to know which animation to transition to
    pub height: Height,
    pub properties: HitProperties,
    // Proximity block boxes
    // pub proximity: Option<(ProximityBox, u32)>,
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

            // Read active boxes
            if let Some(action) = data.get(state.current.name()) {
                if let Some(hurtboxes) = &action.hurtboxes {
                    for hurtbox in hurtboxes.iter() {
                        let hurtbox = hurtbox.translated(offset, physics.facing_left);
                        if hurtbox.is_active(state.ctx.elapsed) {
                            collisions.hurtboxes.push((entity.id(), hurtbox));
                        }
                    }
                }

                if let Some(hitboxes) = &action.hitboxes {
                    for hitbox in hitboxes.iter() {
                        let hitbox = hitbox.translated(offset, physics.facing_left);
                        if hitbox.is_active(state.ctx.elapsed) {
                            collisions.hitboxes.push((entity.id(), hitbox));
                        }
                    }

                    {
                        let first = action.hitboxes.as_ref().unwrap().first().unwrap();
                        let last = action.hitboxes.as_ref().unwrap().last().unwrap();

                        // If there's a gap between hitboxes, it means that the action is multi-hit and needs to be
                        // able to hit again
                        if state.ctx.elapsed >= first.start_frame
                            && state.ctx.elapsed <= last.start_frame
                        // && state.ctx.reaction.hitstop == 0
                        {
                            if let Some(hitbox) = hitboxes.get(collisions.count) {
                                if !hitbox.is_active(state.ctx.elapsed) {
                                    // state.ctx.reaction.has_hit = false;
                                } else {
                                    collisions.count += 1;
                                }
                            }
                        }
                    }
                }
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

        // Register hit collisions
        let events_q = world.query::<&mut HitEvents>().singleton().build();
        for (attacker, hitbox) in collisions.hitboxes.iter() {
            for (defender, hurtbox) in collisions.hurtboxes.iter() {
                if attacker != defender && boxes_overlap(&hitbox.value, &hurtbox.value) {
                    attacker
                        .entity_view(&*world)
                        .get::<(&mut StateMachine)>(|state| {
                            // If the action has any cancels, set the cancel flag
                            // if let Some(instructions) = &state.context.modifiers.instructions {
                            // if instructions.cancels.is_some() {
                            // state.context.ctx.reaction.can_cancel = true;
                            // Extend the cancel window to account for hitstop
                            // buffer.cancels =
                            // buffer.attack + hitbox.properties.hitstop as usize;
                            // }
                            // }
                            // Don't hit with the same attack more than once
                            let has_hit = &mut state.ctx.reaction.has_hit;
                            if *has_hit {
                                return;
                            }
                            *has_hit = true;
                            println!("Attacker: {} Defender: {}", attacker, defender);

                            events_q.each(|hit_events| {
                                hit_events.push(HitEvent {
                                    attacker: *attacker,
                                    defender: *defender,
                                    height: hurtbox.height,
                                    properties: HitProperties {
                                        hit_type: hitbox.properties.hit_type,
                                        strength: hitbox.properties.strength,
                                        hitstop: hitbox.properties.hitstop,
                                        hitstun: hitbox.properties.hitstun,
                                        blockstun: hitbox.properties.blockstun,
                                        knockback: hitbox.properties.knockback,
                                    },
                                    // proximity: None,
                                });
                            });
                        });
                }
            }
        }

        // Register and handle push collisions
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

        collisions.hitboxes.clear();
        collisions.hurtboxes.clear();
        collisions.pushboxes.clear();
    });
}

impl Hitbox {
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

impl Hurtbox {
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
