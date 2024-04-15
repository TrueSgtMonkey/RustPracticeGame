use bevy::prelude::*;

use sprite_animation::AnimatedEntity; // need this even in submodules
use crate::components::character::CharacterEntity;
use unit_angle::UnitAnglesEights;

pub mod sprite_animation;

pub struct AnimationPlugin;

pub mod unit_angle;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_characters);
    }
}

fn animate_characters(
    mut animated_character_entities: Query<(&CharacterEntity, &mut AnimatedEntity, &mut TextureAtlas), With<AnimatedEntity>>,
    unit_angle: Res<UnitAnglesEights>,
    time: Res<Time>,
)
{
    for (character, mut animated_entity, mut texture_atlas) in &mut animated_character_entities {
        // need to play animations at a fixed rate
        animated_entity.timer.tick(time.delta());
        if !animated_entity.timer.finished() {
            return;
        }

        let angle: usize = unit_angle.get_numeric_direction_angle(&character.direction);
        let st_angle: usize = animated_entity.curr_start_angle;
        let frame_increase: isize = animated_entity.curr_frame_increase;
        let angle_increase: usize = animated_entity.angle_increase;
        animated_entity.iso_animation(angle, st_angle, frame_increase, angle_increase);
        texture_atlas.index = animated_entity.frame_num;
    }
}
