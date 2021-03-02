# riotboot-rs

An implementation of RIOT's bootloader in Rust and Hacspec.
It works as a drop-in replacement for RIOT's riotboot bootloader, albeit with
reduced features (no serial, no dfu, ...).

## Status

Both the pure Rust and the Hacspec implementation have been tested on nrf52840dk.

## How to build

riotboot-rs supports both pure Rust and a Hacspec built. Choose by using one of the
features "internal" or "verified". Beware, the non-release build might be too large
to fit in the bootloader area as specified in memory.x.

    $ cargo build --release --target thumbv7em-none-eabi --features verified

## How to flash

The resuling binary can be flashed with the included `flash.sh`:

    $ sh flash.sh target/thumbv7em-none-eabi/release/riotboot-rs

## How to use with other boards

The actual source code should work as is on every Cortex-M MCU.
The memory map (`memory.x`) might need adjustment, as does the flash script.
