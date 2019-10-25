use core::fmt;
use core::fmt::Write;
use crate::vga::writer::VGA_WRITER;

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    VGA_WRITER.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::print::_print(format_args!($($arg)*)));
}
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
