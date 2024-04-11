// use super::<super_trait>; // include stuff from module above this one

use bevy::prelude::*; // need this even in submodules
use crate::actions::*;

pub mod camera;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct PlayerEntity;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, handle_movement);
    }
}

fn setup_player(mut commands: Commands) {
    commands.spawn(PlayerEntity);
}

fn handle_movement (keyboard_input: Res<ButtonInput<KeyCode>>, action_map: Res<ActionMap>) {
    if keyboard_input.pressed(action_map.get_key(Action::MoveUp)) {
        println!("Move Up")
    }

    if keyboard_input.pressed(action_map.get_key(Action::MoveDown)) {
        println!("Move Down")
    }

    if keyboard_input.pressed(action_map.get_key(Action::MoveRight)) {
        println!("Move Right")
    }

    if keyboard_input.pressed(action_map.get_key(Action::MoveLeft)) {
        println!("Move Left")
    }
}
