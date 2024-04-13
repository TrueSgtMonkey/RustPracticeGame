// Will be used to map "actions" to key presses instead of static keycodes

use bevy::prelude::*; // need this even in submodules

pub mod parse_actions;

#[derive(Resource)]
pub struct ActionMap {
    pub key_map: [KeyCode;KeyAction::MaxSize as usize],
    pub mouse_map: [MouseButton;MouseAction::MaxSize as usize],
}

pub enum KeyAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Pause,
    Sprint,
    MaxSize,
}

impl KeyAction {
    /**
        Used for loading
    */
    pub fn get_key_action_from_str(str_var: &str) -> KeyAction {
        match str_var {
            "MoveUp"    => KeyAction::MoveUp,
            "MoveDown"  => KeyAction::MoveDown,
            "MoveLeft"  => KeyAction::MoveLeft,
            "MoveRight" => KeyAction::MoveRight,
            "Pause"     => KeyAction::Pause,
            "Sprint"    => KeyAction::Sprint,
            _ => KeyAction::MaxSize,
        }
    }

    /**
        Used for saving
    */
    pub fn get_str_from_key_action(key_action: KeyAction) -> String {
        match key_action {
            KeyAction::MoveUp    => "MoveUp".to_string(),
            KeyAction::MoveDown  => "MoveDown".to_string(),
            KeyAction::MoveLeft  => "MoveLeft".to_string(),
            KeyAction::MoveRight => "MoveRight".to_string(),
            KeyAction::Pause     => "Pause".to_string(),
            KeyAction::Sprint    => "Sprint".to_string(),
            KeyAction::MaxSize   => "MaxSize".to_string(),
        }
    }
}

pub enum MouseAction {
    Select,
    MaxSize,
}

impl MouseAction {
    pub fn get_mouse_action_from_str(str_var: &str) -> MouseAction {
        match str_var {
            "Select" => MouseAction::Select,
            _ => MouseAction::MaxSize,
        }
    }

    pub fn get_str_from_mouse_action(mouse_action: MouseAction) -> String {
        match mouse_action {
            MouseAction::Select => "Select".to_string(),
            MouseAction::MaxSize => "MaxSize".to_string(),
        }
    }
}

#[derive(Clone, Copy)]
enum ReadMode {
    Keyboard,
    Mouse,
    Error,
}

impl ActionMap {
    pub fn new() -> Self {
        parse_actions::parse_contents(parse_actions::CONFIG_FILE)
    }

    /**
        Retrieve KeyCode from KeyAction
    */
    pub fn get_key(&self, action: KeyAction) -> KeyCode {
        self.key_map[action as usize]
    }

    /**
        Retrieve MouseButton from MouseAction
    */
    pub fn get_mouse_button(&self, mouse_action: MouseAction) -> MouseButton {
        self.mouse_map[mouse_action as usize]
    }

    fn get_read_mode(&self, line: &str) -> ReadMode {
        match line {
            "[Keyboard]" => ReadMode::Keyboard,
            "[Mouse]" => ReadMode::Mouse,
            _ => ReadMode::Error,
        }
    }
}
