// use super::<super_trait>; // include stuff from module above this one

use bevy::prelude::*; // need this even in submodules

pub struct CameraPlugin;

#[derive(Component)]
pub struct CameraEntity;

// implementations for types -- such as CameraPlugin
// Trait implementation implementing the Plugin trait for CameraPlugin
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
