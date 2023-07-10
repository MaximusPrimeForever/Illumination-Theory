#!/bin/bash

if [ -z "$1" ]; then
  echo "Usage: ./render.sh image_name [optional: image_width] [optional: samples_per_pixel] [optional: trace_depth]"
  exit 1
fi

if [ -z "$2" ]; then
  image_width="400"
else
  image_width=$2
fi

if [ -z "$3" ]; then
  samples_per_pixel="100"
else
  samples_per_pixel=$3
fi

if [ -z "$4" ]; then
  trace_depth="10"
else
  trace_depth=$4
fi

mkdir -p ./renders
RUSTFLAGS="--allow dead_code" cargo run --release --manifest-path=./tracer/Cargo.toml -- $image_width $samples_per_pixel $trace_depth

if command -v eog >/dev/null 2>&1; then
  eog ./output.ppm 2> /dev/null
  cp ./output.ppm ./renders/$1.ppm
else
  echo "Looks like you're not using gnome."
  echo "You can replace *eog* in this script with your preferred image viewer."
  exit 1
fi

