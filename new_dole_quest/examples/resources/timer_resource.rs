use bevy::prelude::*; // brings in essential things from bevy

#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // better that we update the people in the setup rather than per frame
        // only need to rename once.
        app.insert_resource(GreetTimer(Timer::from_seconds(0.8, TimerMode::Repeating)))
            .add_systems(Startup, (add_people, update_people).chain())
            .add_systems(Update, greet_people);
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        .run();
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Ayuss Hoal".to_string())));
    commands.spawn((Person, Name("Cha Hoad".to_string())));
    commands.spawn((Person, Name("Neep Poal".to_string())));
}

/** 
 * update_people -- make sure it is run before greet_people and after add_people
 */
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
 * Res Time -- The Time resource gathered from DefaultPlugins
 * ResMut   -- This is a Mutable Resource of type GreetTimer?
 */
 fn greet_people(
    time: Res<Time>,                   // needed for time.delta
    mut timer: ResMut<GreetTimer>,     // needed for tick method
    query: Query<&Name, With<Person>>  // needed for Name query
    )
{
    // only run the query if the timer has finished
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    for name in &query {
        println!("Fuck you, {}!", name.0);
    }
}
