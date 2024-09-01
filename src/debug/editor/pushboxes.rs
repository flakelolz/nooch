use crate::prelude::*;
pub fn pushboxes(ui: &imgui::Ui, current: &mut Action, editor: &mut EditorData, state: &mut StateMachine) {
    let token = ui.tab_item("Pushbox");
    if let Some(token) = token {
        if let Some(pushboxes) = &mut current.pushboxes {
            ui.slider_config("Index", 0, pushboxes.len() - 1)
                .build(&mut editor.push_index);
            let pushbox = &mut pushboxes[editor.push_index];
            ui.input_scalar("Start frame", &mut pushbox.start_frame)
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
                    let old = action.get(editor.names[editor.index].as_str());
                    if let Some(old) = old {
                        if let Some(pushboxes) = &old.pushboxes {
                            pushbox.value = pushboxes[editor.push_index].value;
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
                editor.default_pushbox = Some(state.ctx.data.pushbox);
            }

            let pushbox = &mut state.ctx.data.pushbox;
            let mut bounds = [pushbox.left, pushbox.right, pushbox.top, pushbox.bottom];
            imgui::Drag::new("L  R  T  B")
                .speed(1000.)
                .build_array(ui, &mut bounds);
            pushbox.left = bounds[0];
            pushbox.right = bounds[1];
            pushbox.top = bounds[2];
            pushbox.bottom = bounds[3];

            if ui.button_with_size("Reset", [100., 20.]) {
                state.ctx.data.pushbox = editor.default_pushbox.unwrap();
            }

            if ui.button("Create new") {
                current.pushboxes = Some(vec![Pushbox::default()]);
            }
        }
        token.end();
    }
}
