mod clock;
mod leds;
mod tty;

use crate::drivers::{gpio::Gpio, uart::Uart};
use crate::hifive::*;
use clock::Clock;
use leds::Leds;
use tty::{Tty, UartWriter};

static mut DEVICES: Option<Devices> = None;

pub struct Devices {
    pub tty: Tty,
    pub clock: Clock,
    pub leds: Leds,
}

impl Devices {
    pub unsafe fn init() {
        let core_clk: u32 = 2_073_600;
        let rtc: u32 = 32_000;
        // Setting up the kernel clock
        let mut clock = Clock::new(core_clk, rtc);
        clock.init();

        // Setting up the tty
        let mut gpio = Gpio::new(GPIO_ADDR);
        let mut uart_writer = UartWriter::new(Uart::new(UART0_ADDR));
        uart_writer.init(&mut gpio, &mut clock);
        let tty = Tty::new(uart_writer);

        let mut leds = Leds::new(gpio);
        leds.init();
        DEVICES = Some(Devices { tty, clock, leds })
    }

    pub fn get<'a>() -> &'a mut Self {
        unsafe { DEVICES.as_mut() }.unwrap()
    }
}
