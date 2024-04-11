use bevy::prelude::*;

use player::PlayerPlugin; // need this even in submodules
use player::camera::CameraPlugin;

pub mod player;

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CameraPlugin, PlayerPlugin));
    }
}