#
# [Back](./../../README.md)

## Crate
* smallest amount of code compiler will consider

### Binary Crate
* Add more than just the `src/main.rs` binary crate by adding the `src/bin` directory.
* Programs you can compile to exe
* You can run these.
    * Like command line or server
* Must have main() function
    * All the crates with a main function are binary???

### Library Crate
* Defined as `lib.rs`
    * Will contain library crate with the same name as the [package](#package) it is in.
    * Located in `src/` directory at `src/lib.rs`.
    * Crate root if it exists.
    * Cargo passes crate root files to rustc to build the libary or binary.
* Do *NOT* have a main() function
* Don't compile to exe
* Intended to be shared with multiple projects.
* Crates are used interchangeably with libraries.
    * Too many people deep into Rust, they will refer to it as both.
    * Although binary and library crates exist.

### Crate Root
* Source file that rust compiler starts from.

### Package
* Bundle of crates (1+).
* Package contains Cargo.toml
    * describes how to build those crates.
* Contains:
    * Library Crate that the Binary crate depends on.
    * Only (at most) one library crate.
    * Arbitrary amount of binary crates.
    * At least one binary/library crate.
        * Either/or

```
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```
`cargo new` creates a package for us.


#
# [Back](./../../README.md)