mod game;
mod input;
mod physics;
mod player;

mod prelude {
    pub use crate::input::*;
    pub use crate::physics::*;
    pub use crate::player::*;
    pub use flecs_ecs::prelude::*;
    pub use glam::IVec2;
    pub use nooch::RaylibImguiSupport;
    pub use raylib::prelude::*;

    pub use include_dir::{include_dir, Dir};
    pub static ASSETS: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets");

    pub const WIDTH: i32 = 640;
    pub const HEIGHT: i32 = 360;
    pub const FWIDTH: f32 = WIDTH as f32;
    pub const FHEIGHT: f32 = HEIGHT as f32;
    pub const WIDTH_3S: i32 = 416;
    pub const HEIGHT_3S: i32 = 234;
    pub const GROUND_OFFSET: i32 = 200;
}

use prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(WIDTH, HEIGHT).title("Neuch").build();
    rl.set_target_fps(60);

    let font = rl
        .load_font_ex(&thread, "assets/fonts/Kenney Mini.ttf", 512, None)
        .expect("Failed to load font");

    rl.gui_set_font(&font);

    game::game(&mut rl, &thread);
}
