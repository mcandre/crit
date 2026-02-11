# DEVELOPMENT GUIDE

crit follows standard, cargo based operations for compiling and unit testing Rust code.

For advanced operations, such as linting and generating install media artifacts, we further supplement with some software industry tools.

# BUILDTIME REQUIREMENTS

* a UNIX-like environment (e.g. [WSL](https://learn.microsoft.com/en-us/windows/wsl/))
* [bash](https://www.gnu.org/software/bash/) 4+
* [cross](https://crates.io/crates/cross) 4e64366af6095c84fa4f54a0fa5a2ba7d9a271aa
* [Docker](https://www.docker.com/) 28.0.1+
* POSIX compliant [findutils](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/find.html)
* POSIX compliant [make](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/make.html)
* [rustup](https://rustup.rs/) 1.28.1+
* [Rust](https://www.rust-lang.org/en-US/)
* [GNU tar](https://www.gnu.org/software/tar/)
* Provision additional dev tools with `make -f install.mk`

## Recommended

* a host capable of running musl/Linux containers (e.g. a GNU/Linux, musl/Linux, macOS, or Windows host)
* [Docker First Aid Kit](https://github.com/mcandre/docker-first-aid-kit)
* Apply `DOCKER_DEFAULT_PLATFORM` = `linux/amd64` environment variable
* [ASDF](https://asdf-vm.com/) 0.18 (run `asdf reshim` after provisioning)
* [direnv](https://direnv.net/) 2
* [GNU time](https://www.gnu.org/software/time/)
* [tree](https://en.wikipedia.org/wiki/Tree_(command))

# INSTALL BINARIES FROM SOURCE

```sh
make install
```

# UNINSTALL BINARIES

```sh
make uninstall
```

# SECURITY AUDIT

```sh
make audit
```

# LINT

```sh
make lint
```

# TEST

```sh
make test
```

# PORT

```sh
make port
```

# PUBLISH

```sh
make publish
```

# CLEAN

```sh
make clean
```
