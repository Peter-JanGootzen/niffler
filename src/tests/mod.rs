#![cfg(test)]

use crate::qemu::QemuExitCode;
use crate::qemu::exit_qemu;

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
