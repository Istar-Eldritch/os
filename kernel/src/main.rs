#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(const_mut_refs)]

mod drivers;
mod hifive;
mod low;
mod macros;
mod riscv;
mod trap;
mod devices;

use core::panic::PanicInfo;
use riscv::wfi;
use trap::init_traps;
use devices::Devices;

#[no_mangle]
pub fn _start() {
    unsafe {
        Devices::init();
    }

    init_traps();

    
    let d = Devices::get();
    d.leds.set_green(true);

    println!("\nKernel initialised");

    loop {
        wfi();
    }
}

#[panic_handler]
fn panic(er: &PanicInfo) -> ! {
    let d = Devices::get();
    println!("Panic!: \n{:?}", er);
    d.leds.set_green(false);
    d.leds.set_blue(false);
    d.leds.set_red(true);
    loop {}
}
