#
# [Back](./../../README.md)

## Plugins
* All bevy engine features implemented as plugins.
* Even the game we will be making.
* Pick the features you want, add the features you want to add.
    * Replace any components you do not like.
    * Write your own plugins.

## Using plugins
* Find/Creatre Plugin
* Add to `Cargo.toml`
    * As a crate under `[dependencies]`
* Import code definitions
    * `use third_party::prelude::*;`
* Add the plugin to your app
    * `app.add_plugins(third_party_plugin)`

## Default Plugins
* There are default plugins at this page: [DefaultPlugins](https://docs.rs/bevy/latest/bevy/struct.DefaultPlugins.html)

```rs
fn main() {
    App::new()
        // DefaultPlugins contains WindowPlugin and WininitPlugin
        // WindowPlugin defines window interface, and WininitPlugin uses winit
        // library to create window using OS
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, (update_people, greet_people).chain()))
        .run();
}
```
* On top of the comments listed above, DefaultPlugins also adds an `event loop` to our app.
* App's ECS schedule now runs in a loop once per frame.

#
# [Back](./../../README.md)