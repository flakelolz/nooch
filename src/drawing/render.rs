use crate::prelude::*;
pub fn create_render_targets(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
) -> (RenderTexture2D, RenderTexture2D) {
    let target_px = rl
        .load_render_texture(thread, WIDTH_PX as u32, HEIGHT_PX as u32)
        .unwrap();
    let target_ui = rl
        .load_render_texture(thread, WIDTH as u32, HEIGHT as u32)
        .unwrap();

    (target_px, target_ui)
}

pub fn rendering(sprite: &mut RenderTexture2D, ui: &mut RenderTexture2D, d: &mut impl RaylibDraw) {
    d.draw_texture_pro(
        sprite.texture(),
        rrect(0.0, 0.0, sprite.texture.width, -sprite.texture.height),
        rrect(0, 0, WIDTH, HEIGHT),
        rvec2(0, 0),
        0.0,
        Color::WHITE,
    );

    d.draw_texture_pro(
        ui.texture(),
        rrect(0.0, 0.0, ui.texture.width, -ui.texture.height),
        rrect(0, 0, WIDTH, HEIGHT),
        rvec2(0, 0),
        0.0,
        Color::WHITE,
    );
}
