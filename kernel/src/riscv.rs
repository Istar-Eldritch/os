#![allow(dead_code)]

use core::arch::asm;
use register::*;

pub fn wfi() {
    unsafe { asm!("wfi") }
}

#[field(wllr, 0, 30)]
#[field(interrupt, 31, 31)]
pub struct MCause(*mut usize);

impl MCause {
    pub fn new() -> Self {
        let mut cause = 0;
        unsafe { asm!("lw {cause}, mcause", cause = inout(reg) cause) };
        MCause(&mut cause)
    }
}

#[field(mie, 3, 3)]
#[field(mpie, 7, 7)]
#[field(mpp, 11, 12)]
#[field(all, 0, 31)]
pub struct MStatus(*mut usize);

impl MStatus {
    pub fn new(v: usize) -> Self {
        let mut status = v;
        MStatus(&mut status)
    }

    pub fn read() -> usize {
        let mut status;
        unsafe { asm!("csrrs x0, mstatus, {status}", status = out(reg) status) };
        status
    }

    pub fn set(&mut self) {
        unsafe {
            let status = *self.0;
            asm!("csrrw {status}, mstatus, x0", status = in(reg) status);
        };
    }
}

pub struct Mie(*mut usize);

impl Mie {
    pub fn new() -> Self {
        let mut m = 0;
        unsafe { asm!("lw {m}, mie", m = inout(reg) m) };
        Mie(&mut m)
    }
}
