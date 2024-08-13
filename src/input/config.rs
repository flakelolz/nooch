use crate::prelude::*;
use raylib::consts::{GamepadButton, KeyboardKey};

pub struct Keyboard {
    pub up: KeyboardKey,
    pub down: KeyboardKey,
    pub left: KeyboardKey,
    pub right: KeyboardKey,
    pub lp: KeyboardKey,
    pub mp: KeyboardKey,
    pub hp: KeyboardKey,
    pub lk: KeyboardKey,
    pub mk: KeyboardKey,
    pub hk: KeyboardKey,
}

impl Keyboard {
    pub fn one() -> Self {
        Self {
            up: KeyboardKey::KEY_SPACE,
            down: KeyboardKey::KEY_S,
            left: KeyboardKey::KEY_A,
            right: KeyboardKey::KEY_D,
            lp: KeyboardKey::KEY_U,
            mp: KeyboardKey::KEY_I,
            hp: KeyboardKey::KEY_O,
            lk: KeyboardKey::KEY_J,
            mk: KeyboardKey::KEY_K,
            hk: KeyboardKey::KEY_L,
        }
    }

    pub fn two() -> Self {
        Self {
            up: KeyboardKey::KEY_UP,
            down: KeyboardKey::KEY_DOWN,
            left: KeyboardKey::KEY_LEFT,
            right: KeyboardKey::KEY_RIGHT,
            lp: KeyboardKey::KEY_ONE,
            mp: KeyboardKey::KEY_TWO,
            hp: KeyboardKey::KEY_THREE,
            lk: KeyboardKey::KEY_FOUR,
            mk: KeyboardKey::KEY_FIVE,
            hk: KeyboardKey::KEY_SIX,
        }
    }
}

pub struct Gamepad {
    pub up: GamepadButton,
    pub down: GamepadButton,
    pub left: GamepadButton,
    pub right: GamepadButton,
    pub lp: GamepadButton,
    pub mp: GamepadButton,
    pub hp: GamepadButton,
    pub lk: GamepadButton,
    pub mk: GamepadButton,
    pub hk: GamepadButton,
}

impl Gamepad {
    pub fn new() -> Self {
        Self {
            up: GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_UP,
            down: GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_DOWN,
            left: GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_LEFT,
            right: GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_RIGHT,
            lp: GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_LEFT,
            mp: GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_UP,
            hp: GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_1,
            lk: GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN,
            mk: GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_RIGHT,
            hk: GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_2,
        }
    }
}

#[derive(Component)]
pub struct InputConfig {
    pub kb: Keyboard,
    pub pad: Gamepad,
}

impl InputConfig {
    pub fn one() -> Self {
        Self {
            kb: Keyboard::one(),
            pad: Gamepad::new(),
        }
    }

    pub fn two() -> Self {
        Self {
            kb: Keyboard::two(),
            pad: Gamepad::new(),
        }
    }
}

