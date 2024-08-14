use crate::prelude::*;

pub fn game(rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut rl_imgui = RaylibImguiSupport::setup(rl, thread);
    // Create ECS world
    let mut world = World::new();
    // Optional, gather statistics for explorer
    world.import::<stats::Stats>();
    // Creates REST server on default port (27750)
    world.set(flecs::rest::Rest::default());

    let (mut target_px, mut target_ui) = create_render_targets(rl, thread);
    // Camera
    let camera = Camera2D {
        target: rvec2(0., 0.),
        offset: rvec2(0., 0.),
        rotation: 0.,
        zoom: 1.,
    };
    // Setup entities for gameplay
    setup(&mut world, rl, thread);
    // Debug pause
    let mut paused = false;

    while !rl.window_should_close() {
        // Runs the system serving up REST requests
        world.progress();

        let mut advance = false;
        if rl.is_key_pressed(KeyboardKey::KEY_ENTER)
            || rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_MIDDLE_RIGHT)
        {
            paused = !paused;
            println!("Paused");
        } else if rl.is_key_pressed(KeyboardKey::KEY_BACKSLASH)
            || rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_MIDDLE_LEFT)
        {
            advance = true;
        }

        update_input(&mut world, rl);

        if !paused || advance {
            update_physics(&mut world);
            update_state(&mut world);
        }

        // Calculate window
        let width = rl.get_screen_width();
        let height = rl.get_screen_height();
        let scale = (width / WIDTH).min(height / HEIGHT) as f32;

        // Mouse scale
        rl.set_mouse_scale(1.0 / scale, 1.0 / scale);
        rl.set_mouse_offset(rvec2(
            -(rl.get_screen_width() as f32 - (FWIDTH * scale)) * 0.5,
            -(rl.get_screen_height() as f32 - (FHEIGHT * scale)) * 0.5,
        ));

        // start imgui frame
        let ui = &mut rl_imgui.start_frame(rl);

        // Drawing
        let mut d = rl.begin_drawing(thread);
        d.clear_background(Color::BLACK);

        {
            // Render to pixel texture target
            let mut d = d.begin_texture_mode(thread, &mut target_px);

            if !paused || advance {
                d.clear_background(Color::BLACK);
                actor_animation(&mut d, &world);
            }
        }
        {
            // Render to UI texture target
            let mut d = d.begin_texture_mode(thread, &mut target_ui);
            // Debug
            if !paused || advance {
                d.clear_background(Color::BLANK);
                // show_frame_count(&world, &mut d, &debug);
                // show_state(&world, &mut d, &debug);
                // show_info(&world, &mut d, &debug);
                // show_inputs(&world, &mut d, &debug);
            }
        }

        ui.window("Debug")
            .size([220., 70.], imgui::Condition::FirstUseEver)
            .position([1., 1.], imgui::Condition::FirstUseEver)
            .build(|| {
                world.lookup("Player 1").get::<&Input>(|input| {
                    ui.text(format!("Input: {input:010b}"));
                });
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
            });
        let mut d = d.begin_mode2D(camera);
        rendering(&mut target_px, &mut target_ui, &mut d);
        // render imgui frame
        rl_imgui.end_frame(&mut d);
    }
}
