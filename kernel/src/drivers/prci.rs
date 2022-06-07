#![allow(dead_code)]

use core::ptr::{read_volatile, write_volatile};

pub struct PRCI {
    pub hfrosccfg: HFROSCCFG,
    pub hfxosccfg: HFXOSCCFG,
    pub pllcfg: PLLCFG,
    pub plloutdiv: PLLOUTDIV,
    pub procmoncfg: PROCMONCFG,
}

impl PRCI {
    pub fn new(addr: *mut u32) -> Self {
        PRCI {
            hfrosccfg: HFROSCCFG::new(addr),
            hfxosccfg: HFXOSCCFG::new((addr as u32 + 0x04) as *mut u32),
            pllcfg: PLLCFG::new((addr as u32 + 0x08) as *mut u32),
            plloutdiv: PLLOUTDIV::new((addr as u32 + 0x0C) as *mut u32),
            procmoncfg: PROCMONCFG::new((addr as u32 + 0xF0) as *mut u32),
        }
    }
}

/// hfrosccfg: Ring Oscillator Configuration and Status
pub struct HFROSCCFG {
    ptr: *mut u32,
}

impl HFROSCCFG {
    const HFROSCDIV_MASK: u32 = 0b11111;
    const HFROSCTRIM_SHIFT: u32 = 16;
    const HFROSCTRIM_MASK: u32 = 0b11111 << Self::HFROSCTRIM_SHIFT;
    const HFROSCEN_SHIFT: u32 = 30;

    pub fn new(ptr: *mut u32) -> Self {
        HFROSCCFG { ptr }
    }

    /// Ring Oscillator Divider Register
    pub fn hfroscdiv(&self) -> u32 {
        unsafe { read_volatile(self.ptr) & Self::HFROSCDIV_MASK }
    }

    pub fn set_hfroscdiv(&mut self, value: u32) {
        unsafe {
            let original = read_volatile(self.ptr) & !Self::HFROSCDIV_MASK;
            write_volatile(self.ptr, original | value);
        }
    }

    /// Ring Oscillator Trim Register
    pub fn hfrosctrim(&self) -> u32 {
        unsafe {
            let value = read_volatile(self.ptr) & Self::HFROSCTRIM_MASK;
            value >> Self::HFROSCTRIM_SHIFT
        }
    }

    pub fn set_hfrosctrim(&mut self, value: u32) {
        unsafe {
            let original = read_volatile(self.ptr) & !Self::HFROSCTRIM_MASK;
            write_volatile(self.ptr, original | (value << Self::HFROSCTRIM_SHIFT));
        }
    }

    /// Ring Oscillator Enable
    pub fn hfroscen(&self) -> bool {
        unsafe { read_volatile(self.ptr) & (1 << Self::HFROSCEN_SHIFT) != 0 }
    }

    pub fn set_hfroscen(&mut self, value: bool) {
        let value = if value { 1 << Self::HFROSCEN_SHIFT } else { 0 };
        unsafe {
            let original = read_volatile(self.ptr);
            write_volatile(self.ptr, original | value);
        }
    }

    /// Ring Oscillator Ready
    pub fn hfroscrdy(&self) -> bool {
        unsafe { read_volatile(self.ptr) & (1 << 31) != 0 }
    }
}

/// hfxosccfg Crystal Oscillator Configuration and Status
pub struct HFXOSCCFG {
    ptr: *mut u32,
}

impl HFXOSCCFG {
    pub fn new(ptr: *mut u32) -> Self {
        HFXOSCCFG { ptr }
    }

    pub fn hfxoscen(&self) -> bool {
        unsafe { read_volatile(self.ptr) & (1 << 30) != 0 }
    }

    pub fn set_hfxoscen(&mut self, value: bool) {
        let value = if value { 1 << 30 } else { 0 };
        unsafe {
            let original = read_volatile(self.ptr);
            write_volatile(self.ptr, original | value);
        }
    }

    pub fn hfroscen(&self) -> bool {
        unsafe { read_volatile(self.ptr) & (1 << 31) != 0 }
    }
}

pub struct PLLCFG {
    ptr: *mut u32,
}

impl PLLCFG {
    pub fn new(ptr: *mut u32) -> Self {
        PLLCFG { ptr }
    }
}

pub struct PLLOUTDIV {
    ptr: *mut u32,
}

impl PLLOUTDIV {
    pub fn new(ptr: *mut u32) -> Self {
        PLLOUTDIV { ptr }
    }
}

pub struct PROCMONCFG {
    ptr: *mut u32,
}

impl PROCMONCFG {
    pub fn new(ptr: *mut u32) -> Self {
        PROCMONCFG { ptr }
    }
}
