#![no_std]
#![no_main]
#![test_runner(test_runner)]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]

#[macro_use]
mod io;
mod qemu;

use core::panic::PanicInfo;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    println!("A niffler appeared!");
    serial_println!("hello");

    loop {}
}

cfg_if::cfg_if! {
    if #[cfg(not(test))] {
        #[panic_handler]
        fn panic(info: &PanicInfo) -> ! {
            println!("{}", info);
            loop {}
        }
    } else {
        use qemu::{ QemuExitCode, exit_qemu };

        #[panic_handler]
        fn panic(info: &PanicInfo) -> ! {
            serial_println!("[failed]");
            serial_println!("Error: {}", info);
            exit_qemu(QemuExitCode::Failure);
            loop {}
        }
        pub fn test_runner(tests: &[&dyn Fn()]) {
            println!("Running {} tests", tests.len());
            for test in tests {
                test();
            }
            exit_qemu(QemuExitCode::Success);
        }
        #[test_case]
        fn test_assert() {
            print!("trivial assertion... ");
            assert_eq!(1, 1);
            println!("[ok]");
        }
    }
}
