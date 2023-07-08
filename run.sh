#!/bin/bash

if [ $# -eq 0 ]; then
  echo "Usage: ./render.sh [image_name]"
  exit 1
fi

mkdir -p ./renders
RUSTFLAGS="--allow dead_code" cargo run --manifest-path=./tracer/Cargo.toml

if command -v eog >/dev/null 2>&1; then
  eog ./output.ppm 2> /dev/null
  cp ./output.ppm ./renders/$1.ppm
else
  echo "Looks like you're not using gnome."
  echo "You can replace *eog* in this script with your preferred image viewer."
  exit 1
fi

