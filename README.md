# crit: Rust cross-compiler

```text
             .__  __
  ___________|__|/  |_
_/ ___\_  __ \  \   __\
\  \___|  | \/  ||  |
 \___  >__|  |__||__|
     \/
```

# SUMMARY

`crit` compiles Rust application ports for many different target platforms. This effort is based on conventional Rust tooling, including `cross`, `cargo`, and the amazing `rustc` compiler.

# EXAMPLE

```console
$ cd example

$ crit

$ ls .crit/bin
aarch64-apple-darwin
aarch64-unknown-linux-gnu
aarch64-unknown-linux-musl
...
```

By default, crit builds in release mode (`-- -r`).

See `crit -h` for more options.

# CRATE

https://crates.io/crates/crit

# API DOCUMENTATION

https://docs.rs/crit/

# DOWNLOAD

https://github.com/mcandre/crit/releases

# INSTALL FROM SOURCE

```console
$ cargo install --force --path .
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
* [tar](https://en.wikipedia.org/wiki/Tar_(computing)) / [zip](https://en.wikipedia.org/wiki/ZIP_(file_format))
* [tinyrick](https://github.com/mcandre/tinyrick) 0.0.9
* [tree](https://en.wikipedia.org/wiki/Tree_(command))
* GNU compatible [time](https://www.gnu.org/software/time/)
* [Amphetamine](https://apps.apple.com/us/app/amphetamine/id937984704?mt=12) (macOS), [The Caffeine](https://www.microsoft.com/store/productId/9PJBW5SCH9LC) (Windows), [Caffeine](https://launchpad.net/caffeine) (Linux) can prevent hibernation during any long builds

# CONTRIBUTING

For more details on developing crit itself, see [DEVELOPMENT.md](DEVELOPMENT.md).

# FAQ

## Help, some targets are broken?

Check that your project is able to build with conventional `cross` or `cargo` commands against a single target. A project that does not compile against a single target, will naturally have difficulty when attempting to cross-compile for multiple targets.

Note that Rust introduces new, under-supported targets all the time. We try to keep up, but sometimes we miss a few of these. Regardless, you can declare which targets are disabled, by writing a custom pattern for the `-e` / `--exclude-targets` flag.

Some targets may lack stock support for the Rust `std` library. This is common for bare metal or embedded targets. For these kinds of targets, you have several strategies for resolution:

* Provide a `std` implementation. Reach out to specialists for the specific target involved.
* Avoid using the `std` library, in both your code, as well as the dependency tree. This is actually common practice for many Rust projects, as an proactive stance on embedded development support.
* Disable undesired targets.

## Help, cross-compilation appears frozen?

crit hides a lot of compiler noise. While a target is building, you can use common Docker commands to inspect the compilation process:

* `docker ps -a`
* `docker logs [--follow] <container id>`

## Help, cross-compilation is slow?

Yes, it sure is! Almost as slow as using Virtual Machines for cross-compilation.

Rustaceans come to expect that the Rust compiler is analytical, spending more time optimizing programs, so that the final binaries will run safer and faster. The Rust compiler often taking a long time to compile each individual target.

Naturally, when cross-compiling multiple targets, that time multiplies by the number of targets.

Some cross-compilation performance tips:

* Tune your Docker setup (see the Docker First Aid Kit above)
* Reset common Cargo build profile options (`codegen-units`, `lto`, `strip`, etc.)
* Use debug mode (e.g., `--`)
* Use fewer dependencies
* Design with the [UNIX Philosophy](https://en.wikipedia.org/wiki/Unix_philosophy), namely *Make each program do one thing well.* Not a hundred features poorly.
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
