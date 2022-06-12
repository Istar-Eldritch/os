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
mod clock;

use core::panic::PanicInfo;
use drivers::gpio::*;
use hifive::*;
use riscv::wfi;
use term::{init_term};
use trap::init_traps;
use clock::init_clock;

#[no_mangle]
pub fn _start() {

    let coreclk_freq = 2_073_600;
    init_clock(coreclk_freq);
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

#[panic_handler]
fn panic(_er: &PanicInfo) -> ! {
    let gpio = GPIO::new(GPIO_ADDR);
    // Turn on the red led
    gpio.output_val().set_pin22(1);
    loop {}
}
