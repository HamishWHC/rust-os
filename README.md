# Rust OS
This is a repo containing my code from following [this tutorial](https://os.phil-opp.com/) about writing an OS in Rust!

## Setup Instructions

- Install rust nightly (should be automatic via the rust-toolchain file, otherwise `rustup override set nightly`).
- Install rust-src: `rustup component add rust-src`
- Install llvm-tools-preview: `rustup component add llvm-tools-preview`
- Install cargo bootimage: `cargo install bootimage`

## Running

Build: `cargo build`
Run in QEMU: `cargo run`
Create bootable image: `cargo bootimage`