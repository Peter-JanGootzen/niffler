#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use lazy_static::lazy_static;
use x86_64::structures::idt::{ InterruptDescriptorTable, InterruptStackFrame };
use core::panic::PanicInfo;
use niffler::qemu::{exit_qemu, QemuExitCode};
use niffler::serial_println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    niffler::cpu::gdt::init();
    idt_init_test();

    stack_overflow();

    panic!("Code execution continued after the stack overflow.");
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    // This volatile read prevents recursion optimizations
    // https://en.wikipedia.org/wiki/Tail_call
    volatile::Volatile::new(0).read();
}

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault.set_handler_fn(test_double_fault_handler)
                .set_stack_index(niffler::cpu::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

fn idt_init_test() {
    TEST_IDT.load();
}

extern "x86-interrupt" fn test_double_fault_handler(_stack_frame: &mut InterruptStackFrame, _error_code: u64) -> ! {
    serial_println!("Double Fault handler succesfully called after stack overflow.");
    serial_println!("stack_overflow... [ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    niffler::test_panic_handler(info)
}
