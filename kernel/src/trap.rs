
use crate::drivers::clint::*;
use crate::riscv::*;
use crate::hifive::*;
use crate::{print, println};
use crate::drivers::gpio::*;
use crate::clock::get_clock;
use crate::leds::{get_leds};

#[derive(Debug)]
#[allow(dead_code)]
struct Interrupt {
    time: u64,
    exception: bool,
    code: usize,
    pc: usize,
}

pub fn init_traps() {
    let mut mstatus = MStatus::new();
    let mut mie = Mie::new();

    mie.set_mtie(1);
    mstatus.set_mie(1);

    mie.apply();
    mstatus.apply();

    let clint = Clint::new(CLINT_ADDR);
    // Triggers the first timer interrupt in 1s.
    clint.mtimecmp().set_time(get_clock().get_rtc_out() as u64);
}

#[no_mangle]
pub fn trap_handler() {
    let mcause = MCause::new();
    let clint = Clint::new(CLINT_ADDR);
    let time: u64 = clint.mtime().get_time();

    // Timer Interrupt
    if mcause.code() as u32 == 7 {
        print!(".");
        // Trigger an interrupt in 1s if the clock runs at 32.768KHz
        // For some reason looks like this is using the AON block low freq clock, I still don't understand why its not using hf clock.
        // TODO: What clock is actually running the CPU?
        clint.mtimecmp().set_time(time + 32_768);
        return;
    }

    let i = Interrupt {
        time,
        exception: mcause.interrupt() != 1,
        code: mcause.code(),
        pc: Mepc::new().all(),
    };

    println!("Exception:\n{:?}", i);

    // HALT on Exceptions
    if mcause.interrupt() == 0 {
        let mut leds = get_leds();
        leds.set_green(false);
        leds.set_blue(false);
        leds.set_red(true);
        println!("HALTED!");
        // TODO HALT / Recover
        loop {}
    }
}
