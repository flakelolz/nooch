use crate::prelude::*;

pub fn reactions(world: &mut World) {
    let query = world
        .query_named::<&mut StateMachine>("Advance reactions")
        .set_cached()
        .build();
    query.each(|state| {
        let reaction = &mut state.ctx.reaction;
        if reaction.hitstop > 0 {
            reaction.hitstop -= 1;
        }
        if reaction.hitstun > 0 && reaction.hitstop == 0 {
            reaction.hitstun -= 1;
        }
        if reaction.blockstun > 0 && reaction.hitstop == 0 {
            reaction.blockstun -= 1;
        }
    });

    let events_q = world.query::<&mut HitEvents>().set_cached().build();
    events_q.each(|events| {
        for event in events.iter() {
            event
                .attacker
                .entity_view(&*world)
                .get::<(&mut Physics, &mut StateMachine)>(|(a_physics, a_state)| {
                    event
                        .defender
                        .entity_view(&*world)
                        .get::<(&mut Physics, &mut StateMachine)>(|(b_physics, b_state)| {
                            let knockback = if a_physics.facing_left {
                                -event.properties.knockback
                            } else {
                                event.properties.knockback
                            };

                            b_state.ctx.reaction.knockback = knockback;
                            b_state.ctx.reaction.hitstop = event.properties.hitstop;
                            b_state.ctx.reaction.hitstun = event.properties.hitstun;
                        });
                });
        }
        events.clear();
    });
}
