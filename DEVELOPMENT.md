# BUILDTIME REQUIREMENTS

* [rustup](https://rustup.rs/) 1.28.1+
* [Rust](https://www.rust-lang.org/en-US/) 1.87.0+
* [cross](https://crates.io/crates/cross) 4e64366af6095c84fa4f54a0fa5a2ba7d9a271aa
* [Docker](https://www.docker.com/) 28.0.1+
* [POSIX](https://pubs.opengroup.org/onlinepubs/9799919799/) compatible [make](https://en.wikipedia.org/wiki/Make_(software))
* [POSIX](https://pubs.opengroup.org/onlinepubs/9799919799/) compatible [tar](https://en.wikipedia.org/wiki/Tar_(computing))
* Provision additional dev tools with `make [-j 4]`

## Recommended

* a host capable of running musl/Linux containers (e.g. a GNU/Linux, musl/Linux, macOS, or Windows host)
* [Docker First Aid Kit](https://github.com/mcandre/docker-first-aid-kit)
* Apply `DOCKER_DEFAULT_PLATFORM` = `linux/amd64` environment variable
* [ASDF](https://asdf-vm.com/) 0.10 (run `asdf reshim` after provisioning)
* [cargo-cache](https://crates.io/crates/cargo-cache)
* [direnv](https://direnv.net/) 2
* [GNU](https://www.gnu.org/)/[BSD](https://en.wikipedia.org/wiki/Berkeley_Software_Distribution) [make](https://en.wikipedia.org/wiki/Make_(software))
* [GNU](https://www.gnu.org/)/[BSD](https://en.wikipedia.org/wiki/Berkeley_Software_Distribution) [tar](https://en.wikipedia.org/wiki/Tar_(computing))
* [GNU](https://www.gnu.org/) [time](https://en.wikipedia.org/wiki/Time_(Unix))
* [tree](https://en.wikipedia.org/wiki/Tree_(command))
* a [UNIX](https://en.wikipedia.org/wiki/Unix)-like environment

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

# LINT

```console
$ tinyrick lint
```

# TEST

```console
$ tinyrick test
```

# PORT

```console
$ tinyrick port
```

# PUBLISH

```console
$ tinyrick publish
```

# CLEAN

```console
$ tinyrick clean
```
