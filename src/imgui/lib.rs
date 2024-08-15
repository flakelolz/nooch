mod clipboard;
mod platform;
mod renderer;

use imgui::Io;

pub struct RaylibImguiSupport {
    context: imgui::Context,
    renderer: renderer::RaylibRenderer,
    platform: platform::RaylibPlatform,
}

impl RaylibImguiSupport {
    pub fn setup(
        rl: &mut raylib::RaylibHandle,
        thread: &raylib::RaylibThread,
    ) -> RaylibImguiSupport {
        let mut context = imgui::Context::create();
        context.set_ini_filename(None);
        context.set_log_filename(None);
        context
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

        if let Some(support) = clipboard::ClipboardSupport::init() {
            context.set_clipboard_backend(support);
        } else {
            println!("Could not set clipboard backend");
        }

        let renderer = renderer::RaylibRenderer::init(rl, thread, &mut context);
        let platform = platform::RaylibPlatform::init(rl, &mut context);

        RaylibImguiSupport {
            context,
            renderer,
            platform,
        }
    }

    pub fn start_frame(&mut self, rl: &mut raylib::RaylibHandle) -> &mut imgui::Ui {
        self.platform.new_frame(rl, &mut self.context);
        self.platform.handle_events(rl, &mut self.context);

        self.context.new_frame()
    }

    pub fn end_frame(&mut self, d: &mut raylib::drawing::RaylibDrawHandle) {
        // pub fn end_frame(&mut self, d: &mut raylib::drawing::RaylibMode2D<raylib::drawing::RaylibDrawHandle>) {
        // pub fn end_frame(&mut self, d: &mut raylib::drawing::RaylibTextureMode<raylib::drawing::RaylibDrawHandle>) {
        let [fb_x, fb_y] = self.context.io_mut().display_framebuffer_scale;
        let draw_data = self.context.render();

        self.renderer.render(d, draw_data, [fb_x, fb_y]);
    }

    pub fn io(&mut self) -> &mut Io {
        self.context.io_mut()
    }
}
