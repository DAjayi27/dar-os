use lazy_static::lazy_static;
use spin::Mutex;
use crate::kernel::vga::driver::VgaDriver;
use crate::kernel::writer::Writer;

pub mod vga;

pub mod writer;
mod colors;

lazy_static! {
    pub static ref SCREEN_WRITER: Mutex<Writer<VgaDriver>> = Mutex::new(Writer::new());
}

fn init(){
}