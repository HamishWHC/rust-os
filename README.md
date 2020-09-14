# Setup Instructions

- Install rust-src: `rustup component add rust-src`
- Install llvm-tools-preview: `rustup component add llvm-tools-preview`
- Install cargo bootimage: `cargo install bootimage`

Build: `cargo build`
Run in QEMU: `cargo run`
Create bootable image: `cargo bootimage`