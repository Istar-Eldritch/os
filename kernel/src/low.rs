use core::arch::asm;

#[naked]
#[no_mangle]
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
pub extern "C" fn _start() -> ! {
    unsafe {
        asm!(
            "
                csrw mie, 0
                csrw mip, 0

                li  x1, 0
                li  x2, 0
                li  x3, 0
                li  x4, 0
                li  x5, 0
                li  x6, 0
                li  x7, 0
                li  x8, 0
                li  x9, 0
                li  x10,0
                li  x11,0
                li  x12,0
                li  x13,0
                li  x14,0
                li  x15,0
                li  x16,0
                li  x17,0
                li  x18,0
                li  x19,0
                li  x20,0
                li  x21,0
                li  x22,0
                li  x23,0
                li  x24,0
                li  x25,0
                li  x26,0
                li  x27,0
                li  x28,0
                li  x29,0
                li  x30,0
                li  x31,0
            
                la sp, _stack_start
                lui t0, %hi(_hart_stack_size)
                add t0, t0, %lo(_hart_stack_size)
            
                la t0, _start_trap
                csrw mtvec, t0
                jal zero, main
    ",
            options(noreturn)
        )
    }
}

#[naked]
#[no_mangle]
#[cfg(target_arch = "riscv32")]
pub extern "C" fn _start_trap() -> ! {
    unsafe {
        asm!(
            "
                addi sp, sp, -16*(1<<2)
            
                sw ra, 0*(1 << 2)(sp)
                sw t0, 1*(1 << 2)(sp)
                sw t1, 2*(1 << 2)(sp)
                sw t2, 3*(1 << 2)(sp)
                sw t3, 4*(1 << 2)(sp)
                sw t4, 5*(1 << 2)(sp)
                sw t5, 6*(1 << 2)(sp)
                sw t6, 7*(1 << 2)(sp)
                sw a0, 8*(1 << 2)(sp)
                sw a1, 9*(1 << 2)(sp)
                sw a2, 10*(1 << 2)(sp)
                sw a3, 11*(1 << 2)(sp)
                sw a4, 12*(1 << 2)(sp)
                sw a5, 13*(1 << 2)(sp)
                sw a6, 14*(1 << 2)(sp)
                sw a7, 15*(1 << 2)(sp)
            
                add a0, sp, zero
                jal ra, trap_handler
            
                lw ra, 0*(1 << 2)(sp)
                lw t0, 1*(1 << 2)(sp)
                lw t1, 2*(1 << 2)(sp)
                lw t2, 3*(1 << 2)(sp)
                lw t3, 4*(1 << 2)(sp)
                lw t4, 5*(1 << 2)(sp)
                lw t5, 6*(1 << 2)(sp)
                lw t6, 7*(1 << 2)(sp)
                lw a0, 8*(1 << 2)(sp)
                lw a1, 9*(1 << 2)(sp)
                lw a2, 10*(1 << 2)(sp)
                lw a3, 11*(1 << 2)(sp)
                lw a4, 12*(1 << 2)(sp)
                lw a5, 13*(1 << 2)(sp)
                lw a6, 14*(1 << 2)(sp)
                lw a7, 15*(1 << 2)(sp)
            
                addi sp, sp, 16*(1<<2)
                mret         
            ",
            options(noreturn)
        )
    }
}

