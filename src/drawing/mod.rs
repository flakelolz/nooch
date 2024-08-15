mod animation;
mod render;

pub use animation::*;
pub use render::*;

use raylib::prelude::*;

use crate::{HEIGHT, WIDTH};

pub struct Configs {
    pub display: DisplayCongfig,
}

impl Default for Configs {
    fn default() -> Self {
        Self {
            display: DisplayCongfig {
                width: WIDTH,
                height: HEIGHT,
            },
        }
    }
}

pub struct DisplayCongfig {
    pub width: i32,
    pub height: i32,
}

impl DisplayCongfig {
    pub fn set_360(&mut self, rl: &mut RaylibHandle, camera: &mut Camera2D) {
        camera.zoom = 1.;
        self.width = 640;
        self.height = 360;
        rl.set_window_size(self.width, self.height);
        center_window(rl, self.width, self.height);
    }

    pub fn set_720(&mut self, rl: &mut RaylibHandle, camera: &mut Camera2D) {
        camera.zoom = 2.;
        self.width = 1280;
        self.height = 720;
        rl.set_window_size(self.width, self.height);
        center_window(rl, self.width, self.height);
    }
}

pub fn center_window(rl: &mut RaylibHandle, window_width: i32, window_height: i32) {
    let monitor = get_current_monitor();
    let width = get_monitor_width(monitor);
    let height = get_monitor_height(monitor);
    rl.set_window_position(
        (width / 2) - window_width / 2,
        (height / 2) - window_height / 2,
    );
}
