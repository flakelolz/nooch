use std::time::Instant;

use imgui::{BackendFlags, Context};
use raylib::prelude::*;

struct LastFrame {
    pub focused: bool,
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub super_key: bool,
}
pub struct RaylibPlatform {
    last_frame: Instant,
    cursor: Option<imgui::MouseCursor>,

    last_frame_data: LastFrame,
}

fn translate_cursor(imgui_cursor: imgui::MouseCursor) -> MouseCursor {
    match imgui_cursor {
        imgui::MouseCursor::Arrow => MouseCursor::MOUSE_CURSOR_ARROW,
        imgui::MouseCursor::TextInput => MouseCursor::MOUSE_CURSOR_IBEAM,
        imgui::MouseCursor::ResizeAll => MouseCursor::MOUSE_CURSOR_RESIZE_ALL,
        imgui::MouseCursor::ResizeNS => MouseCursor::MOUSE_CURSOR_RESIZE_NS,
        imgui::MouseCursor::ResizeEW => MouseCursor::MOUSE_CURSOR_RESIZE_EW,
        imgui::MouseCursor::ResizeNESW => MouseCursor::MOUSE_CURSOR_RESIZE_NESW,
        imgui::MouseCursor::ResizeNWSE => MouseCursor::MOUSE_CURSOR_RESIZE_NWSE,
        imgui::MouseCursor::Hand => MouseCursor::MOUSE_CURSOR_POINTING_HAND,
        imgui::MouseCursor::NotAllowed => MouseCursor::MOUSE_CURSOR_NOT_ALLOWED,
    }
}

fn translate_key(key: KeyboardKey) -> imgui::Key {
    match key {
        KeyboardKey::KEY_A => imgui::Key::A,
        KeyboardKey::KEY_B => imgui::Key::B,
        KeyboardKey::KEY_C => imgui::Key::C,
        KeyboardKey::KEY_D => imgui::Key::D,
        KeyboardKey::KEY_E => imgui::Key::E,
        KeyboardKey::KEY_F => imgui::Key::F,
        KeyboardKey::KEY_G => imgui::Key::G,
        KeyboardKey::KEY_H => imgui::Key::H,
        KeyboardKey::KEY_I => imgui::Key::I,
        KeyboardKey::KEY_J => imgui::Key::J,
        KeyboardKey::KEY_K => imgui::Key::K,
        KeyboardKey::KEY_L => imgui::Key::L,
        KeyboardKey::KEY_M => imgui::Key::M,
        KeyboardKey::KEY_N => imgui::Key::N,
        KeyboardKey::KEY_O => imgui::Key::O,
        KeyboardKey::KEY_P => imgui::Key::P,
        KeyboardKey::KEY_Q => imgui::Key::Q,
        KeyboardKey::KEY_R => imgui::Key::R,
        KeyboardKey::KEY_S => imgui::Key::S,
        KeyboardKey::KEY_T => imgui::Key::T,
        KeyboardKey::KEY_U => imgui::Key::U,
        KeyboardKey::KEY_V => imgui::Key::V,
        KeyboardKey::KEY_W => imgui::Key::W,
        KeyboardKey::KEY_X => imgui::Key::X,
        KeyboardKey::KEY_Y => imgui::Key::Y,
        KeyboardKey::KEY_Z => imgui::Key::Z,
        KeyboardKey::KEY_ONE => imgui::Key::Keypad1,
        KeyboardKey::KEY_TWO => imgui::Key::Keypad2,
        KeyboardKey::KEY_THREE => imgui::Key::Keypad3,
        KeyboardKey::KEY_FOUR => imgui::Key::Keypad4,
        KeyboardKey::KEY_FIVE => imgui::Key::Keypad5,
        KeyboardKey::KEY_SIX => imgui::Key::Keypad6,
        KeyboardKey::KEY_SEVEN => imgui::Key::Keypad7,
        KeyboardKey::KEY_EIGHT => imgui::Key::Keypad8,
        KeyboardKey::KEY_NINE => imgui::Key::Keypad9,
        KeyboardKey::KEY_ZERO => imgui::Key::Keypad0,
        KeyboardKey::KEY_ENTER => imgui::Key::Enter,
        KeyboardKey::KEY_ESCAPE => imgui::Key::Escape,
        KeyboardKey::KEY_BACKSPACE => imgui::Key::Backspace,
        KeyboardKey::KEY_TAB => imgui::Key::Tab,
        KeyboardKey::KEY_SPACE => imgui::Key::Space,
        KeyboardKey::KEY_MINUS => imgui::Key::Minus,
        KeyboardKey::KEY_EQUAL => imgui::Key::Equal,
        KeyboardKey::KEY_LEFT_BRACKET => imgui::Key::LeftBracket,
        KeyboardKey::KEY_RIGHT_BRACKET => imgui::Key::RightBracket,
        KeyboardKey::KEY_BACKSLASH => imgui::Key::Backslash,
        KeyboardKey::KEY_SEMICOLON => imgui::Key::Semicolon,
        KeyboardKey::KEY_APOSTROPHE => imgui::Key::Apostrophe,
        KeyboardKey::KEY_GRAVE => imgui::Key::GraveAccent,
        KeyboardKey::KEY_COMMA => imgui::Key::Comma,
        KeyboardKey::KEY_PERIOD => imgui::Key::Period,
        KeyboardKey::KEY_SLASH => imgui::Key::Slash,
        KeyboardKey::KEY_CAPS_LOCK => imgui::Key::CapsLock,
        KeyboardKey::KEY_F1 => imgui::Key::F1,
        KeyboardKey::KEY_F2 => imgui::Key::F2,
        KeyboardKey::KEY_F3 => imgui::Key::F3,
        KeyboardKey::KEY_F4 => imgui::Key::F4,
        KeyboardKey::KEY_F5 => imgui::Key::F5,
        KeyboardKey::KEY_F6 => imgui::Key::F6,
        KeyboardKey::KEY_F7 => imgui::Key::F7,
        KeyboardKey::KEY_F8 => imgui::Key::F8,
        KeyboardKey::KEY_F9 => imgui::Key::F9,
        KeyboardKey::KEY_F10 => imgui::Key::F10,
        KeyboardKey::KEY_F11 => imgui::Key::F11,
        KeyboardKey::KEY_F12 => imgui::Key::F12,
        KeyboardKey::KEY_PRINT_SCREEN => imgui::Key::PrintScreen,
        KeyboardKey::KEY_SCROLL_LOCK => imgui::Key::ScrollLock,
        KeyboardKey::KEY_PAUSE => imgui::Key::Pause,
        KeyboardKey::KEY_INSERT => imgui::Key::Insert,
        KeyboardKey::KEY_HOME => imgui::Key::Home,
        KeyboardKey::KEY_PAGE_UP => imgui::Key::PageUp,
        KeyboardKey::KEY_DELETE => imgui::Key::Delete,
        KeyboardKey::KEY_END => imgui::Key::End,
        KeyboardKey::KEY_PAGE_DOWN => imgui::Key::PageDown,
        KeyboardKey::KEY_RIGHT => imgui::Key::RightArrow,
        KeyboardKey::KEY_LEFT => imgui::Key::LeftArrow,
        KeyboardKey::KEY_DOWN => imgui::Key::DownArrow,
        KeyboardKey::KEY_UP => imgui::Key::UpArrow,
        KeyboardKey::KEY_KP_DIVIDE => imgui::Key::KeypadDivide,
        KeyboardKey::KEY_KP_MULTIPLY => imgui::Key::KeypadMultiply,
        KeyboardKey::KEY_KP_SUBTRACT => imgui::Key::KeypadSubtract,
        KeyboardKey::KEY_KP_ADD => imgui::Key::KeypadAdd,
        KeyboardKey::KEY_KP_ENTER => imgui::Key::KeypadEnter,
        KeyboardKey::KEY_KP_1 => imgui::Key::Keypad1,
        KeyboardKey::KEY_KP_2 => imgui::Key::Keypad2,
        KeyboardKey::KEY_KP_3 => imgui::Key::Keypad3,
        KeyboardKey::KEY_KP_4 => imgui::Key::Keypad4,
        KeyboardKey::KEY_KP_5 => imgui::Key::Keypad5,
        KeyboardKey::KEY_KP_6 => imgui::Key::Keypad6,
        KeyboardKey::KEY_KP_7 => imgui::Key::Keypad7,
        KeyboardKey::KEY_KP_8 => imgui::Key::Keypad8,
        KeyboardKey::KEY_KP_9 => imgui::Key::Keypad9,
        KeyboardKey::KEY_KP_0 => imgui::Key::Keypad0,
        KeyboardKey::KEY_KP_DECIMAL => imgui::Key::KeypadDecimal,
        KeyboardKey::KEY_KB_MENU => imgui::Key::Menu,
        KeyboardKey::KEY_KP_EQUAL => imgui::Key::KeypadEqual,
        KeyboardKey::KEY_LEFT_CONTROL => imgui::Key::LeftCtrl,
        KeyboardKey::KEY_LEFT_SHIFT => imgui::Key::LeftShift,
        KeyboardKey::KEY_LEFT_ALT => imgui::Key::LeftAlt,
        KeyboardKey::KEY_LEFT_SUPER => imgui::Key::LeftSuper,
        KeyboardKey::KEY_RIGHT_CONTROL => imgui::Key::RightCtrl,
        KeyboardKey::KEY_RIGHT_SHIFT => imgui::Key::RightShift,
        KeyboardKey::KEY_RIGHT_ALT => imgui::Key::RightAlt,
        KeyboardKey::KEY_RIGHT_SUPER => imgui::Key::RightSuper,
        _ => imgui::Key::Escape, // TODO Figure this one out
    }
}

fn keyboard_key_from_u32(key: u32) -> KeyboardKey {
    match key {
        39 => KeyboardKey::KEY_APOSTROPHE,
        44 => KeyboardKey::KEY_COMMA,
        45 => KeyboardKey::KEY_MINUS,
        46 => KeyboardKey::KEY_PERIOD,
        47 => KeyboardKey::KEY_SLASH,
        48 => KeyboardKey::KEY_ZERO,
        49 => KeyboardKey::KEY_ONE,
        50 => KeyboardKey::KEY_TWO,
        51 => KeyboardKey::KEY_THREE,
        52 => KeyboardKey::KEY_FOUR,
        53 => KeyboardKey::KEY_FIVE,
        54 => KeyboardKey::KEY_SIX,
        55 => KeyboardKey::KEY_SEVEN,
        56 => KeyboardKey::KEY_EIGHT,
        57 => KeyboardKey::KEY_NINE,
        59 => KeyboardKey::KEY_SEMICOLON,
        61 => KeyboardKey::KEY_EQUAL,
        65 => KeyboardKey::KEY_A,
        66 => KeyboardKey::KEY_B,
        67 => KeyboardKey::KEY_C,
        68 => KeyboardKey::KEY_D,
        69 => KeyboardKey::KEY_E,
        70 => KeyboardKey::KEY_F,
        71 => KeyboardKey::KEY_G,
        72 => KeyboardKey::KEY_H,
        73 => KeyboardKey::KEY_I,
        74 => KeyboardKey::KEY_J,
        75 => KeyboardKey::KEY_K,
        76 => KeyboardKey::KEY_L,
        77 => KeyboardKey::KEY_M,
        78 => KeyboardKey::KEY_N,
        79 => KeyboardKey::KEY_O,
        80 => KeyboardKey::KEY_P,
        81 => KeyboardKey::KEY_Q,
        82 => KeyboardKey::KEY_R,
        83 => KeyboardKey::KEY_S,
        84 => KeyboardKey::KEY_T,
        85 => KeyboardKey::KEY_U,
        86 => KeyboardKey::KEY_V,
        87 => KeyboardKey::KEY_W,
        88 => KeyboardKey::KEY_X,
        89 => KeyboardKey::KEY_Y,
        90 => KeyboardKey::KEY_Z,
        91 => KeyboardKey::KEY_LEFT_BRACKET,
        92 => KeyboardKey::KEY_BACKSLASH,
        93 => KeyboardKey::KEY_RIGHT_BRACKET,
        96 => KeyboardKey::KEY_GRAVE,
        32 => KeyboardKey::KEY_SPACE,
        256 => KeyboardKey::KEY_ESCAPE,
        257 => KeyboardKey::KEY_ENTER,
        258 => KeyboardKey::KEY_TAB,
        259 => KeyboardKey::KEY_BACKSPACE,
        260 => KeyboardKey::KEY_INSERT,
        261 => KeyboardKey::KEY_DELETE,
        262 => KeyboardKey::KEY_RIGHT,
        263 => KeyboardKey::KEY_LEFT,
        264 => KeyboardKey::KEY_DOWN,
        265 => KeyboardKey::KEY_UP,
        266 => KeyboardKey::KEY_PAGE_UP,
        267 => KeyboardKey::KEY_PAGE_DOWN,
        268 => KeyboardKey::KEY_HOME,
        269 => KeyboardKey::KEY_END,
        280 => KeyboardKey::KEY_CAPS_LOCK,
        281 => KeyboardKey::KEY_SCROLL_LOCK,
        282 => KeyboardKey::KEY_NUM_LOCK,
        283 => KeyboardKey::KEY_PRINT_SCREEN,
        284 => KeyboardKey::KEY_PAUSE,
        290 => KeyboardKey::KEY_F1,
        291 => KeyboardKey::KEY_F2,
        292 => KeyboardKey::KEY_F3,
        293 => KeyboardKey::KEY_F4,
        294 => KeyboardKey::KEY_F5,
        295 => KeyboardKey::KEY_F6,
        296 => KeyboardKey::KEY_F7,
        297 => KeyboardKey::KEY_F8,
        298 => KeyboardKey::KEY_F9,
        299 => KeyboardKey::KEY_F10,
        300 => KeyboardKey::KEY_F11,
        301 => KeyboardKey::KEY_F12,
        340 => KeyboardKey::KEY_LEFT_SHIFT,
        341 => KeyboardKey::KEY_LEFT_CONTROL,
        342 => KeyboardKey::KEY_LEFT_ALT,
        343 => KeyboardKey::KEY_LEFT_SUPER,
        344 => KeyboardKey::KEY_RIGHT_SHIFT,
        345 => KeyboardKey::KEY_RIGHT_CONTROL,
        346 => KeyboardKey::KEY_RIGHT_ALT,
        347 => KeyboardKey::KEY_RIGHT_SUPER,
        348 => KeyboardKey::KEY_KB_MENU,
        320 => KeyboardKey::KEY_KP_0,
        321 => KeyboardKey::KEY_KP_1,
        322 => KeyboardKey::KEY_KP_2,
        323 => KeyboardKey::KEY_KP_3,
        324 => KeyboardKey::KEY_KP_4,
        325 => KeyboardKey::KEY_KP_5,
        326 => KeyboardKey::KEY_KP_6,
        327 => KeyboardKey::KEY_KP_7,
        328 => KeyboardKey::KEY_KP_8,
        329 => KeyboardKey::KEY_KP_9,
        330 => KeyboardKey::KEY_KP_DECIMAL,
        331 => KeyboardKey::KEY_KP_DIVIDE,
        332 => KeyboardKey::KEY_KP_MULTIPLY,
        333 => KeyboardKey::KEY_KP_SUBTRACT,
        334 => KeyboardKey::KEY_KP_ADD,
        335 => KeyboardKey::KEY_KP_ENTER,
        336 => KeyboardKey::KEY_KP_EQUAL,
        4 => KeyboardKey::KEY_BACK,
        24 => KeyboardKey::KEY_VOLUME_UP,
        25 => KeyboardKey::KEY_VOLUME_DOWN,
        _ => KeyboardKey::KEY_NULL,
    }
}

fn translate_mouse_button(button: MouseButton) -> imgui::MouseButton {
    match button {
        MouseButton::MOUSE_BUTTON_LEFT => imgui::MouseButton::Left,
        MouseButton::MOUSE_BUTTON_RIGHT => imgui::MouseButton::Right,
        MouseButton::MOUSE_BUTTON_MIDDLE => imgui::MouseButton::Middle,
        MouseButton::MOUSE_BUTTON_SIDE => imgui::MouseButton::Left,
        MouseButton::MOUSE_BUTTON_EXTRA => imgui::MouseButton::Left,
        MouseButton::MOUSE_BUTTON_FORWARD => imgui::MouseButton::Extra1,
        MouseButton::MOUSE_BUTTON_BACK => imgui::MouseButton::Extra2,
    }
}

fn mouse_button_from_u32(button: u32) -> MouseButton {
    match button {
        0 => MouseButton::MOUSE_BUTTON_LEFT,
        1 => MouseButton::MOUSE_BUTTON_RIGHT,
        2 => MouseButton::MOUSE_BUTTON_MIDDLE,
        3 => MouseButton::MOUSE_BUTTON_SIDE,
        4 => MouseButton::MOUSE_BUTTON_EXTRA,
        5 => MouseButton::MOUSE_BUTTON_FORWARD,
        6 => MouseButton::MOUSE_BUTTON_BACK,
        _ => MouseButton::MOUSE_BUTTON_LEFT,
    }
}

impl RaylibPlatform {
    pub fn init(rl: &mut RaylibHandle, imgui: &mut Context) -> RaylibPlatform {
        let io = imgui.io_mut();

        io.backend_flags.insert(BackendFlags::HAS_MOUSE_CURSORS);
        io.backend_flags.insert(BackendFlags::HAS_SET_MOUSE_POS);

        imgui.set_platform_name(Some(String::from("imgui-raylib-support")));

        imgui.style_mut().use_dark_colors();

        RaylibPlatform {
            last_frame: Instant::now(),
            cursor: None,

            last_frame_data: LastFrame {
                focused: rl.is_window_focused(),
                ctrl: false,
                shift: false,
                alt: false,
                super_key: false,
            },
        }
    }

    pub fn new_frame(&mut self, rl: &mut raylib::RaylibHandle, context: &mut Context) {
        let io = context.io_mut();

        if rl.is_window_fullscreen() {
            let monitor = raylib::window::get_current_monitor();
            io.display_size[0] = raylib::window::get_monitor_width(monitor) as f32;
            io.display_size[1] = raylib::window::get_monitor_height(monitor) as f32;
        } else {
            io.display_size[0] = rl.get_screen_width() as f32;
            io.display_size[1] = rl.get_screen_height() as f32;
        }

        let [width, height] = io.display_size;

        if width > 0.0 && height > 0.0 {
            io.display_framebuffer_scale = [
                width.floor() / io.display_size[0],
                height.floor() / io.display_size[1],
            ];
        } else {
            io.display_framebuffer_scale = [1.0, 1.0];
        }

        let now = Instant::now();
        let delta_time = now - self.last_frame;
        self.last_frame = now;

        io.delta_time = delta_time.as_secs_f32();

        if io.want_set_mouse_pos {
            rl.set_mouse_position(Vector2 {
                x: io.mouse_pos[0],
                y: io.mouse_pos[1],
            });
        } else {
            io.mouse_pos[0] = rl.get_mouse_x() as f32;
            io.mouse_pos[1] = rl.get_mouse_y() as f32;
        }

        io.mouse_down[0] = rl.is_mouse_button_down(consts::MouseButton::MOUSE_BUTTON_LEFT);
        io.mouse_down[1] = rl.is_mouse_button_down(consts::MouseButton::MOUSE_BUTTON_RIGHT);
        io.mouse_down[2] = rl.is_mouse_button_down(consts::MouseButton::MOUSE_BUTTON_MIDDLE);

        let mouse_wheel = rl.get_mouse_wheel_move_v();
        io.mouse_wheel += mouse_wheel.y;
        io.mouse_wheel_h += mouse_wheel.x;

        if !io
            .config_flags
            .contains(imgui::ConfigFlags::NO_MOUSE_CURSOR_CHANGE)
        {
            let imgui_cursor = context.mouse_cursor();
            if imgui_cursor != self.cursor {
                self.cursor = imgui_cursor;

                if let Some(cursor) = imgui_cursor {
                    rl.show_cursor();

                    rl.set_mouse_cursor(translate_cursor(cursor));
                } else {
                    rl.hide_cursor();
                }
            }
        }
    }

    pub fn handle_events(&mut self, rl: &mut raylib::RaylibHandle, context: &mut Context) {
        let io = context.io_mut();

        let focused = rl.is_window_focused();
        if focused != self.last_frame_data.focused {
            self.last_frame_data.focused = focused;
        }

        let ctrl = rl.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL)
            || rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL);
        if ctrl != self.last_frame_data.ctrl {
            self.last_frame_data.ctrl = ctrl;
            io.add_key_event(imgui::Key::ModCtrl, ctrl);
        }

        let alt =
            rl.is_key_down(KeyboardKey::KEY_RIGHT_ALT) || rl.is_key_down(KeyboardKey::KEY_LEFT_ALT);
        if alt != self.last_frame_data.alt {
            self.last_frame_data.alt = alt;
            io.add_key_event(imgui::Key::ModAlt, alt);
        }

        let shift = rl.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT)
            || rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT);
        if shift != self.last_frame_data.shift {
            self.last_frame_data.shift = shift;
            io.add_key_event(imgui::Key::ModShift, shift);
        }

        let super_key = rl.is_key_down(KeyboardKey::KEY_RIGHT_SUPER)
            || rl.is_key_down(KeyboardKey::KEY_LEFT_SUPER);
        if super_key != self.last_frame_data.super_key {
            self.last_frame_data.super_key = super_key;
            io.add_key_event(imgui::Key::ModSuper, super_key);
        }

        while let Some(key) = rl.get_key_pressed() {
            io.add_key_event(translate_key(key), true);
        }

        for i in 0..io.keys_down.len() {
            let key = keyboard_key_from_u32(i as u32);
            if key == KeyboardKey::KEY_NULL {
                continue;
            }

            if rl.is_key_released(key) {
                io.add_key_event(translate_key(key), false);
            }
        }

        while let Some(char) = rl.get_char_pressed() {
            io.add_input_character(char);
        }

        for i in 0..imgui::MouseButton::COUNT {
            let button = mouse_button_from_u32(i as u32);
            if rl.is_mouse_button_pressed(button) {
                io.add_mouse_button_event(translate_mouse_button(button), true);
            }
            if rl.is_mouse_button_released(button) {
                io.add_mouse_button_event(translate_mouse_button(button), false);
            }
        }
    }
}
