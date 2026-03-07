#[path = "vga/driver.rs"]
pub mod vga;

#[path = "writer/mod.rs"]
pub mod writer;

pub fn init() {
    vga::clear_screen();
}