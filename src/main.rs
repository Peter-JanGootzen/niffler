#![no_std]
#![no_main]
#![test_runner(niffler::test_runner)]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]

#[macro_use]
extern crate niffler;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("A niffler appeared on the VGA buffer!");
    serial_println!("A niffler appeared on the serial output");

    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    niffler::test_panic_handler(info)
}
