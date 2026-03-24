# CONFIGURATION

crit uses [TOML](https://toml.io/en/) syntax for configuration files.

# crit.toml

crit looks for a `crit.toml` file in the current working directory.

# debug

Default: `false`

Enables additional logging.

Example:

```toml
debug = true
```

# banner

Nests artifacts in a convenient parent directory.

Example:

```toml
banner = "hello"
```

# rustflags

Default: (`RUSTFLAGS` environment variable)

Maps target triple patterns to RUSTFLAGS.

Patterns use Rust [regex](https://crates.io/crates/regex) notation.

Example:

```toml
# Linux Portability
rustflags."musl" = "-C target-feature=+crt-static"
```

# feature_excludes

Default: (empty)

Skips cargo binaries with the named features.

Example:

```toml
feature_excludes = [
    "letmeout",
]
```

# cross_args

Default: (empty)

Supply additional command line arguments to `cross` commands.

Example:

```toml
cross_args = [
    "-v",
]
```

# Rust Targets

[Rust Documentation](https://doc.rust-lang.org/stable/rustc/platform-support.html)

For details on available targets, run `rustup target list`.

## arch

Default: (empty)

Enables the named Rust target ISA's.

Example:

```toml
# Skip missing/broken cross images
arch = [
    "aarch64",
    "arm",
    # "arm64ec",
    "armv5te",
    "armv7",
    "armv7a",
    "armv7r",
    "armv8r",
    "i586",
    "i686",
    "loongarch64",
    "nvptx64",
    "powerpc",
    "powerpc64",
    "powerpc64le",
    "riscv32i",
    "riscv32im",
    "riscv32imac",
    "riscv32imafc",
    "riscv32imc",
    "riscv64gc",
    "riscv64imac",
    "s390x",
    "sparc64",
    # "sparcv9",
    "thumbv6m",
    "thumbv7em",
    "thumbv7m",
    "thumbv7neon",
    "thumbv8m.base",
    "thumbv8m.main",
    "wasm32",
    "wasm32v1",
    "x86_64",
]
```

# vendor

Default: (empty)

Enables the named Rust target vendors.

Example:

```toml
# Skip missing/broken cross images
# Skip bare metal
vendor = [
    "apple",
    # "fortanix",
    "linux",
    "nvidia",
    "pc",
    # "sun",
    "unknown",
    # "wasip1",
    # "wasip2",
    # "none",
]
```

# os

Default: (empty)

Enables the named Rust target operating systems.

Example:

```toml
# Skip missing/broken cross images
# Skip bare metal
# Skip mobile SDKs
os = [
    "",
    # "android",
    # "androideabi",
    # "cuda",
    "darwin",
    "eabi",
    "eabihf",
    "emscripten",
    "freebsd",
    # "fuchsia",
    "illumos",
    # "ios",
    "linux",
    "netbsd",
    # "none",
    # "redox",
    # "solaris",
    "threads",
    # "uefi",
    "unknown",
    "windows",
]
```

# abi

Default: (empty)

Enables the named Rust target ABI's.

Example:

```toml
# Skip missing/broken cross images
abi = [
    "",
    "elf",
    "gnu",
    "gnueabi",
    "gnueabihf",
    # "gnullvm",
    # "gnux32",
    "macabi",
    "msvc",
    "musl",
    "musleabi",
    "musleabihf",
    # "ohos",
    "sgx",
    "sim",
    "softfloat",
]
```

# target_excludes

Default: (empty)

Skips the named Rust target identifiers.

Example:

```toml
# Skip missing/broken cross images
target_excludes = [
    "i686-pc-windows-gnu",
    "powerpc64le-unknown-linux-musl",
    "riscv64gc-unknown-linux-musl",
    "wasm32-unknown-unknown",
]
```

# binary_extensions

Default:

```toml
[
    "",
    "exe",
    "js",
    "wasm",
]
```

Collates the named file extensions.

Example:

```toml
binary_extensions = [
    "",
    "exe",
]
```
