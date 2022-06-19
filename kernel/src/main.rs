#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(const_mut_refs)]

mod devices;
mod drivers;
mod hifive;
mod low;
mod macros;
mod riscv;
mod trap;

use core::panic::PanicInfo;
use devices::Devices;
use riscv::*;
use trap::TrapManager;

#[no_mangle]
pub fn _start() {
    unsafe {
        TrapManager::init();
        Devices::init();
    }
    
    let d = Devices::get();
    d.clock.enable_timer_interrupt();

    d.leds.set_green(true);

    println!("\n\rKernel initialised");
  
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
