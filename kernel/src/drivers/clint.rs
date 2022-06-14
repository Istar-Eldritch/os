#![allow(dead_code)]

use register::*;

#[register(msip, Msip, 0)]
#[register(mtimecmp, MTime, 0x4000)]
#[register(mtime, MTime, 0xbff8)]
pub struct Clint {
    addr: *mut usize,
}

impl Clint {
    pub fn new(addr: *mut usize) -> Self {
        Clint { addr }
    }
}

#[field(all, 0, 31)]
pub struct Msip {
    addr: *mut usize,
}

#[register(low, Time, 0)]
#[register(high, Time, 0x4)]
pub struct MTime {
    addr: *mut usize,
}

impl MTime {
    pub fn get_time(&self) -> u64 {
        self.into()
    }
    pub fn set_time(&mut self, time: u64) {
        self.high().set_all((time >> 31) as usize);
        self.low().set_all(time as usize);
    }
}

impl From<&MTime> for u64 {
    fn from(mtime: &MTime) -> u64 {
        let time: u64 = (mtime.high().all() as u64) << 31;
        time + mtime.low().all() as u64
    }
}

#[field(all, 0, 31)]
pub struct Time {
    addr: *mut usize,
}
