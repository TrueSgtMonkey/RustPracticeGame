# Monster Masher
## Story
* Will be a pokemon-like game with a "serious" (ridiculous) story
* bloody - you are trapped on a hell-scape of a land with creatures that
  can easily kill you.
* Unfortunately, you can do little to fight against them, so you must
  capture monsters and use them to fight other monsters/people.
* Will have option to spare the other people you come across?
* If all your monsters die, the monster will kill you.

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