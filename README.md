# Lexicoid

![Build Status](https://github.com/lmammino/lexicoid/actions/workflows/ci-build.yml/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/lexicoid.svg)](https://crates.io/crates/lexicoid)
[![docs.rs](https://docs.rs/lexicoid/badge.svg)](https://docs.rs/lexicoid)

Short & stable IDs based on timestamps.

Heavily inspired by [Short, friendly base32 slugs from timestamps](https://brandur.org/fragments/base32-slugs) by [@brandur](https://github.com/brandur).


## Install

Install with cargo by updating your `Cargo.toml`:

```toml
[dependencies]
lexicoid = "*"
```

or with `cargo-add` you can run:

```bash
cargo add lexicoid
```


## Usage

```rust
use lexicoid::*;

// generates a lexicoid for the current timestamp
println!("{}", lexicoid_now()) // gj7x3vc

// generates a lexicoid for a given unix timestamp (as u64)
println!("{}", lexicoid(1654401676)) // gei4p52
```

## Use cases

Whenever you need simple and short ids that are lexicographically sorted based on the generation timestamp.

Examples:

  - [@brandur](https://github.com/brandur), who inspired this crate, uses this to generate ids for [short entries on his website](https://brandur.org/atoms).
  - You want to obfuscate filenames in a folder and replace them with strings that are still preserving lexicographic sorting (based on file creation or modification time).
  - You need to prefix filenames with a unique id while keeping them sorted by creation (e.g. to manage your own blog posts, migration files or other types of files that you end up creating slowly over time).

> **Warning**: since the resolution of the timestamps is per second, if you try to generate multiple ids in the same second, they will all be equal. This is not directly suitable for high frequency ID generation. But, if you need to adapt this approach to high frequency ID generation, you can always append the current number of milliseconds (or nanoseconds) to the generated ID.


## Contributing

Everyone is very welcome to contribute to this project.
You can contribute just by submitting bugs or suggesting improvements by
[opening an issue on GitHub](https://github.com/lmammino/lexicoid/issues).


## License

Licensed under [MIT License](LICENSE). © Luciano Mammino.
