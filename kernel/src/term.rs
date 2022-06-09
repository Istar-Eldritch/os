use crate::drivers::uart::*;
use core::fmt;

pub struct Writer {
    uart: UART,
}

impl Writer {
    pub fn new(uart: UART) -> Self {
        Writer { uart }
    }
    pub fn write_char(&mut self, c: u8) {
        loop {
            if self.uart.txdata().full() == 0 {
                break;
            }
        }
        self.uart.txdata().set_data(c as usize);
    }

    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_char(byte);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}
