# crit: Rust cross-compiler

[![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/crit?label=crate%20downloads)](https://crates.io/crates/crit) [![Docker Pulls](https://img.shields.io/docker/pulls/n4jm4/crit)](https://hub.docker.com/r/n4jm4/crit) [![GitHub Downloads](https://img.shields.io/github/downloads/mcandre/crit/total?logo=github)](https://github.com/mcandre/crit/releases) [![Test](https://github.com/mcandre/crit/actions/workflows/test.yml/badge.svg)](https://github.com/mcandre/crit/actions/workflows/test.yml) [![license](https://img.shields.io/badge/license-BSD-3)](LICENSE.md) [![Donate](https://img.shields.io/badge/GUMROAD-36a9ae?style=flat&logo=gumroad&logoColor=white)](https://mcandre.gumroad.com/)

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

# DOWNLOAD

## Install

```sh
curl -L https://raw.githubusercontent.com/mcandre/crit/refs/heads/main/install-crit | sh
```

## Postinstall

Ensure `$HOME/.local/bin` is registered with your shell's `PATH` environment variable.

## Uninstall

```sh
curl -L https://raw.githubusercontent.com/mcandre/crit/refs/heads/main/uninstall-crit | sh
```

## System Requirements

Supported host environments:

* FreeBSD (x86_64)
* macOS (aarch64 / x86_64)
* NetBSD (x86_64)
* Linux (aarch64 / x86_64)
* Illumos (x86_64)
* Windows (aarch64 / x86_64) via [WSL](https://learn.microsoft.com/en-us/windows/wsl/)

Prerequisites:

* [curl](https://curl.se/)

For more installation methods, see our [install guide](INSTALL.md).

# RUNTIME REQUIREMENTS

* [Docker](https://www.docker.com/) 28.0.1+
* [Rust](https://www.rust-lang.org/en-US/) 1.92.0+
* [cross](https://github.com/cross-rs/cross) at ref `4e64366af6095c84fa4f54a0fa5a2ba7d9a271aa`

```sh
cargo install --force cross --git https://github.com/cross-rs/cross --rev 4e64366af6095c84fa4f54a0fa5a2ba7d9a271aa
```

## Recommended

* a host capable of running musl/Linux containers (e.g. a GNU/Linux, musl/Linux, macOS, or Windows host)
* a UNIX-like environment (e.g. [WSL](https://learn.microsoft.com/en-us/windows/wsl/))
* [Docker First Aid Kit](https://github.com/mcandre/docker-first-aid-kit)
* 200 GB of disk space allocated to Docker
* Apply `DOCKER_DEFAULT_PLATFORM` = `linux/amd64` environment variable
* [cargo-cache](https://crates.io/crates/cargo-cache)
* [tree](https://en.wikipedia.org/wiki/Tree_(command))
* [GNU](https://www.gnu.org/) [time](https://en.wikipedia.org/wiki/Time_(Unix))
* [Amphetamine](https://apps.apple.com/us/app/amphetamine/id937984704?mt=12) (macOS), [The Caffeine](https://www.microsoft.com/store/productId/9PJBW5SCH9LC) (Windows), [Caffeine](https://launchpad.net/caffeine) (Linux) can prevent hibernation during long builds

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

crit specializes in crosscompiling Rust applications for multiple platforms.

In addition to crit, we cite related resources, including prior art, personal plugs, and tools for developing portable applications (including non-Rust projects)!

Resources:

* [cross](https://github.com/cross-rs/cross) provides the underlying cross-compiler system that powers crit.
* [cross-toolchains](https://github.com/cross-rs/cross-toolchains) provisions cross Docker images.
* [mcandre/chandler](https://github.com/mcandre/chandler) normalizes executable archives.
* [mcandre/factorio](https://github.com/mcandre/factorio) automates crossplatform ports for Go projects.
* [mcandre/rockhopper](https://github.com/mcandre/rockhopper) generates install packages.
* [mcandre/tuggy](https://github.com/mcandre/tuggy) automates multiplatform Docker image builds.
* [mcandre/unmake](https://github.com/mcandre/unmake) detects quirks in makefiles.
* [WASM](https://webassembly.org/) provides a portable interface for C/C++ code.
* [xgo](https://github.com/techknowlogick/xgo) compiles ports for cGo projects.
