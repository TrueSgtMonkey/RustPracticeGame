use bevy::prelude::*; // brings in essential things from bevy

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people)
            .add_systems(Update, (hello_world, (update_people, greet_people).chain()));
    }
}

fn main() {
    App::new()
        // DefaultPlugins contains WindowPlugin and WininitPlugin
        // WindowPlugin defines window interface, and WininitPlugin uses winit
        // library to create window using OS
        .add_plugins((DefaultPlugins, HelloPlugin))
        .run();
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Ayuss Hoal".to_string())));
    commands.spawn((Person, Name("Cha Hoad".to_string())));
    commands.spawn((Person, Name("Neep Poal".to_string())));
}

/** 
 * update_people -- make sure it is run before add_people
 * */
fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Cha Hoad" {
            name.0 = "Cha Boner".to_string();
            break; // We don’t need to change any other names
        }
    }
}

/**
 * Query -- in this case, iterate over every component following the pattern given
 * Query<&Name, With Person> -- iterate over every Name component that also has a Person component
 * */
fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Fuck you, {}!", name.0);
    }
}

fn hello_world() {
    println!("Fuck Shit!");
}
