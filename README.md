# rocky: Rust cross-compiler

# CAUTION

Work in progress.

# EXAMPLE

```console
$ cd example

$ rocky

$ ls .rocky
aarch64-apple-darwin
aarch64-linux-android
aarch64-unknown-linux-gnu
...
```

# CRATE

https://crates.io/crates/rocky

# API DOCUMENTATION

https://docs.rs/rocky/

# RUNTIME REQUIREMENTS

* [Rust](https://www.rust-lang.org/en-US/) 1.64+
* [cross](https://crates.io/crates/cross) 0.2.5+
* [Docker](https://www.docker.com/) 20.10.23+

## Recommended

* ~100 GB disk space, for cross Docker images
* a host capable of running musl/Linux containers (e.g. a GNU/Linux, musl/Linux, macOS, or Windows host)
* [ASDF](https://asdf-vm.com/) 0.10 (run `asdf reshim` after each Rust application binary installation)
* [direnv](https://direnv.net/) 2
* [cargo-cache](https://crates.io/crates/cargo-cache)
* [tree](https://en.wikipedia.org/wiki/Tree_(command))

# CONTRIBUTING

For more details on developing tinyrick itself, see [DEVELOPMENT.md](DEVELOPMENT.md).

![red rock crab](rocky.png)

# CREDITS

* [cross](https://github.com/cross-rs/cross) underlying cross-compiler system
* [cross-toolchains](https://github.com/cross-rs/cross-toolchains) provisions cross Docker images
* [cubejs/rust-cross](https://hub.docker.com/r/cubejs/rust-cross/tags) Docker images for additional cross targets
* [factorio](https://github.com/mcandre/factorio) generates Go application ports based on the standard Go toolchain
* [freeznet](https://hub.docker.com/u/freeznet) Docker images for additional cross targets
* [tug](https://github.com/mcandre/tug) automates multi-platform Docker image builds
