# crit: Rust cross-compiler

[![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/crit?label=crate%20downloads)](https://crates.io/crates/crit) [![GitHub Downloads](https://img.shields.io/github/downloads/mcandre/crit/total?logo=github)](https://github.com/mcandre/crit/releases) [![Test](https://github.com/mcandre/crit/actions/workflows/test.yml/badge.svg)](https://github.com/mcandre/crit/actions/workflows/test.yml) [![license](https://img.shields.io/badge/license-BSD-3)](LICENSE.md) [![Donate](https://img.shields.io/badge/%E2%99%A5-Sponsor-BF3988)](https://github.com/sponsors/mcandre)

```text
             .__  __
  ___________|__|/  |_
_/ ___\_  __ \  \   __\
\  \___|  | \/  ||  |
 \___  >__|  |__||__|
     \/
```

# SUMMARY

`crit` automates cross-compiling Rust applications for many different kinds of target platforms.

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

```sh
cargo install crit
```

# PREREQUISITES

* [cargo](https://doc.rust-lang.org/cargo/)
* [Docker](https://www.docker.com/)
* [cross](https://github.com/cross-rs/cross) at ref `4e64366af6095c84fa4f54a0fa5a2ba7d9a271aa`

```sh
cargo install --force cross --git https://github.com/cross-rs/cross --rev 4e64366af6095c84fa4f54a0fa5a2ba7d9a271aa
```

## Recommended

* a UNIX-like environment (e.g. [WSL](https://learn.microsoft.com/en-us/windows/wsl/))
* Apple Silicon macOS users may want to apply `DOCKER_DEFAULT_PLATFORM=linux/amd64`, in order to account for images commonly lacking `linux/arm64` buildx platforms
* [Amphetamine](https://apps.apple.com/us/app/amphetamine/id937984704?mt=12) (macOS), [The Caffeine](https://www.microsoft.com/store/productId/9PJBW5SCH9LC) (Windows), [Caffeine](https://launchpad.net/caffeine) (Linux) can prevent hibernation during long builds
* GNU [time](https://en.wikipedia.org/wiki/Time_(Unix))
* [tree](https://en.wikipedia.org/wiki/Tree_(command))

For more installation methods, see our [install guide](INSTALL.md).

# CONFIGURATION

For information on configuring crit, see our [configuration guide](CONFIGURATION.md).

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

# RESOURCES

Prior art, personal plugs, and tools for developing portable applications (including non-Rust projects)!

* [cross](https://github.com/cross-rs/cross) provides the underlying cross-compiler system that powers crit.
* [cross-toolchains](https://github.com/cross-rs/cross-toolchains) provisions cross Docker images.
* [mcandre/tuco](https://github.com/mcandre/tuco) automates crossplatform ports for Go projects.
* [mcandre/rockhopper](https://github.com/mcandre/rockhopper) generates install packages.
* [mcandre/unmake](https://github.com/mcandre/unmake) detects quirks in makefiles.
* [WASM](https://webassembly.org/) provides a portable interface for C/C++ code.
* [xgo](https://github.com/techknowlogick/xgo) compiles ports for cGo projects.
