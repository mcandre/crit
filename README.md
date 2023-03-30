# crit: Rust cross-compiler

```text
             .__  __
  ___________|__|/  |_
_/ ___\_  __ \  \   __\
\  \___|  | \/  ||  |
 \___  >__|  |__||__|
     \/
```

# CAUTION

Work in progress.

# EXAMPLE

```console
$ cd example

$ crit

$ ls .crit
aarch64-apple-darwin
aarch64-linux-android
aarch64-unknown-linux-gnu
...
```

By default, crit builds in release mode (`-- -r`).

See `crit -h` for more options.

# CRATE

https://crates.io/crates/crit

# DOWNLOAD

https://github.com/mcandre/crit/releases

# INSTALL FROM SOURCE

```console
$ cargo install --path .
```

# LICENSE

FreeBSD

# RUNTIME REQUIREMENTS

* [rustup](https://rustup.rs/) 1.25.2+
* [Rust](https://www.rust-lang.org/en-US/) 1.64+
* [cross](https://crates.io/crates/cross) 0.2.5+
* [Docker](https://www.docker.com/) 20.10.23+

## Recommended

* [Docker First Aid Kit](https://github.com/mcandre/docker-first-aid-kit)
* a host capable of running musl/Linux containers (e.g. a GNU/Linux, musl/Linux, macOS, or Windows host)
* [ASDF](https://asdf-vm.com/) 0.10 (run `asdf reshim` after each Rust application binary installation)
* [direnv](https://direnv.net/) 2
* [cargo-cache](https://crates.io/crates/cargo-cache)
* [tree](https://en.wikipedia.org/wiki/Tree_(command))
* GNU compatible [time](https://www.gnu.org/software/time/)
* [Amphetamine](https://apps.apple.com/us/app/amphetamine/id937984704?mt=12) (macOS), [The Caffeine](https://www.microsoft.com/store/productId/9PJBW5SCH9LC) (Windows), [Caffeine](https://launchpad.net/caffeine) (Linux) can prevent hibernation during any long builds

# CONTRIBUTING

For more details on developing crit itself, see [DEVELOPMENT.md](DEVELOPMENT.md).

# CREDITS

* [cross](https://github.com/cross-rs/cross) underlying cross-compiler system
* [cross-toolchains](https://github.com/cross-rs/cross-toolchains) provisions cross Docker images
* [cubejs/rust-cross](https://hub.docker.com/r/cubejs/rust-cross/tags) Docker images for additional cross targets
* [factorio](https://github.com/mcandre/factorio) generates Go application ports based on the standard Go toolchain
* [snek](https://github.com/mcandre/snek) ports native C/C++ applications.
* [tug](https://github.com/mcandre/tug) automates multi-platform Docker image builds
* [WASM](https://webassembly.org/) provides a portable interface for C/C++ code.
* [xgo](https://github.com/crazy-max/xgo) supports Go projects with native cgo dependencies.
