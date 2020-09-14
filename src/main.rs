#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

extern crate rlibc;
extern crate volatile;
extern crate lazy_static;
extern crate spin;
mod vga_buffer;

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Just printing lines with a counter to demonstrate text moving up the screen.
    let mut count = 0;
    loop {
        count += 1;
        println!("Line {}", count);
    }
    
    loop {}
}