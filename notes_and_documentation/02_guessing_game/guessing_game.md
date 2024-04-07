#
# [Back](./../notes.md)

```rs
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)           // Guessing that multiline calls are normal convention?
        .expect("Failed to read line!"); // Cool that we can put our own exception?

    // We can use this format to put variables in strings?
    println!("You guessed: {guess}"); // Guessing that this prints out "guess" variable?

    let test = "Testing: {guess}".to_string();
    println!("{test}");
```
* While we can use {guess} in prints to output the value of the variable inside a string, we cannot use that same notation in normal strings.

In order to add a rand library, we have to add a dependency manually:
```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = "0.8.5" # here is our dependency

[lints.rust]
non_snake_case = { level = "allow" }
```

Then, we build the project with:
```
cargo build
```

Grabbed from the crates in here:

[Crates.io](https://crates.io/)

Using this command will open a browser tab with documentation for all dependencies.
```
cargo doc --open
```

#
# [Back](./../notes.md)