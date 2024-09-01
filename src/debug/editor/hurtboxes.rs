use crate::prelude::*;
pub fn hurtboxes(ui: &imgui::Ui, current: &mut Action, editor: &mut EditorData) {
    let token = ui.tab_item("Hurtboxes");
    if let Some(token) = token {
        if let Some(hurtboxes) = &mut current.hurtboxes {
            ui.slider_config("Index", 0, hurtboxes.len() - 1)
                .build(&mut editor.hurt_index);

            let hurtbox = &mut hurtboxes[editor.hurt_index];
            ui.input_scalar("Start frame", &mut hurtbox.start_frame)
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
                    let old = action.get(editor.names[editor.index].as_str());
                    if let Some(old) = old {
                        if let Some(hurtboxes) = &old.hurtboxes {
                            hurtbox.value = hurtboxes[editor.hurt_index].value;
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
}
