// Will be used to map "actions" to key presses instead of static keycodes

use bevy::prelude::*; // need this even in submodules

const CONFIG_FILE: &str = "./config/input_config.cfg";

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
        parse_contents(CONFIG_FILE)
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

/**
    This function parses a config file and loads in the correct Code
    (KeyCode/MouseButton/Etc...) for each Action
    (KeyAction/MouseAction/Etc...).
*/
// TODO: Refactor perhaps? Split into different functions?
fn parse_contents(filename: &str) -> ActionMap {
    let mut read_mode: ReadMode = ReadMode::Keyboard;
    let mut action_map: ActionMap = ActionMap {
        key_map: [KeyCode::KeyW;KeyAction::MaxSize as usize],
        mouse_map: [MouseButton::Left;MouseAction::MaxSize as usize],
    };

    for line in std::fs::read_to_string(filename).unwrap().lines() {
        // Empty lines will exist in this file
        if line.chars().count() == 0 {
            continue;
        }
        let line = line.trim();

        // changing the mode to (hopefully) a valid ReadMode
        if line.as_bytes()[0] == '[' as u8 {
            read_mode = action_map.get_read_mode(line);
            // println!("ReadMode={}", read_mode as u16); // useful for verifying if read_mode changing
            continue;
        }

        let equal_idx = line.find("=").unwrap();
        let left_side:  &str = &line[..equal_idx].trim();
        let right_side: &str = &line[(equal_idx+1)..].trim();
        match read_mode {
            ReadMode::Keyboard => {
                let key_action_idx: usize = KeyAction::get_key_action_from_str(left_side) as usize;
                let key_code: KeyCode = get_key_code_from_line(right_side);
                action_map.key_map[key_action_idx] = key_code;
            }
            ReadMode::Mouse => {
                let mouse_action_idx: usize = MouseAction::get_mouse_action_from_str(left_side) as usize;
                let mouse_button: MouseButton = get_mouse_button_from_line(right_side);
                action_map.mouse_map[mouse_action_idx] = mouse_button;
            }
            ReadMode::Error => {
                println!("Error found at: {}", line);
            }
        }
    }

    return action_map;
}

fn get_key_code_from_line(right_side: &str) -> KeyCode {
    // hacky way, but beats not being able to make this work without a mega match statement
    let key_code: u128 = right_side.trim().parse().expect("right_side must be a number!");
    let key_code: KeyCode = unsafe {std::mem::transmute_copy(&key_code)};
    return key_code;
}

fn get_mouse_button_from_line(right_side: &str) -> MouseButton {
    let mouse_button: u32 = right_side.trim().parse().expect("right_side must be a number!");
    let mouse_button: MouseButton = unsafe {std::mem::transmute(mouse_button)};
    return mouse_button;
}

// TODO: Implement saving system for key_code from KeyActions
// TODO: Make KeyAction and MouseAction editor
// Making these _ for now to keep from warnings, but remember to implement key 
// editing and saving system -- avoid manually editing
fn _get_u32_from_key_code(key_code: KeyCode) -> u32 {
    unsafe { 
        let key_code: u32 = std::mem::transmute_copy(&key_code);
        return key_code;
    }
}

fn _get_u32_from_mouse_button(mouse_button: MouseButton) -> u32 {
    unsafe { 
        let mouse_button: u32 = std::mem::transmute_copy(&mouse_button);
        return mouse_button;
    }
}
