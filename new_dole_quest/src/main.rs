use actions::ActionMap;
use bevy::{prelude::*, window::WindowMode};
use bevy::window::WindowResolution;
use animation::unit_angle::UnitAnglesEights;

pub mod components;
pub mod actions;
pub mod animation;

fn main() {
    App::new().insert_resource(ActionMap::new())
        .insert_resource(UnitAnglesEights::new())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
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
    ))
        .add_plugins((components::ComponentsPlugin, animation::AnimationPlugin))
        .run();
}
