use bevy::prelude::*; // brings in essential things from bevy

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Dog;

// Instead of creating Person.name, it's better to make Name its own component
// since other Entities will probably have names as well!
#[derive(Component)]
struct Name(String);

fn main() {
    // keeps track of world, schedule, and runner
    App::new()
        .add_systems(Startup, (add_people, add_dogs)) // Startup systems run once before all other systems
        .add_systems(Update, (hello_world, greet_people, greet_dogs))
        .run(); // by itself - empty shell capable of app logic
}

// Startup systems - such as adding people
fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Ayuss Hoal".to_string())));
    commands.spawn((Person, Name("Cha Hoad".to_string())));
    commands.spawn((Person, Name("Neep Poal".to_string())));
}

fn add_dogs(mut commands: Commands) {
    commands.spawn((Dog, Name("Buckley".to_string())));
    commands.spawn((Dog, Name("Billy".to_string())));
}

/**
 * Query -- in this case, iterate over every component following the pattern given
 * Query<&Name, With Person> -- iterate over every Name component that also has a Person component
 * */
fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        // name.0 -- Grabbing the first parameter grabbed from the constructor
        //           from Name() struct
        // This so happens to be a String type
        println!("Fuck you, {}!", name.0);
    }
}

fn greet_dogs(query: Query<&Name, With<Dog>>) {
    for name in &query {
        println!("hewwoah {} doggo!", name.0);
    }
}

fn hello_world() {
    println!("Fuck Shit!");
}
