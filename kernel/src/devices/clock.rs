use crate::drivers::prci::*;
use crate::hifive::*;

#[derive(Clone)]
pub struct Clock {
    coreclk_out: u32,
    rtc_out: u32,
}

impl Clock {
    
    pub fn new(coreclk: u32, rtc: u32) -> Self {
        Self {
            coreclk_out: coreclk,
            rtc_out: rtc
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
        prci.hfrosccfg().set_freq(self.coreclk_out);

        // Wait for the clock to be ready
        loop {
            if prci.hfrosccfg().hfroscrdy() == 1 {
                break;
            }
        }
    }
}
