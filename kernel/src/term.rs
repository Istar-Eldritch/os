use crate::drivers::{gpio::*, uart::*};
use crate::hifive::*;
use core::fmt;

// TODO: Use a Mutex here
static mut WRITER: Option<Writer> = None;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::term::_print(format_args!($($arg)*)))
}

#[macro_export]
macro_rules! println {
    () => ($crate::term::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n",format_args!($($arg)*)))
}

pub fn _print(s: fmt::Arguments) {
    use fmt::Write;
    unsafe {
        WRITER.clone().map(|mut w| w.write_fmt(s).unwrap());
    }
}

pub fn init_term() {
    let uart = setup_uart();
    unsafe {
        WRITER = Some(Writer::new(uart));
    }
}

fn setup_uart() -> UART {
    // Enble UART GPIOs
    let gpio = GPIO::new(GPIO_ADDR);
    gpio.iof_en().set_all(UART0_PIN_RX | UART0_PIN_TX);
    gpio.iof_sel().set_all(0x0);

    let uart = UART::new(UART0_ADDR);
    uart.txctrl().set_txen(1);
    uart.rxctrl().set_rxen(1);
    // 115200 Baud from  a 2.0736MHz clock
    uart.div().set_div(2_073_600 / 115200 - 1);
    uart
}

#[derive(Clone)]
struct Writer {
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

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_char(byte);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
