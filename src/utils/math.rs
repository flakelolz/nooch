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

/// Translate from the sprite (416x234) resolution to base resolution (640x360).
pub fn sprite_to_ui(x: i32, y: i32) -> (i32, i32) {
    let old_x = x as f32;
    let old_y = y as f32;
    (
        ((old_x / WIDTH_PX as f32) * WIDTH as f32) as i32,
        ((old_y / HEIGHT_PX as f32) * HEIGHT as f32) as i32,
    )
}
/// Translate from the base resolution (640x360) to the sprite (416x234) resolution.
pub fn ui_to_sprite(x: i32, y: i32) -> (i32, i32) {
    let old_x = x as f32;
    let old_y = y as f32;
    (
        ((old_x / WIDTH as f32) * WIDTH_PX as f32) as i32,
        ((old_y / HEIGHT as f32) * HEIGHT_PX as f32) as i32,
    )
}

/// Translate from the current screen resolution to the sprite (416x234) resolution.
pub fn screen_to_ui(x: f32, y: f32, w: i32, h: i32) -> (f32, f32) {
    let old_x = x;
    let old_y = y;
    (
        ((old_x / w as f32) * WIDTH_PX as f32),
        ((old_y / h as f32) * HEIGHT_PX as f32),
    )
}

/// Translate from the current screen resolution to the base resolution (640x360).
pub fn screen_to_sprite(x: f32, y: f32, w: i32, h: i32) -> (f32, f32) {
    let old_x = x;
    let old_y = y;
    (
        ((old_x / w as f32) * WIDTH as f32),
        ((old_y / h as f32) * HEIGHT as f32),
    )
}

pub fn sprite_to_ui_num(x: i32) -> i32 {
    let x = x as f32;
    ((x / WIDTH_PX as f32) * WIDTH as f32) as i32
}

pub fn world_to_sprite_to_ui_num(coord: i32) -> i32 {
    let num = world_to_screen_num(coord);
    sprite_to_ui_num(num)
}
