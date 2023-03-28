# BUILDTIME REQUIREMENTS

* [Rust](https://www.rust-lang.org/en-US/) 1.63+ with `rustup component add clippy` and `cargo install cargo-audit@0.17.5 tinyrick@0.0.9`
* [Docker](https://www.docker.com/) 20.10.12+

## Recommended

* [ASDF](https://asdf-vm.com/) 0.10 (run `asdf reshim` after each Rust application binary installation)
* [direnv](https://direnv.net/) 2
* [cargo-cache](https://crates.io/crates/cargo-cache)
* [tree](https://en.wikipedia.org/wiki/Tree_(command))

# INSTALL BINARIES FROM SOURCE

```console
$ tinyrick install
```

# UNINSTALL BINARIES

```console
$ tinyrick uninstall
```

# SECURITY AUDIT

```console
$ tinyrick audit
```

# BUILD: Doc, Lint, Test, and Compile

```console
$ tinyrick [build]
```

# PORT

```console
$ tinyrick port
```

# PUBLISH CRATE

```console
$ tinyrick publish
```

# CLEAN

```console
$ tinyrick clean
```
