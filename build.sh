#! /bin/bash

set -euo pipefail

cargo build --release

mkdir -p target/gba
for product in buttons running; do
  rom_path="target/gba/${product}.gba"
  # rom title is the UPPERCASE product name padded or clipped to 12 bytes
  rom_title="$(printf "%-12.12s" "$(echo $product | tr '[:lower:]' '[:upper:]')")"
  # rom code is the UPPERCASE first two characters of the product name with circumfix C__E
  rom_code="$(printf "C%-2.2sE" "$(echo $product | tr '[:lower:]' '[:upper:]')")"
  rom_maker='DA'
  rom_version='0'
  arm-none-eabi-objcopy -O binary "target/thumbv4t-none-eabi/release/${product}" "$rom_path"
  gbafix -t"$rom_title" -c"$rom_code" -m"$rom_maker" -r"$rom_version" "$rom_path"
  echo "> Created $rom_path (AGB-$rom_code $rom_title)"
done
