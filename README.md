# wasm-gol

This repo implements a `no_std` version of Conway's Game of Life in Rust that uses a compile-time constant sized matrix.

It has the following packages:

- `gol`: `no_std` Game of Life library. It should compile to most architectures, like `wasm32-unknown-unknown`, without issues.
- `wasm-gol`: Contains engine (Bevy? eGUI?) for running the Game of Life in the browser.
