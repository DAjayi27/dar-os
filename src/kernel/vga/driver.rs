use crate::kernel::colors::{Color, ColorCode};
use crate::kernel::writer::Write;
use volatile::Volatile;
pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

#[repr(transparent)]
struct Buffer {
    data: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}
pub struct VgaDriver{
    col_pos:usize,
    row_pos:usize,
    curr_color: ColorCode,
    buffer: &'static mut Buffer,
}

// Safety: VgaDriver exclusively accesses the VGA hardware buffer at a fixed
// memory-mapped address (0xb8000). There is only ever one instance, protected
// by a Mutex, so sending it across threads is safe.
unsafe impl Send for VgaDriver {}

impl VgaDriver {

    pub const VGA_MIO_ADDR: usize = 0xb8000;




    fn write_new_line (&mut self){
        self.col_pos = 0;
        self.row_pos += 1;
    }

}
impl Write for VgaDriver {

     fn new() -> Self{
        let default_color = ColorCode::new(Color::Black,Color::White);
        return Self{
            row_pos: 0,
            col_pos: 0,
            curr_color: default_color,
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        }
    }

    fn write(&mut self) {
        // no-op: use write_char instead
    }

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

        self.buffer.data[self.row_pos][self.col_pos].write(screen_data);

        self.col_pos += 1;


    }

    fn clear_row(&mut self, row:usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.curr_color,
        };

        for i in 0..BUFFER_WIDTH {
            self.buffer.data[row][i].write(blank);
        }
    }

    fn clear_screen(&mut self){
        for i in 0..BUFFER_HEIGHT {
            self.clear_row(i);
        }
    }
}


