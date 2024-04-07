# Table of contents
* [Hello World](./00_hello_world/hello_world.md)
* [Cargo](./01_hello_cargo/hello_cargo.md)
* [Guessing Game](./02_guessing_game/guessing_game.md)

# HUGE NOTES
## lints in Cargo.toml files to ignore stupid warnings
* This will allow using your own naming conventions and not having your hand forced by rust's stupid policies:
```toml
[lints.rust]
non_snake_case = { level = "allow" }
```
* NOTE: This does not mean that VSCode will actually highlight camel case variables :(