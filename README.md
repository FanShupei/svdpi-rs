# Svdpi

Rust bindings for Systemverilog DPI and VPI.

**WARNING**: The development is at very early stage.
Features may be _incomplete_ and _NO stability_ is promised.

Currently main users are [t1](https://github.com/chipsalliance/t1) and [chisel-nix](https://github.com/chipsalliance/chisel-nix).

The normative documentation is SystemVerilog LRM (Languare Reference Manual). DPI and VPI interfaces are defined as part of the standard.

The most up-to-date standard version now is [IEEE 1800-2023](https://standards.ieee.org/ieee/1800/7743/).
The standard is available at no cost via the IEEE Get Program.

## High Level Bindings

Features are currently very limited and are developed as needed.

Feature requests and PRs are welcomed.

## Low Level Bindings

`svdpi.h` raw bindings lives in `sys::dpi`.

`svvpi.h` raw bindings lives in `sys::vpi`.

## SystemVerilog Language Version

The baseline is SystemVerilog 2017. Enable `sv2023` feature to use features defined in SystemVerilog 2023 (like `svGetTime`).

## Use VPI

Enable `vpi` feature to use VPI functions. Currently we focus on ultilizing VPI functions inside DPI functions.

As LRM explicitly states "For VPI access (or any other interface access) to be possible, the appropriate implementation-defined mechanism shall still be used to enable these interface(s)". E.g. Verilator requires you to verilate the design with `--vpi` option.

## Linking with Simulator

This crate only declares DPI (and VPI) function prototypes and does not try to interfere with the compilation process.

Users are responsible to compile the DPI library properly and link it with the simulator. Read LRM 2023 Annex J (Inclusion of foreign language code) and the simulator's manual for more help.
