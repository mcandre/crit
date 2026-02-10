#!/bin/sh
unset IFS
set -euf

GH_FORK='mcandre'
APP='crit'
EXECUTABLES='crit'
VERSION='0.0.14'

DEST="$HOME/.local/bin"

fallback() {
    echo "select another install method: https://github.com/$GH_FORK/$APP/blob/main/INSTALL.md" >&2
}

uname_to_rust_os() {
    case "$1" in
        Darwin*)
            echo 'apple-darwin'
            ;;
        FreeBSD*)
            echo 'unknown-freebsd'
            ;;
        Linux*)
            echo 'unknown-linux-musl'
            ;;
        NetBSD*)
            echo 'unknown-netbsd'
            ;;
        *illumos*)
            echo 'unknown-illumos'
            ;;
        *)
            echo "error: unsupported os. uname -a: $1" >&2
            fallback
            exit 1
            ;;
    esac
}

uname_arch_to_rust_arch() {
    case "$1" in
        aarch64 | arm64)
            echo 'aarch64'
            ;;
        x86_64)
            echo 'x86_64'
            ;;
        *)
            echo "error: unsupported arch. uname -a: $(uname -a)" >&2
            fallback
            exit 1
            ;;
    esac
}

UNAME="$(uname -a)"
OS="$(uname_to_rust_os "$UNAME")"
ARCH="$(uname_arch_to_rust_arch "$(uname -m)")"

curl -LO "https://github.com/$GH_FORK/$APP/releases/download/v${VERSION}/$APP-$VERSION.tgz"
tar xzf "$APP-${VERSION}.tgz"

if [ ! -d "$APP-$VERSION/$ARCH-$OS" ]; then
    echo "error: unsupported platform. uname -a: $UNAME"
    fallback
fi

mkdir -p "$DEST"
set -- "$EXECUTABLES"

for EXECUTABLE in "$@"; do
    cp "$APP-$VERSION/$ARCH-$OS/$EXECUTABLE" "$DEST"
done

echo "installed $APP binaries to $DEST"
rm -rf "$APP-$VERSION" "$APP-$VERSION.tgz"
