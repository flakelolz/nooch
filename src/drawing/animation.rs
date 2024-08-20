use crate::prelude::*;

#[derive(Component)]
pub struct AnimationData {
    /// Actor's name
    name: Name,
    /// Keyframe data
    data: std::collections::HashMap<String, Vec<Keyframe>>,
}

impl AnimationData {
    pub fn new(name: Name) -> Self {
        Self {
            name,
            data: load_animation_data(name.into()),
        }
    }

    /// Get the actor's name
    pub fn name(&self) -> Name {
        self.name
    }

    /// Get the keyframes for an animation by name
    pub fn get(&self, name: &str) -> Option<&Vec<Keyframe>> {
        self.data.get(name)
    }
}

#[derive(Component)]
pub struct Animator {
    pub current: String,
    /// Character's origin
    pub origin: Vec2,
    /// Keyframe index
    pub index: usize,
    /// Internal timer for each keyframe
    pub tick: u32,
    /// Current keyframe duration
    pub duration: u32,
    /// Width scale of entity being drawn
    pub w_scale: f32,
    /// Height scale of entity being drawn
    pub h_scale: f32,
    /// Z index based on layer
    pub layer: u8,
}

impl Animator {
    pub fn new(current: String, layer: u8, origin: Vec2) -> Self {
        Self {
            current,
            origin,
            index: 0,
            tick: 0,
            duration: 0,
            w_scale: 1.,
            h_scale: 1.,
            layer,
        }
    }

    pub fn reset(&mut self) {
        self.duration = 0;
        self.index = 0;
        self.tick = 0;
    }
}

pub struct Keyframe {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub duration: u32,
}

struct Draw {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    flip: bool,
    w_scale: f32,
    h_scale: f32,
    origin: Vec2,
    layer: u8,
    pos: IVec2,
    name: Name,
}

pub fn actor_animation(d: &mut impl RaylibDraw, world: &World) {
    let mut buffer: Vec<Draw> = Vec::new();

    let query = world
        .query_named::<(&mut Animator, &Physics, &AnimationData)>("Animate Player")
        .set_cached()
        .build();

    query.each(|(animator, physics, data)| {
        // Find keyframe
        let Some(keyframes) = data.get(&animator.current) else {
            return;
        };
        let frame = &keyframes[animator.index];

        // Construct Draw command
        let pos_x = physics.position.x;
        let pos_y = -physics.position.y;
        let draw = Draw {
            x: frame.x,
            y: frame.y,
            w: frame.w,
            h: frame.h,
            flip: physics.facing_left,
            w_scale: animator.w_scale,
            h_scale: animator.h_scale,
            origin: animator.origin,
            layer: animator.layer,
            pos: IVec2::new(pos_x, pos_y),
            name: data.name(),
        };

        // Update animator
        animator.duration = frame.duration;

        animator.tick += 1;

        if animator.tick >= animator.duration {
            animator.tick = 0;
            animator.index += 1;

            if animator.index >= keyframes.len() {
                animator.index = 0;
            }
        }

        // Add to buffer
        buffer.push(draw);
    });

    draw_actor(d, buffer, world);
}

fn draw_actor(d: &mut impl RaylibDraw, mut commands: Vec<Draw>, world: &World) {
    world.get::<&Assets>(|assets| {
        let Some(first) = commands.first() else {
            return;
        };
        let name = first.name;
        let Some(texture) = assets.get(name) else {
            return;
        };

        commands.sort_by(|a, b| a.layer.cmp(&b.layer));
        for command in &commands {
            let (screen_x, screen_y) = pos_to_screen(command.pos);
            let (width, height) = (command.w, command.h);

            let source_rec = rrect(
                command.x,
                command.y,
                {
                    if command.flip {
                        -width * command.w_scale
                    } else {
                        width * command.w_scale
                    }
                },
                height * command.h_scale,
            );
            let dest_rec = rrect(screen_x, screen_y, width, height);
            let origin = rvec2(width * command.origin.x, height * command.origin.y);
            let rotation = 0.;
            let tint = Color::WHITE;

            d.draw_texture_pro(texture, source_rec, dest_rec, origin, rotation, tint);
        }
    });
}
