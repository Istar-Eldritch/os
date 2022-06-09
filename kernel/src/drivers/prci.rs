#![allow(dead_code)]

use register::*;

#[register(hfrosccfg, HFROSCCFG, 0x0)]
#[register(hfxosccfg, HFXOSCCFG, 0x4)]
#[register(pllcfg, PLLCFG, 0x8)]
#[register(plloutdiv, PLLOUTDIV, 0xC)]
#[register(procmoncfg, PROCMONCFG, 0xF0)]
pub struct PRCI(*mut usize);

impl PRCI {
    pub fn new(addr: *mut usize) -> Self {
        PRCI(addr)
    }
}

/// Internal Trimmable Programmable 72 MHz Oscillator (HFROSC)
#[field(hfroscdiv, 0, 5)]
#[field(hfrosctrim, 16, 20)]
#[field(hfroscen, 30, 30)]
#[field(hfroscrdy, 31, 31)]
pub struct HFROSCCFG(*mut usize);

impl HFROSCCFG {
    const FREQ: u32 = 72_000_000;

    pub fn new(ptr: *mut usize) -> Self {
        HFROSCCFG(ptr)
    }

    pub fn set_freq(&mut self, freq: u32) {
        let div = HFROSCCFG::FREQ / freq;

        // TODO: Calibration should be read from the OTP
        // TODO: Test this with an oscilloscope
        // This calibration was done by trial and error
        self.set_hfrosctrim(4);
        self.set_hfroscdiv((div - 1) as usize);
    }
}

/// External 16 MHz Crystal Oscillator (HFXOSC)
#[field(hfxoscen, 30, 30)]
#[field(hfxoscrdy, 31, 31)]
pub struct HFXOSCCFG(*mut usize);

impl HFXOSCCFG {
    pub fn new(ptr: *mut usize) -> Self {
        HFXOSCCFG(ptr)
    }
}

/// Internal High-Frequency PLL (HFPLL)
#[field(pllr, 0, 2)]
#[field(pllf, 4, 9)]
#[field(pllq, 10, 11)]
#[field(pllsel, 16, 16)]
#[field(pllrefsel, 17, 17)]
#[field(pllbypass, 18, 18)]
#[field(plllock, 31, 31)]
pub struct PLLCFG(*mut usize);

impl PLLCFG {
    pub fn new(ptr: *mut usize) -> Self {
        PLLCFG(ptr)
    }
}

/// PLL Output Divider
#[field(plloutdiv, 0, 5)]
#[field(plloutdivby1, 8, 8)]
pub struct PLLOUTDIV(*mut usize);

impl PLLOUTDIV {
    pub fn new(ptr: *mut usize) -> Self {
        PLLOUTDIV(ptr)
    }
}

#[field(all, 0, 31)]
pub struct PROCMONCFG(*mut usize);

impl PROCMONCFG {
    pub fn new(ptr: *mut usize) -> Self {
        PROCMONCFG(ptr)
    }
}
