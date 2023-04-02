# tinyrick_extras: common tasks for tinyrick projects

# EXAMPLE

```console
$ cd example
$ tinyrick
running 1 test
test smoketest ... ok
...
Value(94)
Buzz
Fizz
Value(97)
Value(98)
Fizz
Buzz
```

# ABOUT

tinyrick_extras defines some common tasks, such as unit tests, linting, generating API documentation, publishing packages, installing and uninstalling packages, for your [tinyrick](https://github.com/mcandre/tinyrick) projects. Boom. Take what works for your build workflow, leave the rest.

See the [example](example) project for usage. Other examples include [ios7crypt-rs](https://github.com/mcandre/ios7crypt-rs).

# CRATE

https://crates.io/crates/tinyrick_extras

# API DOCUMENTATION

https://docs.rs/tinyrick_extras/

# RUNTIME REQUIREMENTS

* [Rust](https://www.rust-lang.org/en-US/) 1.30+

## Recommended

* [clippy](https://github.com/rust-lang-nursery/rust-clippy) when running the `tinyrick_extras::clippy` task.

# CONTRIBUTING

For more details on developing tinyrick_extras itself, see [DEVELOPMENT.md](DEVELOPMENT.md).
