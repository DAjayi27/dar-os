pub mod writer;
pub use writer::{Write, Writer};



use lazy_static::lazy_static;
use spin::Mutex;
use crate::kernel::vga::VgaDriver;
use core::fmt;




lazy_static! {
    pub static ref WRITER: Mutex<Writer<VgaDriver>> = Mutex::new(Writer::new());
}


#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::writer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}