#!/bin/bash

if [ $# -eq 0 ]; then
  echo "Usage: ./render.sh [image_name].ppm"
  exit 1
fi

RUSTFLAGS="--allow dead_code" cargo run --manifest-path=./tracer/Cargo.toml > $1.ppm
eog $1.ppm 2> /dev/null
