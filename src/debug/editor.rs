use std::io::Write;

use serde_json::to_string_pretty;

use crate::prelude::*;

#[derive(Component, Default)]
pub struct EditorData {
    /// Action index
    pub index: usize,
    /// List of action names
    pub names: Vec<String>,
    pub push_index: usize,
    pub hurt_index: usize,
    pub hit_index: usize,
    pub default_pushbox: Option<Boxes>,
    pub actions: Option<IndexMap<String, Action>>,
}

impl EditorData {
    pub fn new(names: Vec<String>) -> Self {
        Self {
            index: 0,
            names,
            push_index: 0,
            hurt_index: 0,
            hit_index: 0,
            default_pushbox: None,
            actions: None,
        }
    }
}

pub fn editor(world: &mut World, ui: &mut &mut imgui::Ui, d: &mut RaylibDrawHandle) {
    let size_x = 300.;
    let size_y = 500.;
    let debug_q = world.query::<&mut DebugUI>().singleton().build();
    let editor_q = world.query::<&mut EditorData>().build();
    let data_q = world
        .query::<(&mut StateMachine, &mut ActionData, &Player)>()
        .build();

    debug_q.each(|debug| {
        if d.is_key_pressed(KeyboardKey::KEY_F1) {
            debug.editor = !debug.editor;
        }
        if d.is_key_pressed(KeyboardKey::KEY_F11) {
            debug.demo = !debug.demo;
        }
        if debug.editor {
            editor_q.each(|editor| {
                ui.window("Editor")
                    .size([size_x, size_y], imgui::Condition::FirstUseEver)
                    .position([1., 1.], imgui::Condition::FirstUseEver)
                    .bg_alpha(0.5)
                    .build(|| {
                        data_q.each(|(state, actions, player)| {
                            if let Player::One = player {
                                if editor.actions.is_none() {
                                    editor.actions.replace(actions.clone());
                                }
                                let current_state =
                                    actions.get_mut(editor.names[editor.index].as_str());
                                let Some(current) = current_state else {
                                    return;
                                };
                                let combo =
                                    ui.begin_combo(" ", editor.names[editor.index].as_str());
                                if let Some(token) = combo {
                                    for (i, name) in editor.names.iter().enumerate() {
                                        if ui
                                            .selectable_config(name.as_str())
                                            .selected(editor.index == i)
                                            .build()
                                        {
                                            editor.index = i;
                                        }
                                    }
                                    token.end();
                                }
                                ui.input_scalar("Total", &mut current.total).step(1).build();
                                ui.checkbox("Looping", &mut current.looping);

                                let tab = ui.tab_bar("boxes");
                                if let Some(tab_token) = tab {
                                    let token = ui.tab_item("Hurtboxes");
                                    if let Some(token) = token {
                                        if let Some(hurtboxes) = &mut current.hurtboxes {
                                            ui.slider_config("Index", 0, hurtboxes.len() - 1)
                                                .build(&mut editor.hurt_index);

                                            let hurtbox = &mut hurtboxes[editor.hurt_index];
                                            ui.input_scalar(
                                                "Start frame",
                                                &mut hurtbox.start_frame,
                                            )
                                            .step(1)
                                            .build();

                                            ui.input_scalar("Duration", &mut hurtbox.duration)
                                                .step(1)
                                                .build();

                                            let mut bounds = [
                                                hurtbox.value.left,
                                                hurtbox.value.right,
                                                hurtbox.value.top,
                                                hurtbox.value.bottom,
                                            ];
                                            imgui::Drag::new("L  R  T  B")
                                                .speed(1000.)
                                                .build_array(ui, &mut bounds);
                                            hurtbox.value.left = bounds[0];
                                            hurtbox.value.right = bounds[1];
                                            hurtbox.value.top = bounds[2];
                                            hurtbox.value.bottom = bounds[3];

                                            if ui.button("Reset") {
                                                if let Some(action) = &editor.actions {
                                                    let old = action
                                                        .get(editor.names[editor.index].as_str());
                                                    if let Some(old) = old {
                                                        if let Some(hurtboxes) = &old.hurtboxes {
                                                            hurtbox.value =
                                                                hurtboxes[editor.hurt_index].value;
                                                        }
                                                    }
                                                }
                                            }

                                            if ui.button("Add") {
                                                hurtboxes.push(Hurtbox::default());
                                            }
                                            if ui.button("Remove") {
                                                if hurtboxes.len() > 1 {
                                                    hurtboxes.remove(editor.hurt_index);
                                                    if editor.hurt_index >= hurtboxes.len() {
                                                        editor.hurt_index = hurtboxes.len() - 1
                                                    }
                                                } else {
                                                    current.hurtboxes = None;
                                                }
                                            }
                                        } else if ui.button("Create new") {
                                            current.hurtboxes = Some(vec![Hurtbox::default()]);
                                        }

                                        token.end();
                                    }
                                    let token = ui.tab_item("Hitbox");
                                    if let Some(token) = token {
                                        if let Some(hitboxes) = &mut current.hitboxes {
                                            ui.slider_config("Index", 0, hitboxes.len() - 1)
                                                .build(&mut editor.hit_index);

                                            let hitbox = &mut hitboxes[editor.hit_index];
                                            ui.input_scalar("Start frame", &mut hitbox.start_frame)
                                                .step(1)
                                                .build();

                                            ui.input_scalar("Duration", &mut hitbox.duration)
                                                .step(1)
                                                .build();
                                            imgui::Drag::new("Knockback")
                                                .speed(100.)
                                                .build(ui, &mut hitbox.properties.knockback);

                                            let mut bounds = [
                                                hitbox.value.left,
                                                hitbox.value.right,
                                                hitbox.value.top,
                                                hitbox.value.bottom,
                                            ];
                                            imgui::Drag::new("L  R  T  B")
                                                .speed(1000.)
                                                .build_array(ui, &mut bounds);
                                            hitbox.value.left = bounds[0];
                                            hitbox.value.right = bounds[1];
                                            hitbox.value.top = bounds[2];
                                            hitbox.value.bottom = bounds[3];

                                            if ui.button_with_size("Reset", [100., 20.]) {
                                                if let Some(action) = &editor.actions {
                                                    let old = action
                                                        .get(editor.names[editor.index].as_str());
                                                    if let Some(old) = old {
                                                        if let Some(hitboxes) = &old.hitboxes {
                                                            hitbox.value =
                                                                hitboxes[editor.hit_index].value;
                                                        }
                                                    }
                                                }
                                            }

                                            if ui.button("Add") {
                                                hitboxes.push(Hitbox::default());
                                            }
                                            if ui.button("Remove") {
                                                if hitboxes.len() > 1 {
                                                    hitboxes.remove(editor.hit_index);
                                                    if editor.hit_index >= hitboxes.len() {
                                                        editor.hit_index = hitboxes.len() - 1
                                                    }
                                                } else {
                                                    current.hitboxes = None;
                                                }
                                            }
                                        } else if ui.button("Create new") {
                                            current.hitboxes = Some(vec![Hitbox::default()]);
                                        }
                                        token.end();
                                    }
                                    let token = ui.tab_item("Pushbox");
                                    if let Some(token) = token {
                                        if let Some(pushboxes) = &mut current.pushboxes {
                                            ui.slider_config("Index", 0, pushboxes.len() - 1)
                                                .build(&mut editor.push_index);
                                            let pushbox = &mut pushboxes[editor.push_index];
                                            ui.input_scalar(
                                                "Start frame",
                                                &mut pushbox.start_frame,
                                            )
                                            .step(1)
                                            .build();

                                            ui.input_scalar("Duration", &mut pushbox.duration)
                                                .step(1)
                                                .build();


                                            let mut bounds = [
                                                pushbox.value.left,
                                                pushbox.value.right,
                                                pushbox.value.top,
                                                pushbox.value.bottom,
                                            ];
                                            imgui::Drag::new("L  R  T  B")
                                                .speed(1000.)
                                                .build_array(ui, &mut bounds);
                                            pushbox.value.left = bounds[0];
                                            pushbox.value.right = bounds[1];
                                            pushbox.value.top = bounds[2];
                                            pushbox.value.bottom = bounds[3];

                                            if ui.button_with_size("Reset", [100., 20.]) {
                                                if let Some(action) = &editor.actions {
                                                    let old = action
                                                        .get(editor.names[editor.index].as_str());
                                                    if let Some(old) = old {
                                                        if let Some(pushboxes) = &old.pushboxes {
                                                            pushbox.value =
                                                                pushboxes[editor.push_index].value;
                                                        }
                                                    }
                                                }
                                            }

                                            if ui.button("Add") {
                                                pushboxes.push(Pushbox::default());
                                            }
                                            if ui.button("Remove") {
                                                if pushboxes.len() > 1 {
                                                    pushboxes.remove(editor.push_index);
                                                    if editor.push_index >= pushboxes.len() {
                                                        editor.push_index = pushboxes.len() - 1
                                                    }
                                                } else {
                                                    current.pushboxes = None;
                                                }
                                            }
                                        } else {
                                            if editor.default_pushbox.is_none() {
                                                editor.default_pushbox =
                                                    Some(state.ctx.data.pushbox);
                                            }

                                            let pushbox = &mut state.ctx.data.pushbox;
                                            let mut bounds = [
                                                pushbox.left,
                                                pushbox.right,
                                                pushbox.top,
                                                pushbox.bottom,
                                            ];
                                            imgui::Drag::new("L  R  T  B")
                                                .speed(1000.)
                                                .build_array(ui, &mut bounds);
                                            pushbox.left = bounds[0];
                                            pushbox.right = bounds[1];
                                            pushbox.top = bounds[2];
                                            pushbox.bottom = bounds[3];

                                            if ui.button_with_size("Reset", [100., 20.]) {
                                                state.ctx.data.pushbox =
                                                    editor.default_pushbox.unwrap();
                                            }

                                            if ui.button("Create new") {
                                                current.pushboxes = Some(vec![Pushbox::default()]);
                                            }
                                        }
                                        token.end();
                                    }

                                    tab_token.end();
                                }

                                if ui.button("Save") {
                                    save_action_data(actions, &state.ctx.data);
                                }
                                if debug.demo {
                                    ui.show_demo_window(&mut debug.demo);
                                }
                            }
                        });
                    });
            });
        }
    });
}

fn save_action_data(action: &ActionData, character: &CharacterData) {
    let actor = &character.name;
    let path = format!("assets/data/char/{}/data.json", actor);
    let file = std::fs::File::options()
        .read(true)
        .write(true)
        .open(path.clone())
        .unwrap();
    let mut data: CharacterFile = serde_json::from_reader(&file).unwrap();
    data.actions = action.to_vec();

    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(to_string_pretty(&data).unwrap().as_bytes())
        .unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
struct CharacterFile {
    pub name: String,
    pub max_health: i32,
    pub forward_walk: i32,
    pub backward_walk: i32,
    pub jump_velocity: i32,
    pub jump_deceleration: i32,
    pub jump_forward: i32,
    pub jump_backward: i32,
    pub origin: Vec2,
    pub pushbox: Boxes,
    pub actions: Vec<Action>,
}
