use crate::prelude::*;
pub fn hitboxes(ui: &imgui::Ui, current: &mut Action, editor: &mut EditorData) {
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

            let hit_combo = ui.begin_combo("Hit type", hitbox.properties.hit_type.as_str());
            if let Some(token) = hit_combo {
                let arr = [HitType::Ground, HitType::Air, HitType::Projectile];
                for kind in &arr {
                    if ui
                        .selectable_config(kind.as_str())
                        .selected(*kind == hitbox.properties.hit_type)
                        .build()
                    {
                        hitbox.properties.hit_type = *kind;
                    }
                }
                token.end();
            }
            let strength_combo = ui.begin_combo("Strength", hitbox.properties.strength.as_str());
            if let Some(token) = strength_combo {
                let arr = [
                    Strength::Weak,
                    Strength::Mid,
                    Strength::Strong,
                    Strength::FrontSpin,
                    Strength::BackSpin,
                ];
                for kind in &arr {
                    if ui
                        .selectable_config(kind.as_str())
                        .selected(*kind == hitbox.properties.strength)
                        .build()
                    {
                        hitbox.properties.strength = *kind;
                    }
                }
                token.end();
            }
            ui.input_scalar("Hitstop", &mut hitbox.properties.hitstop)
                .step(1)
                .build();
            ui.input_scalar("Hitstun", &mut hitbox.properties.hitstun)
                .step(1)
                .build();
            ui.input_scalar("Blockstun", &mut hitbox.properties.blockstun)
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
                    let old = action.get(editor.names[editor.index].as_str());
                    if let Some(old) = old {
                        if let Some(hitboxes) = &old.hitboxes {
                            hitbox.value = hitboxes[editor.hit_index].value;
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
}
