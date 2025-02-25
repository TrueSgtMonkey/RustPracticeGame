use bevy::prelude::*;

use super::collider::BoxCollider;

pub struct CharacterPlugin;

#[derive(Component)]
pub struct CharacterEntity {
    pub direction: Vec2,
    pub velocity: Vec2,
    pub speed: f32,
    pub sprint_multiplier: f32,
    pub width: f32,
    pub height: f32,
    pub position: Vec2,
    pub radius: f32, // for sphere collisions
}

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_characters);
    }
}

impl Default for CharacterEntity {
    fn default() -> Self {
        let mut character = Self {
            direction: Vec2 {
                ..Default::default()
            },
            velocity: Vec2 {
                ..Default::default()
            },
            speed: 1.0f32,
            sprint_multiplier: 2.0f32,
            width: 32.0f32,
            height: 32.0f32,
            position: Vec2::new(0f32, 0f32),
            radius: 0f32,
        };

        let width_squared = character.width * character.width;
        let height_squared = character.height * character.height;
        character.radius = f32::sqrt(width_squared + height_squared);
        println!("{:?}", character.radius);

        return character;
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
    box_colliders: Query<&BoxCollider>,
    time: Res<Time>
) 
{
    for (mut character, mut transform) in &mut characters {
        let mut change: Vec2 = character.velocity * time.delta_seconds();
        for box_collider in &box_colliders {
            //println!("player: {:?} ; collision: {:?}", character.position, collider.position);
            if box_collider.is_colliding(&character.position, character.width, character.height) {
                //collider.collision_response(&mut change, &character.position, character.width, character.height);
                box_collider.gigi_collsison_response(&mut change, &character.position, character.width, character.height);
            }
        }
        
        transform.translation.x += change.x;
        transform.translation.y += change.y;
        character.position.x = transform.translation.x;
        character.position.y = transform.translation.y;
    }
}
