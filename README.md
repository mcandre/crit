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
* [Rust](https://www.rust-lang.org/en-US/) 1.68.2+
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

# FAQ

## Help, some targets are broken?

First, check that your project is able to build with conventional `cross` or `cargo` commands. A project which does not compile, will naturally have difficulty cross-compiling.

Note that Rust introduces new, under-supported targets all the time. We try to keep up, but sometimes we miss a few of these. Regardless, you can declare which targets are disabled, by writing a custom pattern for the `-e` / `--exclude-targets` flag.

Some targets may lack stock support for the Rust `std` library. This is common for bare metal or embedded targets. For these kinds of targets, you have several strategies for resolution:

* Provide a `std` implementation. Reach out to specialists for the specific target involved.
* Avoid using the `std` library, in both your code, as well as the dependency tree. This is actually common practice for many Rust projects, as an proactive stance on embedded development support.
* Disable undesired targets.

## Help, cross-compilation is slow?

Yes, it is. The Rust compiler is notoriously analytical, often taking a long time to compile a single target. When cross-compiling multiple targets, that time multiplies by the number of targets.

Some cross-compilation performance tips:

* Tune your Docker setup (see the Docker First Aid Kit above)
* Temporarily disable common Cargo build profile options (`codegen-units`, `lto`, `strip`, etc.)
* Use debug mode (e.g., `--`)
* Use fewer dependencies
* Keep the host awake (see Amphetamine / The Caffeine / Caffeine above)
* Reserve cross-compilation as a release-time step, distinct from more rapid development tasks
* Perform cross-compilation in a CI/CD pipeline with more CPU, disk, and RAM resources
* Exclude more targets (e.g., `-e <target pattern>`)

# CREDITS

* [cross](https://github.com/cross-rs/cross) underlying cross-compiler system
* [cross-toolchains](https://github.com/cross-rs/cross-toolchains) provisions cross Docker images
* [cubejs/rust-cross](https://hub.docker.com/r/cubejs/rust-cross/tags) Docker images for additional cross targets
* [factorio](https://github.com/mcandre/factorio) generates Go application ports based on the standard Go toolchain
* [snek](https://github.com/mcandre/snek) ports native C/C++ applications.
* [tug](https://github.com/mcandre/tug) automates multi-platform Docker image builds
* [WASM](https://webassembly.org/) provides a portable interface for C/C++ code.
* [xgo](https://github.com/crazy-max/xgo) supports Go projects with native cgo dependencies.
