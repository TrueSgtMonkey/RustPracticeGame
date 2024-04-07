#
# [Back](./../../README.md)

## Adding Bevy to the project
```
cargo add bevy
```
* Adds bevy to your project

Here is the result of this command in the toml file:
```toml
[package]
name = "new_dole_quest"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
bevy = "0.13.2"
```

Alternatively, you can do this:
```toml
[package]
name = "my_bevy_game"
version = "0.1.0"
edition = "2021" # this needs to be 2021, or you need to set "resolver=2"

[dependencies]
bevy = "0.13" # make sure this is the latest version
```

## Performance builds
Add these to the Cargo.toml file to keep fast iterative compiles and have playable framerates:
```toml
# Enable a small amount of optimization in debug mode
[profile.dev]
opt-level = 1

# Enable high optimizations for dependencies (incl. Bevy), but not for our code:
[profile.dev.package."*"]
opt-level = 3
```
* You do *not* want to develop with Release builds since:
```
You might think to simply develop in release mode instead, but we recommend against this as it can worsen the development experience by slowing down recompiles and disabling helpful debug symbols and assertions.
```

## Dynamic Linking
```shell
cargo run --features bevy/dynamic_linking
```
* This needs to be run with performance optimizations on Windows

Alternatively, this can be enabled in Cargo.toml:
```toml
[dependencies]
bevy = { version = "0.13.0", features = ["dynamic_linking"] }
```
* DISABLE THIS FOR RELEASE!!!

## LLD Linker
```shell
cargo install -f cargo-binutils
rustup component add llvm-tools-preview
```
* Install the LLD linker by running this command
* LLD is much **faster** at linking than the default Rust linker.

## nightly rust compiler
```toml
[toolchain]
channel = "nightly"
```
* Add this to a `rust-toolchain.toml` file

#
# [Back](./../../README.md)