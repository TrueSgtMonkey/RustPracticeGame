// use super::<super_trait>; // include stuff from module above this one

use bevy::prelude::*; // need this even in submodules
use crate::actions::*;
use crate::components::CharacterEntity;

pub mod camera;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct PlayerEntity;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, change_player_velocity);
    }
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_texture: Handle<Image> = asset_server.load("open_source_assets/tilesets/tilemap.png");
    commands.spawn(
        (
            PlayerEntity, 
            SpriteSheetBundle {
                texture: player_texture,
                ..Default::default()
            },
            CharacterEntity {
                speed: 135.0f32,
                ..Default::default()
            },
        )
    );
}

fn change_player_velocity (
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    action_map: Res<ActionMap>, 
    mut characters: Query<&mut CharacterEntity, With<PlayerEntity>>
)
{
    let mut input_count: u8 = 0;
    let mut velocity: Vec2 = Vec2 {
        x: 0.0f32,
        y: 0.0f32
    };

    if keyboard_input.pressed(action_map.get_key(KeyAction::MoveUp)) {
        velocity.y += 1f32;
        input_count += 1;
    }

    if keyboard_input.pressed(action_map.get_key(KeyAction::MoveDown)) {
        velocity.y += -1f32;
        input_count += 1;
    }

    if keyboard_input.pressed(action_map.get_key(KeyAction::MoveRight)) {
        velocity.x += 1f32;
        input_count += 1;
    }

    if keyboard_input.pressed(action_map.get_key(KeyAction::MoveLeft)) {
        velocity.x += -1f32;
        input_count += 1;
    }

    // no keys were entered -- velocity will be 0
    if input_count == 0 {
        for mut character in &mut characters {
            character.velocity = velocity;
        }
        return;
    }

    velocity = velocity.normalize_or_zero();
    for mut character in &mut characters {
        character.velocity = velocity * character.speed;
    }
}
