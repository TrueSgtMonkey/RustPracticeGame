use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
//use std::vec::Vec;

use super::map_material::MapMaterial;

pub mod parse_map;

const GROUP_MIN: usize = 0;
const GROUP_MAX: usize = 1;

pub struct MapBuilderPlugin;

#[derive(Resource)]
pub struct MapBuilder {
    pub materials: Vec<String>,
    pub tile_groups: Vec<(usize, Vec2, Vec2)>,
    pub collision_groups: Vec<(Vec2, Vec2)>,
    pub level_name: String,
}

impl MapBuilder {
    /// TODO: Load in all of the textures based on the [header] from level name specified
    pub fn new(level_name: &str) -> Self {
        parse_map::parse_map(level_name)
    }
}

impl Plugin for MapBuilderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, map_pre_setup)
            .add_systems(Startup, map_setup);
    }
}

fn map_pre_setup(mut commands: Commands) {
    let map_builder = MapBuilder::new("./assets/maps/test_level.map2d");
    commands.insert_resource(map_builder);
}

// Spawn an entity using `CustomMaterial`.
fn map_setup(
    mut commands: Commands, 
    mut materials: ResMut<Assets<ColorMaterial>>, 
    map_builder: Res<MapBuilder>, 
    asset_server: Res<AssetServer>, 
    mut assets_meshes: ResMut<Assets<Mesh>>
)
{
    //fn map_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tile_groups: &Vec<(usize, Vec2, Vec2)> = &map_builder.tile_groups;
    let map_materials: &Vec<String> = &map_builder.materials;
    let collision_groups: &Vec<(Vec2, Vec2)> = &map_builder.collision_groups;

    for (id, min_vec, max_vec) in tile_groups {
        println!("{:?};{:?}", *min_vec, *max_vec);
        let dimensions: Vec2 = Vec2 {
            x: (max_vec.x - min_vec.x) * 32f32,
            y: (max_vec.y - min_vec.y) * 32f32,
        };
        let averages: Vec2 = Vec2 {
            x: (max_vec.x - min_vec.x) * 0.5f32,
            y: (max_vec.y - min_vec.y) * 0.5f32
        };
        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(assets_meshes.add(Rectangle::new(dimensions.x + 32.0f32, dimensions.y + 32.0f32))),
            // material: materials.add(MapMaterial {
            //     id: *id,
            //     color: Color::RED,
            //     texture: asset_server.load(&map_materials[*id]),
            // }),
            material: materials.add(Color::hsl(180f32 * (*id as f32), 0.95, 0.7)),
            transform: Transform {
                translation: Vec3::new((min_vec.x + averages.x) * 32.0f32, (min_vec.y + averages.y) * -32.0f32, -((*id + 1) as f32)),
                ..Default::default()
            },
            ..Default::default()
        });
    }
    // commands.spawn(MaterialMesh2dBundle {
    //     material: materials.add(MapMaterial {
    //         color: Color::RED,
    //         color_texture: asset_server.load("textures/0000_blood_rock.png"),
    //     }),
    //     ..Default::default()
    // });
}
