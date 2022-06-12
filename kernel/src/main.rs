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

use core::panic::PanicInfo;
use drivers::clint::*;
use drivers::gpio::*;
use drivers::prci::*;
use hifive::*;
use riscv::{wfi, MCause, MStatus, Mepc, Mie};
use term::{init_term};

#[no_mangle]
pub fn _start() {

    setup_clock();
    enable_interrupts();
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

pub fn enable_interrupts() {
    let mut mstatus = MStatus::new();
    let mut mie = Mie::new();

    mie.set_mtie(1);
    mstatus.set_mie(1);

    mie.apply();
    mstatus.apply();

    let clint = Clint::new(CLINT_ADDR);
    // Triggers the first timer interrupt in 1s.
    clint.mtimecmp().set_time(32_768);
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

#[derive(Debug)]
#[allow(dead_code)]
struct Interrupt {
    time: u64,
    exception: bool,
    code: usize,
    pc: usize,
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
        // Turn on the red led
        let gpio = GPIO::new(GPIO_ADDR);
        gpio.output_val().set_all(0 | LED_RED);
        println!("HALTED!");
        // TODO HALT / Recover
        loop {}
    }
}
