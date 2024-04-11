use actions::ActionMap;
use bevy::{prelude::*, window::WindowMode};
use bevy::window::WindowResolution;

pub mod components;
pub mod actions;

// TODO: Remove -- just for reference
// #[derive(Resource)]
// struct GreetTimer(Timer);

fn main() {
    App::new().insert_resource(ActionMap::new())
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
        .add_plugins(components::ComponentsPlugin)
        .run();
}

// TODO: Remove -- just for reference
// pub fn has_user_input(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     mouse_button_input: Res<ButtonInput<MouseButton>>,
//     touch_input: Res<Touches>,
// ) -> bool {
//     keyboard_input.just_pressed(KeyCode::Space)
//         || mouse_button_input.just_pressed(MouseButton::Left)
//         || touch_input.any_just_pressed()
// }