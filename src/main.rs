#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#[macro_use]
mod vga_buffer;


use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello from Reco{}", "!");
    #[cfg(test)]
    test_main();
    loop {}
}
#[cfg(test)]

fn test_runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test();
    }
}
#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}