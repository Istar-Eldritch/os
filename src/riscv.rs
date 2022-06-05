use core::arch::asm;

pub fn wfi() {
    unsafe { asm!("wfi") }
}
