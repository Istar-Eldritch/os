#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(const_mut_refs)]

mod drivers;
mod hifive;
mod low;
mod macros;
mod riscv;
mod term;
mod trap;

use core::panic::PanicInfo;
use drivers::gpio::*;
use drivers::prci::*;
use hifive::*;
use riscv::wfi;
use term::{init_term};
use trap::init_traps;

#[no_mangle]
pub fn _start() {

    setup_clock();
    init_traps();
    init_term();

    let gpio = GPIO::new(GPIO_ADDR);

    // Enable the leds
    gpio.output_en().set_all(LED_GREEN | LED_RED | LED_BLUE);
    gpio.out_xor().set_all(LED_GREEN | LED_RED | LED_BLUE);
   
    // Turn on the green led
    gpio.output_val().set_pin19(1);

    println!("\nKernel initialised");
    loop {
        wfi();
    }
}

pub fn setup_clock() {
    let prci = PRCI::new(PRCI_ADDR);
    // Set frequency to 2.0736MHz
    prci.hfrosccfg().set_freq(2_073_600);

    // Wait for the clock to be ready
    loop {
        if prci.hfrosccfg().hfroscrdy() == 1 {
            break;
        }
    }
}

#[panic_handler]
fn panic(_er: &PanicInfo) -> ! {
    let gpio = GPIO::new(GPIO_ADDR);
    // Turn on the red led
    gpio.output_val().set_pin22(1);
    loop {}
}
