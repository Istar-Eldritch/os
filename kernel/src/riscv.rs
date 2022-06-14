#![allow(dead_code)]

use core::arch::asm;
use register::*;

pub fn wfi() {
    unsafe { asm!("wfi") }
}

#[link_section = ".bss"]
static mut MSTATUS: usize = 0;
#[link_section = ".bss"]
static mut MIE: usize = 0;
#[link_section = ".bss"]
static mut MCAUSE: usize = 0;
#[link_section = ".bss"]
static mut MEPC: usize = 0;

#[field(mie, 3, 3)]
#[field(mpie, 7, 7)]
#[field(mpp, 11, 12)]
#[field(all, 0, 31)]
pub struct MStatus{ addr: *mut usize }

impl MStatus {
    pub fn new() -> Self {
        unsafe {
            let mut mstatus = MStatus { addr: &mut MSTATUS };
            mstatus.reload();
            mstatus
        }
    }

    pub fn reload(&mut self) {
        unsafe { asm!("csrr {s}, mstatus", s = out(reg) MSTATUS) };
    }

    pub fn apply(&mut self) {
        unsafe {
            asm!("csrrw x0, mstatus, {s}", s = in(reg) MSTATUS);
        };
    }
}

#[field(msie, 3, 3)]
#[field(mtie, 7, 7)]
#[field(meie, 11, 11)]
pub struct Mie{ addr: *mut usize }

impl Mie {
    pub fn new() -> Self {
        unsafe {
            let mut mie = Mie{ addr: &mut MIE } ;
            mie.reload();
            mie
        }
    }

    pub fn reload(&mut self) {
        unsafe { asm!("csrr {m}, mie", m = out(reg) MIE) };
    }

    pub fn apply(&mut self) {
        unsafe {
            asm!("csrrw x0, mie, {m}", m = in(reg) MIE);
        }
    }
}

#[field(code, 0, 9)]
#[field(interrupt, 31, 31)]
#[field(all, 0, 31)]
pub struct MCause{ addr: *mut usize}

impl MCause {
    pub fn new() -> Self {
        let mut mcause = unsafe { MCause { addr: &mut MCAUSE} };
        mcause.reload();
        mcause
    }

    pub fn reload(&mut self) {
        unsafe { asm!("csrr {m}, mcause", m = out(reg) MCAUSE) };
    }
}

#[field(all, 0, 31)]
pub struct Mepc{ addr: *mut usize }

impl Mepc {
    pub fn new() -> Self {
        let mut pc = unsafe { Mepc{ addr: &mut MEPC} };
        pc.reload();
        pc
    }

    pub fn reload(&mut self) {
        unsafe { asm!("csrr {m}, mepc", m = out(reg) MEPC) };
    }
}
