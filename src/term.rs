use crate::bit;
use crate::drivers::uart::*;
use core::ptr::{read_volatile, write_volatile};

pub struct Writer {
    data: *mut u32,
}

impl Writer {
    pub fn new(uart_addr: *mut u32) -> Self {
        Writer {
            data: (uart_addr as u32 + UART_TXDATA) as *mut u32,
        }
    }
    pub fn write_char(&mut self, c: char) {
        unsafe {
            loop {
                if read_volatile(self.data) & bit!(31) == 0 {
                    break;
                }
            }
            write_volatile(self.data, c as u32)
        }
    }

    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_char(byte as char);
        }
    }
}
