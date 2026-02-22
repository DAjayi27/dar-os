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
use crate::io_devices::usb_serial::write_to_serial_jtag;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let delay = Delay::new();

    let mut usb = UsbSerialJtag::new(peripherals.USB_DEVICE);

    let ledPinConfig = OutputConfig::default();

    let mut ledPin = Output::new(peripherals.GPIO5, Level::Low, ledPinConfig);


    unsafe {
        write_to_serial_jtag("Pin Set High");
    }


    loop {

        ledPin.set_high();


        let data = "Test";

        unsafe {
            write_to_serial_jtag(data);
        }



        delay.delay_millis(500);

        ledPin.set_low();
    }


}



