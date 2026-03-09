pub mod writer;
pub use writer::{Write, Writer};


/// Prints formatted text to the screen without a newline.
/// 
/// This macro works similarly to the standard `print!` macro but writes to the VGA buffer.
/// 
/// # Examples
/// 
/// ```
/// print!("Hello, {}!", "world");
/// ```
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::kernel::vga::_print(format_args!($($arg)*)));
}

/// Prints formatted text to the screen with a newline.
/// 
/// This macro works similarly to the standard `println!` macro but writes to the VGA buffer.
/// 
/// # Examples
/// 
/// ```
/// println!("Hello, world!");
/// println!("The answer is {}", 42);
/// ```
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
