use x86_64::instructions::interrupts::without_interrupts;
use core::fmt;
use core::fmt::Write;
use crate::io::vga::writer::VGA_WRITER;

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    without_interrupts(|| {
        VGA_WRITER.lock().write_fmt(args).unwrap();
    })
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::vga::print::_print(format_args!($($arg)*)));
}
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
