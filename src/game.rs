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
    let mut camera = Camera2D {
        target: rvec2(0., 0.),
        offset: rvec2(0., 0.),
        rotation: 0.,
        zoom: 1.,
    };
    // Setup entities for gameplay
    setup(&mut world, rl, thread);
    // Debug pause
    let mut paused = false;
    let mut configs = Configs::default();
    configs.display.set_720(rl, &mut camera);

    while !rl.window_should_close() {
        let mut advance = false;
        if rl.is_key_pressed(KeyboardKey::KEY_INSERT)
            || rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_MIDDLE_RIGHT)
        {
            paused = !paused;
            println!("Paused");
        } else if rl.is_key_pressed(KeyboardKey::KEY_DELETE)
            || rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_MIDDLE_LEFT)
        {
            advance = true;
        }

        update_input(&mut world, rl);

        if !paused || advance {
            update_buffer(&mut world);
            update_physics(&mut world);
            update_state(&mut world);
            collisions(&mut world);
            reactions(&mut world);
        }

        reset_physics(&mut world, rl);

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
                // show_info(&world, &mut d, &debug);
                // show_inputs(&world, &mut d, &debug);
            }
        }
        {
            let mut d = d.begin_mode2D(camera);
            rendering(&mut target_px, &mut target_ui, &mut d);
            show_hitboxes(&world, &mut d);
            show_pushboxes(&world, &mut d);
            show_hurtboxes(&world, &mut d);
            show_state(&world, &mut d);
            show_fps(&mut d);
            show_position(&world, &mut d);
        }
        // render imgui frame
        debug(&world, ui, &mut d);
        editor(&mut world, ui, &mut d);
        rl_imgui.end_frame(&mut d);

        // Runs the system serving up REST requests
        world.progress();
    }
}
