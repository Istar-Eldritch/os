#![allow(dead_code)]

mod csr;

use core::arch::asm;
pub use csr::*;

pub fn wfi() {
    unsafe { asm!("wfi") }
}
