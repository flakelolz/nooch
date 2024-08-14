use crate::prelude::*;

pub fn game(rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut rl_imgui = RaylibImguiSupport::setup(rl, thread);
    // Create ECS world
    let mut world = World::new();
    // Optional, gather statistics for explorer
    world.import::<stats::Stats>();
    // Creates REST server on default port (27750)
    world.set(flecs::rest::Rest::default());

    // Setup entities for gameplay
    setup(&mut world, rl, thread);

    while !rl.window_should_close() {
        // Runs the system serving up REST requests
        world.progress();

        update_input(&mut world, rl);

        // Drawing
        // start imgui frame
        let ui = &mut rl_imgui.start_frame(rl);
        let mut d = rl.begin_drawing(thread);
        actor_animation(&mut d, &world);
        d.clear_background(Color::BLACK);

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
        // render imgui frame
        rl_imgui.end_frame(&mut d);
    }
}
