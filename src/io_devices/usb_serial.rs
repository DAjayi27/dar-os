use core::hint::spin_loop;
use core::ptr::read_volatile;
use core::str::from_utf8;
use esp_println::{print, println};
use crate::const_vals::io::{SERIAL_IN_EP_DATA_FREE, SERIAL_OUT_EP_DATA_AVAIL_BIT, USB_SERIAL_JTAG_EP1_CONF_REG, USB_SERIAL_JTAG_JFIFO_ST_REG, USB_SERIAL_JTAG_WR_DONE, USB_SERIAL_JTAG__FIFO};
use crate::io_devices::LedStatus;

pub unsafe fn write_to_serial_jtag(input: &str) {
    unsafe {

        for byte in input.as_bytes() {
            USB_SERIAL_JTAG__FIFO.write_volatile(*byte as u32);
        }
        USB_SERIAL_JTAG__FIFO.write_volatile(b'\n' as u32);

        USB_SERIAL_JTAG_EP1_CONF_REG.write_volatile(USB_SERIAL_JTAG_WR_DONE as u32);

        wait_for_fifo_to_be_flushed();
    }
}

unsafe fn usb_serial_jtag_fifo_data_available_to_read() -> bool{
    unsafe {
        return (USB_SERIAL_JTAG_EP1_CONF_REG.read_volatile() & (1u32 << SERIAL_OUT_EP_DATA_AVAIL_BIT)) != 0;
    }
}

unsafe fn fnusb_serial_jtag_fifo_is_flushed() -> bool{
    unsafe {
        return  (USB_SERIAL_JTAG_EP1_CONF_REG.read_volatile() & (1u32 << SERIAL_IN_EP_DATA_FREE)) != 0;
    }
}

unsafe fn wait_for_fifo_to_be_flushed() {
    unsafe {
        while !fnusb_serial_jtag_fifo_is_flushed() {
            spin_loop()
        }
    }
}

pub unsafe fn read_from_serial_jtag(buffer: &mut [u8]) -> &str {
    let mut index = 0;

    loop {
        while !usb_serial_jtag_fifo_data_available_to_read() {
            core::hint::spin_loop();
        }

        let data = USB_SERIAL_JTAG__FIFO.read_volatile() as u8;

        if data == b'\n' || data == b'\r' {
            print!("{}",'\n');
            break;
        }

        if index < buffer.len() {
            buffer[index] = data;
            index += 1;
        } else {
            break; // Buffer full, stop reading
        }

        print!("{}",data as char);

    }

    match core::str::from_utf8(&buffer[0..index]) {
        Ok(string) => string.trim(), // trim removes accidental extra spaces
        Err(_) => "Error",
    }
}

