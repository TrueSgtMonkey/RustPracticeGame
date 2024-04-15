use bevy::prelude::*;

pub struct CharacterPlugin;

#[derive(Component)]
pub struct CharacterEntity {
    pub direction: Vec2,
    pub velocity: Vec2,
    pub speed: f32,
    pub sprint_multiplier: f32,
}

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_characters);
    }
}

impl Default for CharacterEntity {
    fn default() -> Self {
        Self {
            direction: Vec2 {
                ..Default::default()
            },
            velocity: Vec2 {
                ..Default::default()
            },
            speed: 1.0f32,
            sprint_multiplier: 2.0f32,
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
