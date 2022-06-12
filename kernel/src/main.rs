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

use core::fmt::Write;
use core::panic::PanicInfo;
use drivers::clint::*;
use drivers::gpio::*;
use drivers::prci::*;
use drivers::uart::*;
use hifive::*;
use riscv::{wfi, MCause, MStatus, Mepc, Mie};
use term::Writer;

#[no_mangle]
pub fn _start() {
    let gpio = GPIO::new(GPIO_ADDR);

    // Enable the leds
    gpio.output_en().set_all(LED_GREEN | LED_RED | LED_BLUE);
    gpio.out_xor().set_all(LED_GREEN | LED_RED | LED_BLUE);

    setup_clock();
    let uart = setup_uart0();
    let mut writer = Writer::new(uart);

    enable_interrupts();

    // Turn on the green led
    gpio.output_val().set_pin19(1);

    loop {
        let time: u64 = Clint::new(CLINT_ADDR).mtime().get_time();
        writeln!(writer, "Time @ {}", time).unwrap();

        wfi();
        writer.write_str("Looping!\n");
    }
}

pub fn enable_interrupts() {
    let uart = setup_uart0();
    let mut writer = Writer::new(uart);

    writeln!(writer, "Enabling interrupts").unwrap();
    let mut mstatus = MStatus::new();
    let mut mie = Mie::new();

    mie.set_mtie(1);
    mstatus.set_mie(1);

    mie.apply();
    mstatus.apply();

    let clint = Clint::new(CLINT_ADDR);

    writeln!(
        writer,
        "status_mie: {:b} -- mie_mtie: {:b}",
        mstatus.mie(),
        mie.mtie()
    )
    .unwrap();

    // Triggers an intterupt inmediatly
    clint.mtimecmp().set_time(0);
    writeln!(writer, "Interrupts enabled").unwrap();
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

pub fn setup_uart0() -> UART {
    // Enble UART GPIOs
    let gpio = GPIO::new(GPIO_ADDR);
    gpio.iof_en().set_all(UART0_PIN_RX | UART0_PIN_TX);
    gpio.iof_sel().set_all(0x0);

    let uart = UART::new(UART0_ADDR);
    uart.txctrl().set_txen(1);
    uart.rxctrl().set_rxen(1);
    // 115200 Baud from  a 2.0736MHz clock
    uart.div().set_div(2_073_600 / 115200 - 1);
    uart
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
    let uart = UART::new(UART0_ADDR);
    let mut writer = Writer::new(uart);
    let clint = Clint::new(CLINT_ADDR);
    let time: u64 = clint.mtime().get_time();

    // Timer Interrupt
    if mcause.code() as u32 == 7 {
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

    writeln!(writer, "Exception:\n{:?}", i).unwrap();

    // HALT on Exceptions
    if mcause.interrupt() == 0 {
        // Turn on the red led
        let gpio = GPIO::new(GPIO_ADDR);
        gpio.output_val().set_all(0 | LED_RED);
        writeln!(writer, "HALTED!").unwrap();
        // TODO HALT / Recover
        loop {}
    }
}
