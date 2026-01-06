# DEMO APPS

# EXAMPLES

```console
$ hello
Hello World!
```

# REQUIREMENTS

* [rustup](https://rustup.rs/) 1.28.1+
* [Rust](https://www.rust-lang.org/en-US/) 1.92.0+
* [cargo-audit](https://crates.io/crates/cargo-audit)
* [crit](https://github.com/mcandre/crit)
* [cross](https://crates.io/crates/cross) 4e64366af6095c84fa4f54a0fa5a2ba7d9a271aa
* [Docker](https://www.docker.com/) 28.0.1+

## Recommended

* POSIX compatible [tar](https://pubs.opengroup.org/onlinepubs/7908799/xcu/tar.html)

# BUILD & INSTALL

```console
$ cargo install --bins --path .
```

# UNINSTALL

```console
$ cargo uninstall demo
```

# PORT

```console
$ crit -b hello-0.0.1
$ sh -c "cd .crit/bin && tar czf hello-0.0.1.tgz hello-0.0.1"
```
