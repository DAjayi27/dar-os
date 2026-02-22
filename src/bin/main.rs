#![no_std]
#![no_main]

use core::fmt::Write;
use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::delay::Delay;
use esp_hal::uart::{Uart, Config as UartConfig}; // <--- Import UART tools
use esp_hal::usb_serial_jtag::UsbSerialJtag;


const USB_SERIAL_JTAG_BASE: usize = 0x6004_3000;

const FIFO: *mut u32      = (USB_SERIAL_JTAG_BASE + 0x00) as *mut u32;
const INT_RAW: *const u32 = (USB_SERIAL_JTAG_BASE + 0x20) as *const u32;

const SERIAL_IN_EP1_EMPTY: u32 = 1 << 1;

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

    loop {
        // 2. NOW your unsafe write will work because the hardware is on!
        unsafe {
            write_to_serial_jtag();
        }



        delay.delay_millis(500);
    }


}

// Your custom raw function
unsafe fn write_to_serial_jtag() {


    let serial_jtag = 0x6004_3000 as *mut u32;

    serial_jtag.write_volatile(b'A' as u32);


    const USB_SERIAL_JTAG_EP1_CONF_REG: *mut u32 =  0x6004_3004 as *mut u32;

    const  USB_SERIAL_JTAG_WR_DONE:u8 = 0b001;
    unsafe {  USB_SERIAL_JTAG_EP1_CONF_REG.write_volatile(USB_SERIAL_JTAG_WR_DONE as u32) }

}
