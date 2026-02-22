pub const USB_SERIAL_JTAG_BASE: usize = 0x6004_3000;
pub const USB_SERIAL_JTAG__FIFO: *mut u32      = (USB_SERIAL_JTAG_BASE + 0x00) as *mut u32;
pub const USB_SERIAL_JTAG_EP1_CONF_REG: *mut u32 =  0x6004_3004 as *mut u32;
pub const  USB_SERIAL_JTAG_WR_DONE:u8 = 0b001;
pub const USB_SERIAL_JTAG_JFIFO_ST_REG: *mut u32 = (USB_SERIAL_JTAG_BASE + 0x0020) as *mut u32;
pub const SERIAL_OUT_EP_DATA_AVAIL_BIT: u32 = 2;
