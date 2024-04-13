
use bevy::prelude::*;
use super::StaticEntity; // need this even in submodules
pub struct ObjectPlugin;

#[derive(Component)]
pub struct ObjectEntity;

impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_object);
    }
}

fn setup_object(mut commands: Commands, asset_server: Res<AssetServer>) {
    let object_texture: Handle<Image> = asset_server.load("open_source_assets/tilesets/tree.png");

    commands.spawn((
            StaticEntity::default(),
            SpriteSheetBundle {
                transform: Transform {
                    scale: Vec3 {
                        x: 2.0f32,
                        y: 2.0f32,
                        z: 2.0f32,
                    },
                    ..Default::default()
                },
                texture: object_texture,
                ..Default::default()
            },
        )
    );
}