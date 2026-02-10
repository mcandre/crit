# INSTALL

We support several installation methods.

# RUNTIME REQUIREMENTS

* [Docker](https://www.docker.com/) 28.0.1+
* [Rust](https://www.rust-lang.org/en-US/) 1.92.0+
* [cross](https://github.com/cross-rs/cross) at ref `4e64366af6095c84fa4f54a0fa5a2ba7d9a271aa`

```sh
cargo install --force cross --git https://github.com/cross-rs/cross --rev 4e64366af6095c84fa4f54a0fa5a2ba7d9a271aa
```

# CURL

## Requirements

* a UNIX-like environment (e.g. [WSL](https://learn.microsoft.com/en-us/windows/wsl/))
* Ensure `$HOME/.local/bin` registered in your shell's `PATH`
* [curl](https://curl.se/)

```sh
curl -L https://raw.githubusercontent.com/mcandre/crit/refs/heads/main/install-crit | sh
```

## Uninstall

```sh
curl -L https://raw.githubusercontent.com/mcandre/crit/refs/heads/main/uninstall-crit | sh
```

# PRECOMPILED BINARIES

https://github.com/mcandre/crit/releases

1. Download a tarball corresponding to your environment's architecture and OS.
2. Extract executables into a suitable directory.

   Examples:

   * `~/.local/bin`
   * `~/bin`
   * `/usr/local/bin`
   * `~\AppData\Local`

3. Ensure the directory is registered in your shell's `PATH`.

# DOCKER

```sh
docker pull n4jm4/crit
```

# BUILD FROM SOURCE

```sh
cargo install --force --path .
```

For more details on developing crit itself, see [DEVELOPMENT.md](DEVELOPMENT.md).
