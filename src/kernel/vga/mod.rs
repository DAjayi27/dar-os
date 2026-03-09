pub mod driver;

use core::fmt;
pub use driver::{VgaDriver};
use lazy_static::lazy_static;
use spin::Mutex;
use crate::kernel::writer::writer::{Write, Writer};

/// Global static writer instance protected by a mutex.
///
/// This writer uses a VGA driver for output and is lazily initialized.
/// The mutex ensures safe concurrent access to the writer.
lazy_static! {
    pub static ref WRITER: Mutex<Writer<VgaDriver>> = Mutex::new(Writer::new());
}


/// Internal function used by the `print!` and `println!` macros.
///
/// Acquires the global writer lock and writes the formatted arguments to the VGA buffer.
///
/// # Arguments
///
/// * `args` - Formatted arguments to write
///
/// # Panics
///
/// Panics if writing to the VGA buffer fails.
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}