// use super::<super_trait>; // include stuff from module above this one

use bevy::prelude::*; // need this even in submodules

pub mod camera;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct PlayerEntity;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player);
    }
}

fn setup_player(mut commands: Commands) {
    commands.spawn(PlayerEntity);
}