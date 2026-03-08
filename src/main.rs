#![no_std]
#![no_main]

mod kernel;

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut writer = kernel::SCREEN_WRITER.lock();
    for &byte in HELLO.iter() {
        writer.write_char(byte as char);
    }


    loop {}
}
