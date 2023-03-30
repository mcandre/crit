# BUILDTIME REQUIREMENTS

* [Rust](https://www.rust-lang.org/en-US/) 1.64+ with `rustup component add clippy` and `cargo install cargo-audit@0.17.5 tinyrick@0.0.9`
* [Docker](https://www.docker.com/) 20.10.12+

## Recommended

* [ASDF](https://asdf-vm.com/) 0.10 (run `asdf reshim` after each Rust application binary installation)
* [direnv](https://direnv.net/) 2
* [cargo-cache](https://crates.io/crates/cargo-cache)
* [tree](https://en.wikipedia.org/wiki/Tree_(command))
* GNU compatible [time](https://www.gnu.org/software/time/)

# INSTALL BINARIES FROM SOURCE

```console
$ cargo install
```

# UNINSTALL BINARIES

```console
$ cargo uninstall crit
```

# SECURITY AUDIT

```console
$ cargo audit
```

# PORT

```console
$ crit
...
```
