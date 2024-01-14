# DEMO APPS

# EXAMPLES

```console
$ hello
Hello World!
```

# REQUIREMENTS

* [Rust](https://www.rust-lang.org/en-US/) 1.64+
* [cross](https://crates.io/crates/cross) 0.2.5+
* [Docker](https://www.docker.com/) 20.10.23+
* [crit](https://github.com/mcandre/crit)

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
