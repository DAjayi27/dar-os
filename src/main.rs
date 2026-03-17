#![no_std]
#![no_main]

mod kernel;
use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {


    loop {
        serial_println!("Serial Out Test");
    }
}
