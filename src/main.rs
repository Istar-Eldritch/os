#![no_std]
#![no_main]
mod drivers;
mod hifive;
mod macros;
mod riscv;
mod uart;

use core::panic::PanicInfo;
use drivers::gpio::*;
use hifive::*;
use uart::*;

fn main_loop() {
    loop {}
}

#[no_mangle]
pub fn _start() {
    let mut gpio = GPIO::new(GPIO_ADDR);
    // Turn on the green led
    let enabled_leds = bit!(19);

    gpio.set_output_enabled(enabled_leds);

    setup_clock();
    setup_uart0();
    Writer::new().write_str("Hello world!");
    main_loop();
}

#[panic_handler]
fn panic(_er: &PanicInfo) -> ! {
    let mut gpio = GPIO::new(GPIO_ADDR);
    // Turn on the red led
    gpio.set_output_enabled(bit!(22));
    gpio.set_output_value(bit!(22));
    loop {}
}
