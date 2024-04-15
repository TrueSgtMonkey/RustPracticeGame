use bevy::prelude::*;

use player::PlayerPlugin; // need this even in submodules
use player::camera::CameraPlugin;
use object::ObjectPlugin;
use character::{CharacterPlugin, CharacterEntity};

pub mod character;

const DEBUG_PRINT_KEYS:  bool = false;
const DEBUG_PRINT_MOUSE: bool = false;

const BASELINE_SIZE_COMPONENT: Vec2 = Vec2 {
    x: 64.0,
    y: 64.0,
};

pub mod player;
pub mod object;

pub struct ComponentsPlugin;

#[derive(Component)]
pub struct StaticEntity;

impl Default for StaticEntity {
    fn default() -> Self {
        Self
    }
}

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        println!("Baseline size for sprites: {:?}", BASELINE_SIZE_COMPONENT);

        app.add_plugins((
            CameraPlugin,
            PlayerPlugin,
            ObjectPlugin,
            CharacterPlugin)
        );

        // Add optional debug statements down here
        if DEBUG_PRINT_KEYS {
            app.add_systems(Update, print_debug_keys);
        }
        if DEBUG_PRINT_MOUSE {
            app.add_systems(Update, print_debug_mouse);
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

/**
    Prints mouse inputs as codes to the terminal to make it easier to verify
    if keys are being saved correctly
*/
fn print_debug_mouse(mouse_input: Res<ButtonInput<MouseButton>>) {
    for mouse_press in mouse_input.get_just_pressed() {
        unsafe { 
            let mouse_press: u32 = std::mem::transmute_copy(mouse_press);
            println!("mouse_press={}", mouse_press);
        }
    }
}
