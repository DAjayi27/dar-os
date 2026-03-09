use core::ptr;
use crate::kernel::colors::{Color, ColorCode};
use crate::kernel::writer::{Write, Writer};
use volatile::{ VolatilePtr};

/// The number of text rows in the VGA text buffer.
pub const BUFFER_HEIGHT: usize = 25;

/// The number of text columns in the VGA text buffer.
pub const BUFFER_WIDTH: usize = 80;

/// Represents a single character cell in the VGA text buffer.
///
/// Each cell consists of an ASCII byte and an associated [`ColorCode`] that
/// encodes both the foreground and background colours.  The `#[repr(C)]`
/// attribute ensures the field layout matches the hardware-defined format
/// (character byte followed by attribute byte).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    /// The ASCII value of the character to display.
    ascii_character: u8,
    /// The combined foreground/background colour code for this character cell.
    color_code: ColorCode,
}

/// A transparent wrapper around the full VGA text-mode frame buffer.
///
/// The `#[repr(transparent)]` attribute guarantees that this struct has the
/// same memory layout as its inner array, so a raw pointer cast from the
/// memory-mapped I/O address `0xb8000` is valid.
#[repr(transparent)]
struct Buffer {
    /// The two-dimensional array of character cells (`[row][col]`).
    data: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// High-level driver for the VGA text-mode display.
///
/// `VgaDriver` owns a mutable reference to the hardware-mapped [`Buffer`] at
/// address [`VgaDriver::VGA_MIO_ADDR`] and tracks the current cursor position
/// and active colour.  All writes to the display go through this struct so
/// that the rest of the kernel never needs to manipulate raw pointers.
pub struct VgaDriver{
    /// Current column (x) position of the cursor, zero-indexed.
    col_pos:usize,
    /// Current row (y) position of the cursor, zero-indexed.
    row_pos:usize,
    /// The colour code applied to newly written characters.
    curr_color: ColorCode,
    /// Exclusive mutable reference to the VGA hardware buffer.
    buffer: &'static mut Buffer,
}

// Safety: VgaDriver exclusively accesses the VGA hardware buffer at a fixed
// memory-mapped address (0xb8000). There is only ever one instance, protected
// by a Mutex, so sending it across threads is safe.
unsafe impl Send for VgaDriver {}

impl VgaDriver {

    /// The physical memory-mapped I/O address of the VGA text buffer.
    pub const VGA_MIO_ADDR: usize = 0xb8000;

    /// Moves the cursor to the beginning of the next row.
    ///
    /// Resets `col_pos` to `0` and increments `row_pos` by one.
    /// No bounds-checking is performed here; the caller is responsible for
    /// ensuring `row_pos` does not exceed [`BUFFER_HEIGHT`].
    fn write_new_line (&mut self){
        self.col_pos = 0;
        self.row_pos += 1;
    }

}

impl Write for VgaDriver {

    /// Creates a new `VgaDriver` pointing at the VGA text buffer.
    ///
    /// The cursor is placed at the top-left corner `(row 0, col 0)` and the
    /// default colour is white text on a black background.
    ///
    /// # Safety
    /// This function dereferences the raw pointer `0xb8000` to obtain a
    /// `&'static mut Buffer`.  It is safe as long as only one `VgaDriver`
    /// instance exists at a time (enforced by a `Mutex` at call sites).
     fn new() -> Self{
        let default_color = ColorCode::new(Color::White,Color::Black);
        return Self{
            row_pos: 0,
            col_pos: 0,
            curr_color: default_color,
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        }
    }

    /// Writes every character in `string` to the VGA buffer in order.
    ///
    /// Non-ASCII characters are silently discarded (see [`write_char`]).
    /// A newline is inserted automatically when the end of a row is reached.
    fn write(&mut self, string: &str) {
        for i in string.chars() {
            self.write_char(i);
        }
    }

    /// Writes a single character to the VGA buffer at the current cursor position.
    ///
    /// * Non-ASCII characters are ignored.
    /// * If the cursor has reached the end of the current row (`col_pos >= BUFFER_WIDTH`),
    ///   a new line is started before writing.
    /// * The write is performed with [`ptr::write_volatile`] to prevent the
    ///   compiler from optimising away the memory-mapped I/O store.
    /// * After a successful write, `col_pos` is advanced by one.
    fn write_char(&mut self,char:char){

        if !char.is_ascii() {
            return;
        }

        if self.col_pos >= BUFFER_WIDTH {
            self.write_new_line();
        }

        let color_code = self.curr_color;

        let screen_data =  ScreenChar {
            ascii_character: (char as u8),
            color_code,
        };

        unsafe {
            let reference = &mut self.buffer.data[self.row_pos][self.col_pos];
            ptr::write_volatile(reference as *mut ScreenChar, screen_data);
        }

        self.col_pos += 1;
    }

    /// Overwrites every character cell in the given `row` with a blank space.
    ///
    /// The blank space is written with [`ptr::write_volatile`] using the
    /// current [`curr_color`](VgaDriver::curr_color) so the attribute byte is
    /// set consistently.
    ///
    /// # Parameters
    /// * `row` – Zero-indexed row number to clear.  Must be less than
    ///   [`BUFFER_HEIGHT`].
    fn clear_row(&mut self, row:usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.curr_color,
        };

        for i in 0..BUFFER_WIDTH {
            unsafe {
                let reference = &mut self.buffer.data[row][i];
                ptr::write_volatile(reference as *mut ScreenChar, blank);
            }
        }
    }

    /// Clears the entire VGA text buffer by blanking every row.
    ///
    /// Iterates over all [`BUFFER_HEIGHT`] rows and delegates to
    /// [`clear_row`](VgaDriver::clear_row) for each one.
    fn clear_screen(&mut self){
        for i in 0..BUFFER_HEIGHT {
            self.clear_row(i);
        }
    }
}