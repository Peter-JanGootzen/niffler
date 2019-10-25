#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut i = 0;
    loop {
        println!("We are at: {}", i);
        i += 1;

        if i == 300 {
            panic!("No shiny things left here, the Niffler runs away.");
        }
    }
}
