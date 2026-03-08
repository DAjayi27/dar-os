#![no_std]
#![no_main]

mod kernel;

use core::panic::PanicInfo;
use kernel::writer;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &str = "Hello World!";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {

  


    loop {}
}
