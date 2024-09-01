mod hitboxes;
mod hurtboxes;
mod pushboxes;

use hitboxes::hitboxes;
use hurtboxes::hurtboxes;
use pushboxes::pushboxes;

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
                        synchronize_data(&data_q);
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
                                    hurtboxes(ui, current, editor);
                                    hitboxes(ui, current, editor);
                                    pushboxes(ui, current, editor, state);

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

/// Synchronize data for both players if they're the same character
fn synchronize_data(query: &Query<(&mut StateMachine, &mut ActionData, &Player)>) {
    query.run(|mut it| {
        while it.next() {
            let mut state = it.field::<StateMachine>(0).unwrap();
            let (p1, p2) = state.split_at_mut(1);
            if let Some(state1) = p1.get_mut(0) {
                if let Some(state2) = p2.get_mut(0) {
                    if state1.ctx.name == state2.ctx.name {
                        let mut data = it.field::<ActionData>(1).unwrap();
                        let (d1, d2) = data.split_at_mut(1);
                        if let Some(data1) = d1.get_mut(0) {
                            if let Some(data2) = d2.get_mut(0) {
                                data2.clone_from(data1);
                            }
                        }
                    }
                }
            }
        }
    });
}

impl HitType {
    pub fn as_str(&self) -> &str {
        match self {
            HitType::Ground => "Ground",
            HitType::Air => "Air",
            HitType::Throw => "Throw",
            HitType::Projectile => "Projectile",
        }
    }
}

impl Strength {
    pub fn as_str(&self) -> &str {
        match self {
            Strength::Weak => "Weak",
            Strength::Mid => "Mid",
            Strength::Strong => "Strong",
            Strength::Rising => "Rising",
            Strength::FrontSpin => "FrontSpin",
            Strength::BackSpin => "BackSpin",
        }
    }
}
