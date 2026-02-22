use crate::const_vals::io::{USB_SERIAL_JTAG_EP1_CONF_REG, USB_SERIAL_JTAG_WR_DONE, USB_SERIAL_JTAG__FIFO};

pub unsafe fn write_to_serial_jtag(input: &str) {
    unsafe {

        for byte in input.as_bytes() {
            USB_SERIAL_JTAG__FIFO.write_volatile(*byte as u32);
        }
        USB_SERIAL_JTAG__FIFO.write_volatile(b'\n' as u32);

        USB_SERIAL_JTAG_EP1_CONF_REG.write_volatile(USB_SERIAL_JTAG_WR_DONE as u32);
    }
}