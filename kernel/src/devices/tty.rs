use crate::devices::*;
use core::fmt;

// TODO: Make this use a generic writer once we have an allocator
pub struct Tty {
    writer: UartWriter,
}

impl Tty {
    pub fn new(writer: UartWriter) -> Tty {
        Tty { writer }
    }
}

impl fmt::Write for Tty {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.writer.write_str(s)
    }
}

#[derive(Clone)]
pub struct UartWriter {
    uart: Uart,
}

impl UartWriter {
    pub fn new(uart: Uart) -> Self {
        UartWriter { uart }
    }
    pub fn write_char(&mut self, c: u8) {
        loop {
            if self.uart.txdata().full() == 0 {
                break;
            }
        }
        self.uart.txdata().set_data(c as usize);
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_char(byte);
        }
    }

    // TODO: This should specify the specific pins
    pub fn init(&mut self, gpio: &mut Gpio, clock: &Clock) {
        // Enble UART GPIOs
        gpio.iof_en().set_all(UART0_PIN_RX | UART0_PIN_TX);
        gpio.iof_sel().set_all(0x0);

        self.uart.txctrl().set_txen(1);
        self.uart.rxctrl().set_rxen(1);
        // 115200 Baud from the coreclock
        self.uart.div()
            .set_div(clock.get_coreclk_out() as usize / 115200 - 1);
    }
}

impl fmt::Write for UartWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
