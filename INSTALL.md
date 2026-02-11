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

Supported host environments:

* FreeBSD (x86_64)
* macOS (aarch64 / x86_64)
* NetBSD (x86_64)
* Linux (aarch64 / x86_64)
* Illumos (x86_64)
* Windows (aarch64 / x86_64) via [WSL](https://learn.microsoft.com/en-us/windows/wsl/)

Prerequisites:

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

Supported host environments:

* FreeBSD (x86_64)
* macOS (aarch64 / x86_64)
* NetBSD (x86_64)
* Linux (aarch64 / x86_64)
* Illumos (x86_64)
* Windows (aarch64 / x86_64) native or [WSL](https://learn.microsoft.com/en-us/windows/wsl/)

# INSTALL (DOCKER)

The Docker installation method downloads crit itself as a Docker image.

```sh
docker pull n4jm4/crit
```

## System Requirements

* [Docker in Docker](https://www.docker.com/resources/docker-in-docker-containerized-ci-workflows-dockercon-2023/)
