#![no_std]
#![no_main]

mod const_vals;
mod io_devices;

use core::fmt::Write;
use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::delay::Delay;
use esp_hal::usb_serial_jtag::UsbSerialJtag;
use esp_hal::gpio::{Output, Io, Level,OutputConfig};
use esp_hal::ledc::{LSGlobalClkSource, Ledc};
use esp_println::{print, println};
use crate::io_devices::usb_serial::{read_from_serial_jtag, write_to_serial_jtag};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

const MAX_BUFFER_SIZE:usize = 64;

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let delay = Delay::new();

    let mut usb = UsbSerialJtag::new(peripherals.USB_DEVICE);

    let ledPinConfig = OutputConfig::default();

    let mut ledPin = Output::new(peripherals.GPIO5, Level::Low, ledPinConfig);

    let mut inputBuffer:[u8;MAX_BUFFER_SIZE] = [0;MAX_BUFFER_SIZE];

    let mut inputString: &str;


    unsafe {
        write_to_serial_jtag("BLINKY IS SETUPBlink test");
        write_to_serial_jtag("Type y to turn led on");
        write_to_serial_jtag("Type n to turn led off");
    }

    
    loop {



        unsafe {
          inputString =  read_from_serial_jtag(&mut inputBuffer);
        }

        if inputString.eq_ignore_ascii_case("on") {
            ledPin.set_high();
        }

        if inputString.eq_ignore_ascii_case("off") {
            ledPin.set_low();
        }

        delay.delay_millis(500);

    }


}



