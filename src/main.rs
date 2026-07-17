#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]

#[macro_use]
mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome from Reco!");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn z_getrandom(_buf: *mut u8, _len: usize) -> u32 {
    0
}