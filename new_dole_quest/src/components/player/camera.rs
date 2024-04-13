// use super::<super_trait>; // include stuff from module above this one
use bevy::prelude::*; use crate::components::CharacterEntity;
use super::PlayerEntity;

pub struct CameraPlugin;

#[derive(Component)]
pub struct CameraEntity;

// implementations for types -- such as CameraPlugin
// Trait implementation implementing the Plugin trait for CameraPlugin
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);

        app.add_systems(Update, move_camera_with_player);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(
        (
            CameraEntity,
            Camera2dBundle::default()
        ));
}

/**
    This function updates camera position with change from player's velocity
*/
fn move_camera_with_player (
    mut camera: Query<&mut Transform, With<CameraEntity>>,
    player_group: Query<&CharacterEntity, With<PlayerEntity>>,
    time: Res<Time>,
) {
    for mut transform in &mut camera {
        for player in &player_group {
            let change: Vec2 = player.velocity * time.delta_seconds();
            transform.translation.x += change.x;
            transform.translation.y += change.y;
        }
        //println!("{:?}", transform.translation);
    }
}
