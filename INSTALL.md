# INSTALL

We support several installation methods.

# RUNTIME REQUIREMENTS

* [Docker](https://www.docker.com/) 28.0.1+
* [Rust](https://www.rust-lang.org/en-US/) 1.92.0+
* [cross](https://github.com/cross-rs/cross) at ref `4e64366af6095c84fa4f54a0fa5a2ba7d9a271aa`

```sh
cargo install --force cross --git https://github.com/cross-rs/cross --rev 4e64366af6095c84fa4f54a0fa5a2ba7d9a271aa
```

# PRECOMPILED BINARIES

https://github.com/mcandre/crit/releases

1. Download release archive.
2. Extract archive.
3. Select executables for your target platform.
4. Copy executabless to a convenient location, e.g. `$HOME/bin`.
5. Ensure location is registered in `$PATH`.

# DOCKER

```sh
docker pull n4jm4/crit
```

# BUILD FROM SOURCE

```sh
cargo install --force --path .
```

For more details on developing crit itself, see [DEVELOPMENT.md](DEVELOPMENT.md).
