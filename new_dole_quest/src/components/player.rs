// use super::<super_trait>; // include stuff from module above this one

use std::time::Duration;

use bevy::prelude::*; // need this even in submodules
use crate::actions::*;
use crate::components::CharacterEntity;
use crate::animation::AnimationIndices;
use crate::animation::sprite_animation::AnimatedEntity;

pub mod camera;

const START_ANGLE_WALK: usize   = 0;
const START_ANGLE_IDLE: usize   = 8;
const START_ANGLE_SPRINT: usize = 16;
const START_ANGLE_MAX: usize    = 24;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct PlayerEntity {
    sick: usize,
}

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
    let tile_size: Vec2 = Vec2::new(64.0, 64.0);
    let hframes: usize = 12;
    let vframes: usize = START_ANGLE_MAX;
    let layout = TextureAtlasLayout::from_grid(tile_size, hframes, vframes, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices {
        first: 0,
        last: 7
    };

    commands.spawn(
        (
            PlayerEntity {
                sick: 0,
            }, 
            SpriteSheetBundle {
                texture: player_texture,
                transform: Transform {
                    // scale: Vec3 {
                    //     x: tile_size.x,
                    //     y: tile_size.y,
                    //     ..Default::default()
                    // },
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
                sprint_multiplier: 2.0,
                direction: Vec2 {
                    x: 0f32,
                    y: -1f32,
                },
                ..Default::default()
            },
            AnimatedEntity {
                hframes: hframes,
                vframes: vframes,
                total_sprite_frames: hframes * vframes,
                timer: Timer::new(Duration::from_secs_f32(0.0416667f32), TimerMode::Repeating),
                ..Default::default()
            },
        )
    );
}

fn change_player_velocity (
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    action_map: Res<ActionMap>, 
    mut player_group: Query<(&mut CharacterEntity, &mut AnimatedEntity), With<PlayerEntity>>
)
{
    let mut input_count: u8 = 0;
    let mut velocity: Vec2 = Vec2 {
        x: 0.0f32,
        y: 0.0f32
    };

    let mut sprint_multiplier: f32 = 1.0;
    let mut is_sprinting: bool = false;
    if keyboard_input.pressed(action_map.get_key(KeyAction::Sprint)) {
        for (player, _animated_entity) in &player_group {
            sprint_multiplier = player.sprint_multiplier;
            is_sprinting = true;
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
        for (mut player, mut animated_entity) in &mut player_group {
            player.velocity = velocity;
            animated_entity.curr_start_angle = START_ANGLE_IDLE;
        }
        return;
    }

    for (mut player, mut animated_entity) in &mut player_group {
        player.direction = velocity.normalize_or_zero();
        player.velocity = player.direction * player.speed * sprint_multiplier;
        animated_entity.curr_start_angle = if !is_sprinting {START_ANGLE_WALK} else {START_ANGLE_SPRINT};
    }
}
