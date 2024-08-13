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
    setup(&mut world);

    while !rl.window_should_close() {
        // Runs the system serving up REST requests
        world.progress();

        // Drawing
        // start imgui frame
        let ui = &mut rl_imgui.start_frame(rl);
        let mut d = rl.begin_drawing(thread);
        d.clear_background(Color::BLACK);

        ui.window("Debug")
            .size([220., 50.], imgui::Condition::FirstUseEver)
            .position([0., 0.], imgui::Condition::FirstUseEver)
            .build(|| {
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
