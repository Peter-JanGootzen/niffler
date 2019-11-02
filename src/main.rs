#![no_std]
#![no_main]
#![test_runner(crate::tests::test_runner)]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]

#[macro_use]
mod vga;
mod tests;
mod qemu;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    println!("A niffler appeared!");

    loop {}
}


