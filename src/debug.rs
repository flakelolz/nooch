use crate::{physics, prelude::*};

const TEXT_SIZE: f32 = 10.0;
// const SCREEN_CENTER: i32 = WIDTH / 2;

#[derive(Component)]
pub struct DebugUI {
    pub editor: bool,
    pub info: bool,
    pub position: bool,
    pub state: bool,
    pub inputs: bool,
    pub hitboxes: bool,
    pub hurtboxes: bool,
    pub pushboxes: bool,
    pub proximity: bool,
    pub buffer: bool,
    pub all: bool,
}

impl Default for DebugUI {
    fn default() -> Self {
        Self {
            info: true,
            editor: false,
            position: true,
            state: true,
            inputs: true,
            hitboxes: true,
            hurtboxes: true,
            pushboxes: true,
            proximity: false,
            buffer: true,
            all: true,
        }
    }
}

impl DebugUI {
    pub fn toggle(&mut self, rl: &mut RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_F1) {
            self.editor = !self.editor;
            if self.info {
                self.info = false;
            }
            println!("Editor: {}", self.editor);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_F2) {
            self.hitboxes = !self.hitboxes;
            println!("Hitboxes: {}", self.hitboxes);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_F3) {
            self.hurtboxes = !self.hurtboxes;
            println!("Hurtboxes: {}", self.hurtboxes);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_F4) {
            self.pushboxes = !self.pushboxes;
            println!("Pushboxes: {}", self.pushboxes);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_F5) {
            self.proximity = !self.proximity;
            println!("Proximity: {}", self.proximity);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_F6) {
            self.state = !self.state;
            println!("State: {}", self.state);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_F7) {
            self.position = !self.position;
            println!("Position: {}", self.position);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_F8) {
            self.info = !self.info;
            println!("Information: {}", self.info);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_F9) {
            self.inputs = !self.inputs;
            println!("Inputs: {}", self.inputs);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_F11) {
            if self.all {
                self.all_false();
                println!("All debug options off");
            } else {
                self.all_true();
                println!("All debug options on");
            }
        }
    }

    fn all_false(&mut self) {
        *self = Self {
            info: false,
            position: false,
            state: false,
            inputs: false,
            hitboxes: false,
            hurtboxes: false,
            pushboxes: false,
            proximity: false,
            editor: false,
            buffer: false,
            all: false,
        };
    }

    fn all_true(&mut self) {
        *self = Self {
            info: true,
            position: true,
            state: true,
            inputs: true,
            hitboxes: true,
            hurtboxes: true,
            pushboxes: true,
            proximity: true,
            editor: true,
            buffer: true,
            all: true,
        };
    }
}

pub fn debug(world: &World, ui: &mut &mut imgui::Ui, d: &mut RaylibDrawHandle) {
    let query = world.query::<&mut DebugUI>().singleton().build();
    let size_x = 130.;
    let size_y = 182.;
    let screen_x = d.get_screen_width();
    let screen_y = d.get_screen_height();

    query.each(|debug| {
        if d.is_key_pressed(KeyboardKey::KEY_F1) {
            debug.info = !debug.info;
        }

        ui.window("Debug")
            .collapsed(debug.info, imgui::Condition::Always)
            .size([size_x, size_y], imgui::Condition::Appearing)
            .position([1., 1.], imgui::Condition::FirstUseEver)
            .movable(false)
            .bg_alpha(0.5)
            .build(|| {
                world.lookup("Player 1").get::<&InputBuffer>(|buffer| {
                    ui.text(format!("{:012b}", buffer.current()));
                });
                ui.separator();
                ui.checkbox("Position", &mut debug.position);
                ui.checkbox("State", &mut debug.state);
                ui.checkbox("Pushbox", &mut debug.pushboxes);
                ui.checkbox("Buffer", &mut debug.buffer);
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                let [x, y] = mouse_pos;
                ui.text(format!("{:.1},{:.1}", x, y));
                let mouse = d.get_mouse_position();
                let (x, y) = screen_to_ui(mouse.x, mouse.y, screen_x, screen_y);
                ui.text(format!("{:.1},{:.1}", x, y));

                if debug.buffer {
                    ui.window("Buffer")
                        .position([3., screen_y as f32 - 60.], imgui::Condition::FirstUseEver)
                        .no_decoration()
                        .always_auto_resize(true)
                        .build(|| {
                            world.lookup("Player 1").get::<&InputBuffer>(|buffer| {
                                ui.text(format!("{}", buffer.held));
                                ui.text(format!("{}", Wrapper(buffer.buffer)));
                            });
                        });
                }
            });
    });
}

pub fn reset_physics(world: &mut World, rl: &mut RaylibHandle) {
    let query = world.query::<(&mut Physics, &Player)>().build();
    query.each(|(physics, player)| {
        if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
            match player {
                Player::One => {
                    *physics = Physics::new((112 * 1000, 0), false);
                }
                Player::Two => {
                    *physics = Physics::new((304 * 1000, 0), true);
                }
            }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_F5) {
            physics.facing_left = !physics.facing_left;
        }
    });
}

pub fn show_pushboxes(world: &World, d: &mut impl RaylibDraw) {
    let debug_q = world.query::<&DebugUI>().singleton().build();
    let query = world
        .query::<(&ActionData, &Physics, &StateMachine)>()
        .build();
    debug_q.each(|debug| {
        if !debug.pushboxes {
            return;
        }
        query.each(|(data, physics, state)| {
            if let Some(action) = data.get(state.current.name()) {
                let color = if physics.airborne {
                    Color::LIME
                } else {
                    Color::MAGENTA
                };

                if let Some(pushboxes) = &action.pushboxes {
                    for pushbox in pushboxes.iter() {
                        if pushbox.is_active(state.ctx.elapsed) {
                            let p = pushbox.translated(physics.position, physics.facing_left);
                            let left = world_to_sprite_to_ui_num(p.value.left);
                            let top = -world_to_sprite_to_ui_num(p.value.top) + GROUND_OFFSET;
                            let width = world_to_sprite_to_ui_num(p.value.right - p.value.left);
                            let height = world_to_sprite_to_ui_num(p.value.top - p.value.bottom);
                            d.draw_rectangle_lines(left, top, width, height, color);
                        }
                    }
                } else {
                    // Default pushbox
                    let offset = physics.position;
                    let translated = if physics.facing_left {
                        state.ctx.data.pushbox.translate_flipped(offset)
                    } else {
                        state.ctx.data.pushbox.translate(offset)
                    };
                    let left = world_to_sprite_to_ui_num(translated.left);
                    let top = -world_to_sprite_to_ui_num(translated.top) + GROUND_OFFSET;
                    let width = world_to_sprite_to_ui_num(translated.right - translated.left);
                    let height = world_to_sprite_to_ui_num(translated.top - translated.bottom);
                    d.draw_rectangle_lines(left, top, width, height, color);
                }
            }
        })
    });
}

pub fn show_fps(d: &mut impl RaylibDraw) {
    d.draw_fps(WIDTH - 30, 5);
}

pub fn show_position(world: &World, d: &mut impl RaylibDraw) {
    let font = d.gui_get_font();
    let debug_q = world.query::<&DebugUI>().singleton().build();
    let query = world
        .query_named::<&Physics>("Show position")
        .set_cached()
        .build();
    debug_q.each(|debug| {
        if !debug.position {
            return;
        }
        query.each(|physics| {
            let (screen_x, screen_y) = pos_to_screen(physics.position.neg_y());
            let (screen_x, screen_y) = sprite_to_ui(screen_x, screen_y);
            let (pos_x, pos_y) = world_to_screen(physics.position);
            d.draw_circle(screen_x, screen_y, 1., Color::WHITE);
            d.draw_text_ex(
                &font,
                &format!("{}:{}", pos_x, pos_y),
                rvec2(screen_x, screen_y + 2),
                TEXT_SIZE,
                0.,
                Color::WHITE,
            );
        });
    })
}

pub fn show_state(world: &World, d: &mut impl RaylibDraw) {
    let font = d.gui_get_font();
    let debug_q = world.query::<&DebugUI>().singleton().build();
    let query = world.query::<(&StateMachine, &Physics)>().build();
    debug_q.each(|debug| {
        if !debug.state {
            return;
        }
        query.each(|(state, physics)| {
            let (screen_x, screen_y) = pos_to_screen(physics.position.neg_y());
            let (screen_x, screen_y) = sprite_to_ui(screen_x, screen_y);
            let current = state.current.as_ref();
            let elapsed = state.ctx.elapsed;
            let duration = state.ctx.total;
            let top = 200;
            let offset = 10;

            // State name
            d.draw_text_ex(
                &font,
                current.name(),
                rvec2(screen_x - 30, screen_y - top),
                TEXT_SIZE,
                0.,
                Color::WHITE,
            );
            // Total state duration
            d.draw_text_ex(
                &font,
                &format!("{}", duration),
                rvec2(screen_x - 30, screen_y - top + offset),
                TEXT_SIZE,
                0.,
                Color::WHITE,
            );
            // Frames elapsed
            d.draw_text_ex(
                &font,
                &format!("{}", elapsed),
                rvec2(screen_x - 10, screen_y - top + offset),
                TEXT_SIZE,
                0.,
                Color::WHITE,
            );
        });
    });
}
