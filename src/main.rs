#![no_std]
#![no_main]
mod drivers;
mod hifive;
mod macros;
mod riscv;
mod term;

use core::panic::PanicInfo;
use drivers::gpio::*;
use drivers::prci::*;
use drivers::uart::*;
use hifive::*;
use term::Writer;

fn main_loop() {
    loop {}
}

#[no_mangle]
pub fn _start() {
    let mut gpio = GPIO::new(GPIO_ADDR);
    // Turn on the green led
    gpio.set_output_enabled(LED_GREEN);
    gpio.set_output_value(LED_GREEN);

    setup_clock();
    setup_uart0();
    Writer::new(UART0_ADDR).write_str("Hello world!");
    main_loop();
}

pub fn setup_clock() {
    let mut prci = PRCI::new(PRCI_ADDR);

    // Divider for 14.4MHz
    prci.hfrosccfg.set_hfroscdiv(0x4);

    // TODO: Calibration should be read from the OTP
    // TODO: Test this with an oscilloscope
    // This calibration was done by trial and error
    prci.hfrosccfg.set_hfrosctrim(0x24);
    // Wait for the clock to be ready
    loop {
        if prci.hfrosccfg.hfroscrdy() {
            break;
        }
    }
}

#[panic_handler]
fn panic(_er: &PanicInfo) -> ! {
    let mut gpio = GPIO::new(GPIO_ADDR);
    // Turn on the red led
    gpio.set_output_enabled(LED_RED);
    gpio.set_output_value(LED_RED);
    loop {}
}
