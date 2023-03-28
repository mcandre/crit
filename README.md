# rocky: Rust cross-compiler

![red rock crab](rocky.png)

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

See `rocky -h` for more options.

# LICENSE

FreeBSD

# RUNTIME REQUIREMENTS

* [Rust](https://www.rust-lang.org/en-US/) 1.64+
* [cross](https://crates.io/crates/cross) 0.2.5+
* [Docker](https://www.docker.com/) 20.10.23+

## Recommended

* a host capable of running musl/Linux containers (e.g. a GNU/Linux, musl/Linux, macOS, or Windows host)
* [ASDF](https://asdf-vm.com/) 0.10 (run `asdf reshim` after each Rust application binary installation)
* [direnv](https://direnv.net/) 2
* [cargo-cache](https://crates.io/crates/cargo-cache)
* [tree](https://en.wikipedia.org/wiki/Tree_(command))
* GNU compatible [time](https://www.gnu.org/software/time/)
* [Amphetamine](https://apps.apple.com/us/app/amphetamine/id937984704?mt=12) (macOS), [The Caffeine](https://www.microsoft.com/store/productId/9PJBW5SCH9LC) (Windows), [Caffeine](https://launchpad.net/caffeine) (Linux) can prevent hibernation during any long builds

# CONTRIBUTING

For more details on developing tinyrick itself, see [DEVELOPMENT.md](DEVELOPMENT.md).

# FAQ

## How can I provide more resources to Docker?

* For laptops, ensure host is receiving power from a wall outlet
* Quit any other resource-intensive applications that may be running
* Set `CPUs` to the number of (efficiency) cores
* Set `Memory` to 8 GB or higher
* Set `Swap` to 1 GB or higher
* Set `Virtual disk limit` to 128 GB or higher
* Remove stale containers listed in `docker ps -a`
* Remove stale images listed in `docker images`
* Run `docker system prune -a`

# CREDITS

* [cross](https://github.com/cross-rs/cross) underlying cross-compiler system
* [cross-toolchains](https://github.com/cross-rs/cross-toolchains) provisions cross Docker images
* [cubejs/rust-cross](https://hub.docker.com/r/cubejs/rust-cross/tags) Docker images for additional cross targets
* [factorio](https://github.com/mcandre/factorio) generates Go application ports based on the standard Go toolchain
* [freeznet](https://hub.docker.com/u/freeznet) Docker images for additional cross targets
* [tug](https://github.com/mcandre/tug) automates multi-platform Docker image builds
