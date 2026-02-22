use esp_println::print;
use crate::const_vals::io::{SERIAL_OUT_EP_DATA_AVAIL_BIT, USB_SERIAL_JTAG_EP1_CONF_REG, USB_SERIAL_JTAG_JFIFO_ST_REG, USB_SERIAL_JTAG_WR_DONE, USB_SERIAL_JTAG__FIFO};
use crate::io_devices::LedStatus;

pub unsafe fn write_to_serial_jtag(input: &str) {
    unsafe {

        for byte in input.as_bytes() {
            USB_SERIAL_JTAG__FIFO.write_volatile(*byte as u32);
        }
        USB_SERIAL_JTAG__FIFO.write_volatile(b'\n' as u32);

        USB_SERIAL_JTAG_EP1_CONF_REG.write_volatile(USB_SERIAL_JTAG_WR_DONE as u32);
    }
}

unsafe fn usb_serial_jtag_fifo_data_available_to_read() -> bool{
    unsafe {
        return (USB_SERIAL_JTAG_EP1_CONF_REG.read_volatile() & (1u32 << SERIAL_OUT_EP_DATA_AVAIL_BIT)) != 0;
    }
}

pub unsafe fn read_from_serial_jtag() -> LedStatus{




    return LedStatus::OFF;

}

