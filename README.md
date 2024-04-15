# Monster Masher
## Story
* You have been kidnapped and brought to an island in which most likely you will die.
* Luckily, you find some weapons to fight off a horde of strange creatures.

## Gameplay
### Combat: Hold right click to aim and shoot your gun.
  * You are stuck in place while aiming your gun, but the game is reactive enough that this is not an issue.
  * A crosshair will show up, and you will shoot in that direction.
    * From the player to the crosshair.

### Combat: Health system
* When you are hit by an enemy, your "sick" meter increases.
* When it fills all the way up, you become infected and need to find a vaccine before you become one of the creatures.
* If you become a creature, you are stuck that way for the rest of the game.
  * The game will turn endless, and you can do whatever you want on the island.
  * You will be able to talk to the creatures that are still alive.
  * If you become a creature due to the last boss, then you will fly off the island and possibly spread the creature virus to the world.

# Table of contents
* Notes and documentation
    * [Hello World](./notes_and_documentation/00_hello_world/hello_world.md)
    * [Cargo](./notes_and_documentation/01_hello_cargo/hello_cargo.md)
    * [Guessing Game](./notes_and_documentation/02_guessing_game/guessing_game.md)
    * [Bevy Build Notes](./notes_and_documentation/03_bevy_build_notes/bevy_build_notes.md)
    * [Bevy Intro Notes](./notes_and_documentation/04_bevy_intro_notes/bevy_intro_notes.md)
        * [Bevy Plugin Notes](./notes_and_documentation/04_bevy_intro_notes/bevy_plugin_notes.md)
        * [Bevy Resource Notes](./notes_and_documentation/04_bevy_intro_notes/bevy_resource_notes.md)
    * More Rust Language Concepts
      * [Variables and Mutability](./notes_and_documentation/05_ownership_and_programming_concepts/00_vars_mutability.md)
      * [Basic Functions](./notes_and_documentation/05_ownership_and_programming_concepts/01_basic_functions.md)
      * [Conditionals](./notes_and_documentation/05_ownership_and_programming_concepts/02_conditionals.md)
    * [Packages, Modules, and Crtates](./notes_and_documentation/06_modules_packages_crates/packages_crates.md)
      * [Modules to Control Scope and Privacy](./notes_and_documentation/06_modules_packages_crates/modules_to_control_scope.md)

# HUGE NOTES
## lints in Cargo.toml files to ignore stupid warnings
* This will allow using your own naming conventions and not having your hand forced by rust's stupid policies:
```toml
[lints.rust]
non_snake_case = { level = "allow" }
```
* NOTE: This does not mean that VSCode will actually highlight camel case variables :(

## open documentation for dependencies
```shell
cargo doc --open
```