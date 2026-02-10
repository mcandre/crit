#!/bin/sh
unset IFS
set -eu

EXECUTABLES='crit'

DEST="$HOME/.local/bin"

set -- "$EXECUTABLES"

for EXECUTABLE in "$@"; do
    F="$DEST/$EXECUTABLE"

    # Idempotent
    if [ -f "$F" ]; then
        rm -f "$F"
    fi
done

echo "removed executables: ($EXECUTABLES) from: $DEST"
