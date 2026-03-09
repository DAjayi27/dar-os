
use core::fmt;


/// Trait for low-level write operations.
///
/// Implementors of this trait provide the underlying hardware or system-specific
/// write functionality for outputting text.
pub trait Write {
    /// Creates a new instance of the writer.
    fn new() -> Self;

    /// Writes a string to the output.
    ///
    /// # Arguments
    ///
    /// * `string` - The string slice to write
    fn write(&mut self,string: &str);

    /// Writes a single character to the output.
    ///
    /// # Arguments
    ///
    /// * `char` - The character to write
    fn write_char(&mut self, char:char);

    /// Clears a specific row of the output.
    ///
    /// # Arguments
    ///
    /// * `row` - The row index to clear
    fn clear_row(&mut self, row:usize );

    /// Clears the entire screen/output.
    fn clear_screen(&mut self);
}/// A generic writer that wraps a type implementing the `Write` trait.
///
/// This provides a higher-level interface for writing operations and implements
/// the standard library's `fmt::Write` trait for formatting support.
pub struct Writer<T:Write> {
    writer_driver: T,
}impl <T: Write> Writer<T>{

    /// Creates a new `Writer` instance with a new underlying writer driver.
    pub fn new() -> Self{
        return Self{
            writer_driver: T::new(),
        }
    }

    /// Writes a string using the underlying writer driver.
    ///
    /// # Arguments
    ///
    /// * `string` - The string slice to write
    pub fn write(&mut self, string: &str){
        self.writer_driver.write(string);
    }

    /// Writes a single character using the underlying writer driver.
    ///
    /// # Arguments
    ///
    /// * `char` - The character to write
    pub fn write_char(&mut self, char: char) {
        self.writer_driver.write_char(char);
    }

}/// Implementation of `fmt::Write` trait for `Writer`.
///
/// This allows `Writer` to be used with Rust's formatting macros like `write!`.
impl <T: Write> fmt::Write for Writer<T>{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s);
        return Ok(());
    }
}