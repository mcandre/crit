# INSTALL GUIDE

In addition to cargo, crit supports alternative installation methods.

# INSTALL (CURL)

curl based installs automatically download and extract precompiled binaries.

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

### Bitness

64

### Hosts

* FreeBSD (Intel)
* macOS (ARM, Intel)
* NetBSD (Intel)
* Linux (ARM, Intel)
* Illumos (Intel)
* WSL (ARM, Intel)

Prerequisites:

* [bash](https://www.gnu.org/software/bash/) 4+
* [curl](https://curl.se/)

# INSTALL (PRECOMPILED BINARIES)

Precompiled binaries may be installed manually.

## Install

1. Download a [tarball](https://github.com/mcandre/crit/releases) corresponding to your environment's architecture and OS.
2. Extract executables into a selected directory.

   Examples:

   * `~/.local/bin` (XDG compliant per-user)
   * `/usr/local/bin` (XDG compliant global)
   * `~/bin` (BSD)
   * `~\AppData\Local` (native Windows)

## Postinstall

Ensure the selected directory is registered with your shell's `PATH` environment variable.

## Uninstall

Remove the application executables from the selected directory.

## System Requirements

### Bitness

64

### Hosts

* FreeBSD (Intel)
* macOS (ARM, Intel)
* NetBSD (Intel)
* Linux (ARM, Intel)
* Illumos (Intel)
* Windows (ARM, Intel)

# INSTALL (COMPILE FROM SOURCE)

```sh
git clone https://github.com/mcandre/crit.git
cd crit
cargo install --force --path .
```

## Prerequisites

* [cargo](https://doc.rust-lang.org/cargo/)
* [git](https://git-scm.com/)

For more details on developing crit itself, see our [development guide](DEVELOPMENT.md).
