mod assets;
mod debug;
mod drawing;
mod game;
mod input;
mod physics;
mod state_machine;
mod utils;
mod world;

mod prelude {
    pub use crate::assets::*;
    pub use crate::debug::*;
    pub use crate::drawing::*;
    pub use crate::input::*;
    pub use crate::physics::*;
    pub use crate::state_machine::*;
    pub use crate::utils::*;
    pub use crate::world::*;
    pub use flecs_ecs::prelude::*;
    pub use glam::{IVec2, Vec2};
    pub use nooch::RaylibImguiSupport;
    pub use raylib::prelude::*;
    pub use serde::{Deserialize, Serialize};
    pub use std::collections::HashMap;

    pub use include_dir::{include_dir, Dir};
    pub static ASSETS: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets");

    pub const WIDTH: i32 = 640;
    pub const HEIGHT: i32 = 360;
    pub const FWIDTH: f32 = WIDTH as f32;
    pub const FHEIGHT: f32 = HEIGHT as f32;
    pub const WIDTH_PX: i32 = 416;
    pub const HEIGHT_PX: i32 = 234;
    pub const GROUND_OFFSET: i32 = 200;
}

use prelude::*;

fn main() {
    handle_arguments();
    let (mut rl, thread) = raylib::init().size(WIDTH, HEIGHT).title("Neuch").build();
    rl.set_target_fps(60);

    let font = rl
        .load_font_from_memory(
            &thread,
            ".ttf",
            get_file("fonts/Kenney Mini.ttf").unwrap(),
            512,
            None,
        )
        .expect("Failed to load font");

    rl.gui_set_font(&font);

    game::game(&mut rl, &thread);
}
