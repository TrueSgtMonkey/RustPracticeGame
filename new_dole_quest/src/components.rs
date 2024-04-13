use bevy::prelude::*;

use player::PlayerPlugin; // need this even in submodules
use player::camera::CameraPlugin;
use object::ObjectPlugin;

const DEBUG_PRINT_KEYS:  bool = false;
const DEBUG_PRINT_MOUSE: bool = false;

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

#[derive(Component)]
pub struct CharacterEntity {
    pub velocity: Vec2,
    pub speed: f32,
}

impl Default for CharacterEntity {
    fn default() -> Self {
        Self {
            velocity: Vec2 {
                ..Default::default()
            },
            speed: 1.0f32,
        }
    }
}

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CameraPlugin, PlayerPlugin, ObjectPlugin))
            .add_systems(Update, move_characters);

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

/**
    This function updates the transforms of all entities accoriding to their
    velocities.
    
    For simplification, delta_seconds is multiplied against the
    velocity to ensure that they move at a consistent rate independent
    of hardware.
*/
fn move_characters(
    mut characters: Query<(&mut CharacterEntity, &mut Transform), With<CharacterEntity>>,
    time: Res<Time>
) 
{
    for (character, mut transform) in &mut characters {
        let change: Vec2 = character.velocity * time.delta_seconds();
        transform.translation.x += change.x;
        transform.translation.y += change.y;
    }
}