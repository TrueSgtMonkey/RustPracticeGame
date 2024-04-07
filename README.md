# Table of contents
* Notes and documentation
    * [Hello World](./notes_and_documentation/00_hello_world/hello_world.md)
    * [Cargo](./notes_and_documentation/01_hello_cargo/hello_cargo.md)
    * [Guessing Game](./notes_and_documentation/02_guessing_game/guessing_game.md)
    * [Bevy Build Notes](./notes_and_documentation/03_bevy_build_notes/bevy_build_notes.md)
    * [Bevy Intro Notes](./notes_and_documentation/04_bevy_intro_notes/bevy_intro_notes.md)

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