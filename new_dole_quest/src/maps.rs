use bevy::prelude::*;

use self::map_builder::MapBuilderPlugin; // need this even in submodules

pub mod map_builder;
pub mod map_material;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MapBuilderPlugin);
    }
}
