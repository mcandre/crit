#!/bin/bash
unset IFS
set -euf

DEST="$HOME/.local/bin"
EXECUTABLES=(crit)

for EXECUTABLE in "${EXECUTABLES[@]}"; do
    F="$DEST/$EXECUTABLE"

    # Idempotent
    if [ -f "$F" ]; then
        rm -f "$F"
    fi
done

echo "removed executables: (${EXECUTABLES[*]}) from: $DEST"
