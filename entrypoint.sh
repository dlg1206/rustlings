#!/bin/bash

MOUNT="./rustlings"
cd mount || exit 1
# Init if rustlings wasn't installed or the mount is empty
if [ ! -d "$MOUNT" ] || { [ -d "$MOUNT" ] && [ -z "$(ls -A $MOUNT)" ]; }; then
    yes | rustlings init
fi
cd "$MOUNT" || exit 1
rustlings