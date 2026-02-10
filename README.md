# crit: Rust cross-compiler

[![Docker Pulls](https://img.shields.io/docker/pulls/n4jm4/crit)](https://hub.docker.com/r/n4jm4/crit) [![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/crit?label=crate%20downloads&labelColor=grey&color=green)](https://crates.io/crates/crit) [![Test](https://github.com/mcandre/crit/actions/workflows/test.yml/badge.svg)](https://github.com/mcandre/crit/actions/workflows/test.yml) [![Test-Futureproof-Dependencies](https://github.com/mcandre/crit/actions/workflows/test-futureproof-dependencies.yml/badge.svg)](https://github.com/mcandre/crit/actions/workflows/test-futureproof-dependencies.yml) [![Test-Futureproof-Language](https://github.com/mcandre/crit/actions/workflows/test-futureproof-language.yml/badge.svg)](https://github.com/mcandre/crit/actions/workflows/test-futureproof-language.yml) [![Test-Futureproof-OS](https://github.com/mcandre/crit/actions/workflows/test-futureproof-os.yml/badge.svg)](https://github.com/mcandre/crit/actions/workflows/test-futureproof-os.yml) [![license](https://img.shields.io/badge/license-BSD-3)](LICENSE.md) [![Donate](https://img.shields.io/badge/GUMROAD-36a9ae?style=flat&logo=gumroad&logoColor=white)](https://mcandre.gumroad.com/)

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
aarch64-pc-windows-msvc
aarch64-unknown-linux-gnu
...
```

See `crit -h` for more options.

# INSTALLATION

See [INSTALL.md](INSTALL.md).

# RUNTIME REQUIREMENTS

* [rustup](https://rustup.rs/) 1.28.1+
* [Rust](https://www.rust-lang.org/en-US/)
* [cross](https://crates.io/crates/cross) 4e64366af6095c84fa4f54a0fa5a2ba7d9a271aa
* [Docker](https://www.docker.com/) 28.0.1+

## Recommended

* a host capable of running musl/Linux containers (e.g. a GNU/Linux, musl/Linux, macOS, or Windows host)
* a UNIX-like environment (e.g. [WSL](https://learn.microsoft.com/en-us/windows/wsl/))
* [Docker First Aid Kit](https://github.com/mcandre/docker-first-aid-kit)
* 256 GB of space allocated to Docker
* Apply `DOCKER_DEFAULT_PLATFORM` = `linux/amd64` environment variable
* [ASDF](https://asdf-vm.com/) 0.18 (run `asdf reshim` after each Rust application binary installation)
* [cargo-cache](https://crates.io/crates/cargo-cache)
* [direnv](https://direnv.net/) 2
* POSIX compliant [tar](https://pubs.opengroup.org/onlinepubs/7908799/xcu/tar.html)
* [tinyrick](https://github.com/mcandre/tinyrick)
* [tree](https://en.wikipedia.org/wiki/Tree_(command))
* [GNU](https://www.gnu.org/) [time](https://en.wikipedia.org/wiki/Time_(Unix))
* [Amphetamine](https://apps.apple.com/us/app/amphetamine/id937984704?mt=12) (macOS), [The Caffeine](https://www.microsoft.com/store/productId/9PJBW5SCH9LC) (Windows), [Caffeine](https://launchpad.net/caffeine) (Linux) can prevent hibernation during any long builds

**Warning**: Non-UNIX file systems may not preserve crucial chmod permissions during port generation. This can corrupt downstream artifacts, such as compressed archives and installation procedures.

tar is a portable archiver suitable for creating `*.tgz` tarball archives. Users can then download the tarball and extract the executable relevant to their platform. Tarballs are especially well suited for use in Docker containers, as the tar command is more likely to be installed than unzip.

# CONFIGURATION

See [CONFIGURATION.md](CONFIGURATION.md).

# FAQ

## Help, some targets are broken?

Check that your project is able to build with conventional `cross` or `cargo` commands against a single target. A project that does not compile against a single target, will naturally have difficulty when attempting to cross-compile for multiple targets.

Note that Rust introduces new, under-supported targets all the time. We try to keep up, but sometimes we miss a few of these. Regardless, you can declare which targets are disabled, by configuring custom patterns.

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
* Exclude more targets (e.g. 32 bit targets, GNU targets, or any targets with niche support)

# SEE ALSO

* [chandler](https://github.com/mcandre/chandler) normalizes executable archives
* [cross](https://github.com/cross-rs/cross) underlying cross-compiler system
* [cross-toolchains](https://github.com/cross-rs/cross-toolchains) provisions cross Docker images
* [cubejs/rust-cross](https://hub.docker.com/r/cubejs/rust-cross/tags) Docker images for additional cross targets
* [factorio](https://github.com/mcandre/factorio) generates Go application ports based on the standard Go toolchain
* [rockhopper](https://github.com/mcandre/rockhopper) generates packages for many operating systems
* [tuggy](https://github.com/mcandre/tuggy) automates multiplatform Docker image builds
* [unmake](https://github.com/mcandre/unmake), a linter for makefiles
* [WASM](https://webassembly.org/) provides a portable interface for C/C++ code.
* [xgo](https://github.com/techknowlogick/xgo) supports Go projects with native cgo dependencies.
