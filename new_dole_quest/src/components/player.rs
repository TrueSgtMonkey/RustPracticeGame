// use super::<super_trait>; // include stuff from module above this one

use bevy::prelude::*; // need this even in submodules
use crate::actions::*;
use crate::components::CharacterEntity;
use crate::animation::AnimationIndices;

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

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
)
{
    let player_texture: Handle<Image> = asset_server.load("player/player_model/player_animation.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(128.0, 128.0), 1, 8, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices {
        first: 0,
        last: 7
    };

    commands.spawn(
        (
            PlayerEntity, 
            SpriteSheetBundle {
                texture: player_texture,
                transform: Transform {
                    scale: Vec3 {
                        x: 0.5,
                        y: 0.5,
                        z: 0.5,
                    },
                    ..Default::default()
                },
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
                ..Default::default()
            },
            CharacterEntity {
                speed: 135.0f32,
                sprint_multiplier: 1.75,
                ..Default::default()
            },
        )
    );
}

fn change_player_velocity (
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    action_map: Res<ActionMap>, 
    mut player_group: Query<&mut CharacterEntity, With<PlayerEntity>>
)
{
    let mut input_count: u8 = 0;
    let mut velocity: Vec2 = Vec2 {
        x: 0.0f32,
        y: 0.0f32
    };

    let mut sprint_multiplier: f32 = 1.0;
    if keyboard_input.pressed(action_map.get_key(KeyAction::Sprint)) {
        for player in &player_group {
            sprint_multiplier = player.sprint_multiplier;
        }
    }

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
        for mut player in &mut player_group {
            player.velocity = velocity;
        }
        return;
    }

    velocity = velocity.normalize_or_zero();
    for mut player in &mut player_group {
        player.velocity = velocity * player.speed * sprint_multiplier;
    }
}
