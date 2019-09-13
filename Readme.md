# Lameboy


[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.txt)
[![Build Status](https://travis-ci.org/Palmr/lameboy.svg?branch=master)](https://travis-ci.org/Palmr/lameboy)

Yet another Game Boy emulator.

This emulator is a rust learning project for me. It currently isn't aiming to be the most accurate emulator, the fastest
 or the one with the best code. It's just here as something fun for me to play with while learning how to use rust.

## Current Status

Currently just about loads non-MBC roms and has plenty of issues (definitely need to write instruction-level tests).

Plenty of debug windows are implemented which helps track down issues as they come up however.

![Screenshot of first BG displaying correctly](images/screenshot-25-7-17.png)

![Debug windows galore](images/screenshot-18-11-17.png)

### TODO

- Fix the many bugs that currently exist
  - Verify all existing instructions work
  - Re-write all the PPU code
- Handle all interrupt types & HALT 
- Support all MBC variants
- Handle the construction of the various components better in rust
- Game Boy Color support
- Sound
- Serial support
- Game Boy Camera & Printer support
- Add a decompiler to make memory debugging easier
- Ever more debug windows
  - Watchpoints
    - Watch address
    - See it in u8, u16, i8, i16
    - See it in hex, binary, decimal
    - Chart it?
  - Comment/name calls in disassembly window

