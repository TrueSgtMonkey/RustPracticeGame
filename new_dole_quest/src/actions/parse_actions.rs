use bevy::prelude::*; // need this even in submodules
use super::{ActionMap, KeyAction, MouseAction, ReadMode};

pub const CONFIG_FILE: &str = "./config/input_config.cfg";

/**
    This function parses a config file and loads in the correct Code
    (KeyCode/MouseButton/Etc...) for each Action
    (KeyAction/MouseAction/Etc...).
*/
// TODO: Refactor perhaps? Split into different functions?
pub fn parse_contents(filename: &str) -> ActionMap {
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
        } else if line.as_bytes()[0] == ';' as u8 {
            continue;
        }

        let line = comment_strip(line);
        let equal_idx = line.find("=");
        if equal_idx == None {
            println!("Cannot parse: {} -- No equal sign!", line);
            continue;
        }

        let equal_idx: usize = equal_idx.unwrap();
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

fn comment_strip(line: &str) -> &str {
    // need to make sure this doesn't return the None param
    let comment_idx = line.find(";");
    if comment_idx == None {
        return line;
    }

    let comment_idx: usize = comment_idx.unwrap();
    return &line[..comment_idx];
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