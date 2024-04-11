// Will be used to map "actions" to key presses instead of static keycodes

use bevy::prelude::*; // need this even in submodules

#[derive(Resource)]
pub struct ActionMap {
    pub key_map: [KeyCode;Action::MaxSize as usize],
    pub mouse_map: [MouseButton;MouseAction::MaxSize as usize],
}

pub enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    MaxSize,
}

pub enum MouseAction {
    Select,
    MaxSize,
}

impl ActionMap {
    pub fn new() -> Self {
        // TODO: Read from configuration file
        Self {
            key_map: [KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyA, KeyCode::KeyD],
            mouse_map: [MouseButton::Left],
        }
    }

    /**
        Retrieve KeyCode from Action
    */
    pub fn get_key(&self, action: Action) -> KeyCode {
        self.key_map[action as usize]
    }

    /**
        Retrieve MouseButton from MouseAction
    */
    pub fn get_mouse_button(&self, mouse_action: MouseAction) -> MouseButton {
        self.mouse_map[mouse_action as usize]
    }
}
