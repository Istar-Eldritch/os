#![no_std]
#![no_main]
#![feature(naked_functions)]

mod drivers;
mod hifive;
mod low;
mod macros;
mod riscv;
mod term;

use core::fmt::Write;
use core::panic::PanicInfo;
use drivers::gpio::*;
use drivers::prci::*;
use drivers::uart::*;
use hifive::*;
use riscv::{wfi, MStatus, Mie};
use term::Writer;

#[no_mangle]
pub fn main() {
    let gpio = GPIO::new(GPIO_ADDR);

    // Enable the leds
    gpio.output_en().set_all(LED_GREEN | LED_RED | LED_BLUE);
    gpio.out_xor().set_all(LED_GREEN | LED_RED | LED_BLUE);

    setup_clock();
    let uart = setup_uart0();
    let mut writer = Writer::new(uart);

    writer.write_str("Hello world!");
    // Turnon the green led

    //let mstatus = MStatus::read();
    //let value = mstatus.all();
     write!(writer, "mstatus").unwrap_or(());
    gpio.output_val().set_pin19(1);

    loop {
        writer.write_str("Looping!");
        wfi();
    }
}

pub fn setup_clock() {
    let prci = PRCI::new(PRCI_ADDR);

    // Divider for 14.4MHz
    prci.hfrosccfg().set_hfroscdiv(0x4);

    // TODO: Calibration should be read from the OTP
    // TODO: Test this with an oscilloscope
    // This calibration was done by trial and error
    prci.hfrosccfg().set_hfrosctrim(0x24);
    // Wait for the clock to be ready
    loop {
        if prci.hfrosccfg().hfroscrdy() == 1 {
            break;
        }
    }
}

pub fn setup_uart0() -> UART {
    // Enble UART GPIOs
    let gpio = GPIO::new(GPIO_ADDR);
    gpio.iof_en().set_all(UART0_PIN_RX | UART0_PIN_TX);
    gpio.iof_sel().set_all(0x0);

    let uart = UART::new(UART0_ADDR);
    uart.txctrl().set_txen(1);
    uart.rxctrl().set_rxen(1);
    // 115200 Baud from  a 14.4MHz clock
    uart.div().set_div(0x7c);
    uart
}

#[panic_handler]
fn panic(_er: &PanicInfo) -> ! {
    let gpio = GPIO::new(GPIO_ADDR);
    // Turn on the red led
    gpio.output_val().set_pin22(1);
    loop {}
}

#[no_mangle]
pub fn trap_handler() {
    let uart = UART::new(UART0_ADDR);
    let gpio = GPIO::new(GPIO_ADDR);
    // Turn on the red led
    gpio.output_val().set_pin22(1);
    Writer::new(uart).write_str("Interrupted!");
}
