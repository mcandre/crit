# CONFIGURATION

crit loads an optional `crit.toml` file in the current working directory.

## Example

```toml
# debug = true

# Compile musl targets fully static
#
# rustflags."musl" = "-C target-feature=+crt-static"

# All functional, non-mobile, non-embedded cross targets
#
# exclusion_targets = [
#     "android",
#     "arm64ec-pc-windows-msvc",
#     "cuda",
#     "emscripten",
#     "fortanix",
#     "fuchsia",
#     "gnullvm",
#     "gnux32",
#     "i686-pc-windows-gnu",
#     "ios",
#     "loongarch",
#     "none-eabi",
#     "ohos",
#     "pc-solaris",
#     "powerpc64le-unknown-linux-musl",
#     "redox",
#     "riscv64gc-unknown-linux-musl",
#     "sparcv9-sun-solaris",
#     "uefi",
#     "unknown-none",
#     "wasm",
# ]

# Skip GNU targets and 32 bit targets
#
# exclusion_targets = [
#     "android",
#     "arm",
#     "cuda",
#     "emscripten",
#     "fortanix",
#     "fuchsia",
#     "gnu",
#     "i586",
#     "i686",
#     "ios",
#     "loongarch",
#     "none-eabi",
#     "ohos",
#     "pc-solaris",
#     "powerpc64le-unknown-linux-musl",
#     "redox",
#     "riscv64gc-unknown-linux-musl",
#     "sparcv9-sun-solaris",
#     "uefi",
#     "unknown-none",
#     "wasm",
# ]

# exclusion_features = [
#     "letmeout",
# ]

# banner = "<app>-<version>"

# cross_args = []
```

# debug

Default: `false`

Enables additional logging.

# rustflags

Default:

```sh
$RUSTFLAGS
```

Maps target triple patterns to RUSTFLAGS.

Patterns use Rust [regex](https://crates.io/crates/regex) notation.

# exclusion_targets

Default:

```toml
[
    "android",
    "cuda",
    "emscripten",
    "fortanix",
    "fuchsia",
    "gnullvm",
    "gnux32",
    "i686-pc-windows-gnu",
    "ios",
    "loongarch",
    "msvc",
    "none-eabi",
    "ohos",
    "pc-solaris",
    "powerpc64le-unknown-linux-musl",
    "redox",
    "riscv64gc-unknown-linux-musl",
    "sparcv9-sun-solaris",
    "uefi",
    "unknown-none",
    "wasm",
]
```

Collects patterns of exclusions to skip matching targets.

Patterns use Rust [regex](https://crates.io/crates/regex) notation.

# exclusion_features

Default:

```toml
["letmeout"]
```

Collects patterns of exclusions to skip matching crate features.

Patterns use Rust [regex](https://crates.io/crates/regex) notation.

# banner

Default:

```toml
""
```

Nests artifacts in a convenient parent directory, e.g. `<app>-<version>/`.

# cross_args

Default:

```toml
[]
```

Supply additional command line arguments to `cross` commands.
