#![allow(dead_code)]

use core::arch::asm;

pub fn wfi() {
    unsafe { asm!("wfi") }
}
