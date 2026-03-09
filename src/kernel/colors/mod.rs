/// Represents one of the 16 standard VGA text-mode colours.
///
/// The enum uses `#[repr(u8)]` so each variant maps directly to the
/// 4-bit colour value expected by the VGA text buffer attribute byte.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    /// Black (`0x0`).
    Black = 0,
    /// Blue (`0x1`).
    Blue = 1,
    /// Green (`0x2`).
    Green = 2,
    /// Cyan (`0x3`).
    Cyan = 3,
    /// Red (`0x4`).
    Red = 4,
    /// Magenta (`0x5`).
    Magenta = 5,
    /// Brown (`0x6`).
    Brown = 6,
    /// Light gray (`0x7`).
    LightGray = 7,
    /// Dark gray (`0x8`).
    DarkGray = 8,
    /// Light blue (`0x9`).
    LightBlue = 9,
    /// Light green (`0xA`).
    LightGreen = 10,
    /// Light cyan (`0xB`).
    LightCyan = 11,
    /// Light red (`0xC`).
    LightRed = 12,
    /// Pink (`0xD`).
    Pink = 13,
    /// Yellow (`0xE`).
    Yellow = 14,
    /// White (`0xF`).
    White = 15,
}

/// A packed VGA text-mode colour attribute.
///
/// The inner `u8` stores the background colour in the high 4 bits and the
/// foreground colour in the low 4 bits. `#[repr(transparent)]` ensures
/// the wrapper has the same layout as its underlying byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    /// Creates a VGA colour attribute from foreground and background colours.
    ///
    /// # Parameters
    /// * `foreground` - The text colour stored in the low 4 bits.
    /// * `background` - The background colour stored in the high 4 bits.
    ///
    /// # Returns
    /// A `ColorCode` containing the packed VGA attribute byte.
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}