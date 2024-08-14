use crate::prelude::*;

pub fn world_to_screen_num(coord: i32) -> i32 {
    coord / 1000
}

pub fn screen_to_world_num(coord: i32) -> i32 {
    coord * 1000
}

pub fn world_to_screen(coord: IVec2) -> (i32, i32) {
    (coord.x / 1000, coord.y / 1000)
}

pub fn screen_to_world(coord: IVec2) -> (i32, i32) {
    (coord.x * 1000, coord.y * 1000)
}

pub fn pos_to_screen(coord: IVec2) -> (i32, i32) {
    (
        world_to_screen_num(coord.x),
        world_to_screen_num(coord.y) + GROUND_OFFSET,
    )
}

pub fn world_to_sprite_to_ui_num(coord: i32) -> i32 {
    let num = world_to_screen_num(coord);
    sprite_to_ui_num(num)
}

pub fn sprite_to_ui_num(x: i32) -> i32 {
    let x = x as f32;
    ((x / WIDTH_3S as f32) * WIDTH as f32) as i32
}

/// Translate from the sprite (416x234) layer to base resolution (1280x720).
pub fn sprite_to_ui(x: i32, y: i32) -> (i32, i32) {
    let old_x = x as f32;
    let old_y = y as f32;
    (
        ((old_x / WIDTH_3S as f32) * WIDTH as f32) as i32,
        ((old_y / HEIGHT_3S as f32) * HEIGHT as f32) as i32,
    )
}
