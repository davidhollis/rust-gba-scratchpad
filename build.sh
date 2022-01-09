#! /bin/bash

set -euo pipefail

cargo build --release

mkdir -p target/gba
for product in buttons running; do
  rom_path="target/gba/${product}.gba"
  arm-none-eabi-objcopy -O binary "target/thumbv4t-none-eabi/release/${product}" "$rom_path"
  gbafix "$rom_path"
  echo "> Created $rom_path"
done
