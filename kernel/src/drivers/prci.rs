#![allow(dead_code)]

use register::*;

#[register(hfrosccfg, HfroscCfg, 0x0)]
#[register(hfxosccfg, HfxoscCfg, 0x4)]
#[register(pllcfg, PllCfg, 0x8)]
#[register(plloutdiv, PlloutDiv, 0xC)]
#[register(procmoncfg, ProcMonCfg, 0xF0)]
pub struct Prci {
    addr: *mut usize,
}

impl Prci {
    pub fn new(addr: *mut usize) -> Self {
        Prci { addr }
    }
}

/// Internal Trimmable Programmable 72 MHz Oscillator (HFROSC)
#[field(hfroscdiv, 0, 5)]
#[field(hfrosctrim, 16, 20)]
#[field(hfroscen, 30, 30)]
#[field(hfroscrdy, 31, 31)]
pub struct HfroscCfg {
    addr: *mut usize,
}

impl HfroscCfg {
    const FREQ: u32 = 72_000_000;

    pub fn set_freq(&mut self, freq: u32) {
        let div = HfroscCfg::FREQ / freq;

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
pub struct HfxoscCfg {
    addr: *mut usize,
}

/// Internal High-Frequency PLL (HFPLL)
#[field(pllr, 0, 2)]
#[field(pllf, 4, 9)]
#[field(pllq, 10, 11)]
#[field(pllsel, 16, 16)]
#[field(pllrefsel, 17, 17)]
#[field(pllbypass, 18, 18)]
#[field(plllock, 31, 31)]
pub struct PllCfg {
    addr: *mut usize,
}

/// PLL Output Divider
#[field(plloutdiv, 0, 5)]
#[field(plloutdivby1, 8, 8)]
pub struct PlloutDiv {
    addr: *mut usize,
}

#[field(all, 0, 31)]
pub struct ProcMonCfg {
    addr: *mut usize,
}
