
use crate::drivers::prci::*;
use crate::hifive::*;

// TODO: Mutex here
static mut CLOCK: Option<Clock> = None;

#[derive(Clone)]
pub struct Clock {
    coreclk_out: u32,
    rtc_out: u32,
}

impl Clock {
    pub fn get_coreclk_out(&self) -> u32 {
       self.coreclk_out
    }
    
    pub fn get_rtc_out(&self) -> u32 {
       self.rtc_out 
    }
}


pub fn init_clock(coreclk_freq: u32) {
    let prci = PRCI::new(PRCI_ADDR);
    // Set frequency to 2.0736MHz
    prci.hfrosccfg().set_freq(coreclk_freq);

    // Wait for the clock to be ready
    loop {
        if prci.hfrosccfg().hfroscrdy() == 1 {
            break;
        }
    }
    
    unsafe {
        CLOCK = Some(Clock { coreclk_out: coreclk_freq, rtc_out: 32_768})
    }
}

pub fn get_clock() -> Clock {
    unsafe {CLOCK.clone().unwrap()}
}