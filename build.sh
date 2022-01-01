#! /bin/bash

cargo build --release
arm-none-eabi-objcopy -O binary target/thumbv4t-none-eabi/release/rust-gba-testing target/buttons.gba
gbafix target/buttons.gba
