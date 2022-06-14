use crate::hifive::*;
use crate::clock::Clock;
use core::fmt;
use crate::devices::*;

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

pub fn init() {
    unsafe {
        WRITER = Some(Writer::new());
    }
}

fn setup_uart() {
    // Enble UART GPIOs
    let gpio = unsafe {GPIO.as_mut()}.unwrap();
    gpio.iof_en().set_all(UART0_PIN_RX | UART0_PIN_TX);
    gpio.iof_sel().set_all(0x0);

    let uart = unsafe {UART0.as_mut()}.unwrap();
    uart.txctrl().set_txen(1);
    uart.rxctrl().set_rxen(1);
    // 115200 Baud from  a 2.0736MHz clock
    uart.div().set_div(Clock::get().get_coreclk_out() as usize / 115200 - 1);
}

#[derive(Clone)]
struct Writer {
}

impl Writer {
    pub fn new() -> Self {
        Writer { }
    }
    pub fn write_char(&mut self, c: u8) {
        let uart = unsafe {UART0.as_mut()}.unwrap();
        loop {
            if uart.txdata().full() == 0 {
                break;
            }
        }
        uart.txdata().set_data(c as usize);
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
