const VGA_MIO_ADDR: usize = 0xb8000;

pub fn clear_screen() {
    let vga_buffer = VGA_MIO_ADDR as *mut u8;

    unsafe {
        for i in 0..80 * 25 * 2 {
            *vga_buffer.add(i) = 0;
        }
    }
}
