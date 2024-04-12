use bevy::prelude::*;

use player::PlayerPlugin; // need this even in submodules
use player::camera::CameraPlugin;

const DEBUG_PRINT_KEYS: bool = false;

pub mod player;

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CameraPlugin, PlayerPlugin));

        // Add optional debug statements down here
        if DEBUG_PRINT_KEYS {
            app.add_systems(Update, print_debug_keys);
        }
    }
}

/**
    Prints out number keys to the terminal to make it easier to verify if keys
    are being saved correctly
*/
fn print_debug_keys(keyboard_input: Res<ButtonInput<KeyCode>>) {
    for key in keyboard_input.get_just_pressed() {
        unsafe { 
            let key: u32 = std::mem::transmute_copy(key);
            println!("key={}", key);
        }
    }
}
