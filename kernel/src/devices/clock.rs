use crate::devices::*;
use crate::drivers::{clint::*, prci::*};
use crate::hifive::*;
use crate::print;
use crate::riscv::*;
use crate::trap::*;

#[derive(Clone)]
pub struct Clock {
    coreclk_out: u32,
    rtc_out: u32,
}

impl Clock {
    pub fn new(coreclk: u32, rtc: u32) -> Self {
        Self {
            coreclk_out: coreclk,
            rtc_out: rtc,
        }
    }

    pub fn get_coreclk_out(&self) -> u32 {
        self.coreclk_out
    }

    pub fn get_rtc_out(&self) -> u32 {
        self.rtc_out
    }

    pub fn init(&mut self) {
        let prci = Prci::new(PRCI_ADDR);

        // Set coreclock
        prci.hfrosccfg().set_freq(self.coreclk_out);

        // Wait for the clock to be ready
        loop {
            if prci.hfrosccfg().hfroscrdy() == 1 {
                break;
            }
        }
    }

    pub fn enable_timer_interrupt(&self) {
        // Enable timer interrupts

        TrapManager::get_mut().register_interrupt_handler(
            InterruptCode::MachineTimerInterrupt,
            |_| {
                print!(".");
                let clint = Clint::new(CLINT_ADDR);
                // Triggers the next timer interrupt in 1s.
                clint
                    .mtimecmp()
                    .set_time(clint.mtime().get_time() + Devices::get().clock.get_rtc_out() as u64);
            },
        );

        let mut mstatus = MStatus::new();
        let mut mie = Mie::new();

        mie.set_mtie(1);
        mstatus.set_mie(1);

        mie.apply();
        mstatus.apply();

        let clint = Clint::new(CLINT_ADDR);
        // Triggers the first timer interrupt in 1s.
        clint
            .mtimecmp()
            .set_time(Devices::get().clock.get_rtc_out() as u64);
    }
}
