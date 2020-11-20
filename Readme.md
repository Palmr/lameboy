# Lameboy


[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.txt)
![Rust](https://github.com/Palmr/lameboy/workflows/Rust/badge.svg)

Yet another Game Boy emulator.

This emulator is a rust learning project for me. It currently isn't aiming to be the most accurate emulator, the fastest
 or the one with the best code. It's just here as something fun for me to play with while learning how to use rust.

## Current Status

Lameboy currently loads non-MBC roms and can run some, but it has plenty of issues.

There are plenty of debug windows implemented which can help track down issues as they come up.

![Screenshot of first BG displaying correctly](images/screenshot-25-7-17.png)

![Debug windows galore](images/screenshot-18-11-17.png)

### TODO

- Support MBC1
- Fix the many bugs that currently exist
  - Add support for timer operation (TAC, TIMA, TMA)
  - Verify all existing instructions work
  - Re-write all the PPU code
- Handle all interrupt types & HALT 
- Support all MBC variants
- Handle the construction of the various components better in rust
- Game Boy Color support
- Sound
- Serial support
- Game Boy Camera & Printer support
- Ever more debug windows
  - Watchpoints
    - Watch address
    - See it in u8, u16, i8, i16
    - See it in hex, binary, decimal
    - Chart it?
  - Comment/name calls in disassembly window (save comments too)

