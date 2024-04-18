use actions::ActionMap;
use bevy::sprite::Material2dPlugin;
use bevy::{prelude::*, window::WindowMode};
use bevy::window::WindowResolution;
use animation::unit_angle::UnitAnglesEights;
use maps::map_material::MapMaterial;

pub mod components;
pub mod actions;
pub mod animation;
pub mod maps;
pub mod utilities;

fn main() {
    App::new().insert_resource(ActionMap::new())
        .insert_resource(UnitAnglesEights::new())
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
        // TODO: Make most of these windows attributes non-static values
        primary_window: Some(Window {
            resolution: WindowResolution::new(800.0, 600.0),
            title: "Monster Masher".to_string(),
            position: WindowPosition::At(IVec2::new(200, 150)),
            resizable: false,
            resize_constraints: WindowResizeConstraints {
                min_width: 800.0,
                max_width: 800.0,
                min_height: 600.0,
                max_height: 600.0,
            },
            mode: WindowMode::Windowed,
            ..default()
        }),
        ..default()
    }).set(
        ImagePlugin::default_nearest()
    ), Material2dPlugin::<MapMaterial>::default()))
        .init_asset::<MapMaterial>() // custom initialized resources need to be here
        .add_plugins((components::ComponentsPlugin, animation::AnimationPlugin, maps::MapPlugin))
        .run();
}
