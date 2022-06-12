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
mod leds;

use core::panic::PanicInfo;
use riscv::wfi;
use term::{init_term};
use trap::init_traps;
use clock::init_clock;
use leds::{init_leds, get_leds};

#[no_mangle]
pub fn _start() {

    let coreclk_freq = 2_073_600;
    init_clock(coreclk_freq);
    init_leds();
    init_traps();
    init_term();

    
    let mut leds = get_leds();
    leds.set_green(true);

    println!("\nKernel initialised");

    loop {
        wfi();
    }
}

#[panic_handler]
fn panic(er: &PanicInfo) -> ! {
    let mut leds = get_leds();
    println!("Panic!: \n{:?}", er);
    leds.set_green(false);
    leds.set_blue(false);
    leds.set_red(true);
    loop {}
}
