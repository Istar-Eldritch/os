#![no_std]
#![no_main]
pub mod gpio;
pub mod macros;
pub mod riscv;
pub mod uart;

use crate::gpio::*;
use core::panic::PanicInfo;
use core::ptr::write_volatile;
use uart::*;

fn main_loop() {
    loop {}
}

#[no_mangle]
pub fn _start() {
    // Turn on the green led
    let enabled_leds = bit!(19);
    unsafe {
        write_volatile(GPIO_OUT_EN, enabled_leds);
        write_volatile(GPIO_OUT_XOR, enabled_leds);
        write_volatile(GPIO_OUT_VAL, enabled_leds);
    }

    setup_clock();
    setup_uart0();
    Writer::new().write_str("ñoño!");
    main_loop();
}

#[panic_handler]
fn panic(_er: &PanicInfo) -> ! {
    // Turn on the red led
    let enabled_leds = bit!(22);
    unsafe {
        write_volatile(GPIO_OUT_EN, enabled_leds);
        write_volatile(GPIO_OUT_XOR, enabled_leds);
        write_volatile(GPIO_OUT_VAL, enabled_leds);
    }
    loop {}
}
