use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
//use std::vec::Vec;

use super::map_material::MapMaterial;

pub struct MapBuilderPlugin;

pub struct MapBuilderEntity {
    pub vec_materials: Vec<MapMaterial>,
    pub level_name: String,
}

impl MapBuilderEntity {
    /// TODO: Load in all of the textures based on the [header] from level name specified
    pub fn new(level_name: &str) -> Self {
        let mut vec_materials: Vec<MapMaterial> = Vec::new();

        Self {
            vec_materials: vec_materials,
            level_name: level_name.to_string(),
        }
    }
}

impl Plugin for MapBuilderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, map_setup);
    }
}


// Spawn an entity using `CustomMaterial`.
fn map_setup(mut commands: Commands, mut materials: ResMut<Assets<MapMaterial>>, asset_server: Res<AssetServer>) {
    commands.spawn(MaterialMesh2dBundle {
        material: materials.add(MapMaterial {
            color: Color::RED,
            color_texture: asset_server.load("some_image.png"),
        }),
        ..Default::default()
    });
}
