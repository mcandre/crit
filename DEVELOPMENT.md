# BUILDTIME REQUIREMENTS

* [rustup](https://rustup.rs/) 1.25.2+
* [Rust](https://www.rust-lang.org/en-US/) 1.68.2+ with `rustup component add clippy` and `cargo install cargo-audit@0.17.5 tinyrick@0.0.9`
* [Docker](https://www.docker.com/) 20.10.12+
* a POSIX compliant [sh](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/sh.html) implementation

## Recommended

* [ASDF](https://asdf-vm.com/) 0.10 (run `asdf reshim` after each Rust application binary installation)
* [direnv](https://direnv.net/) 2
* [cargo-cache](https://crates.io/crates/cargo-cache)
* [tree](https://en.wikipedia.org/wiki/Tree_(command))
* GNU compatible [time](https://www.gnu.org/software/time/)
* [zip](https://en.wikipedia.org/wiki/ZIP_(file_format))

# INSTALL BINARIES FROM SOURCE

```console
$ cargo install --bins --path .
```

# UNINSTALL BINARIES

```console
$ cargo uninstall crit
```

# SECURITY AUDIT

```console
$ cargo audit
```

# TEST

```console
$ crit -l
```

# PORT

```console
$ crit -b crit-0.0.5
$ sh -c "cd .crit/bin && zip -r crit-0.0.5.zip crit-0.0.5"
```
