OUTPUT_ARCH(riscv)

ENTRY(_start)

MEMORY
{
  FLASH (rx) : ORIGIN = 0x20000000, LENGTH = 60K 
  RAM (rw) : ORIGIN = 0x80000000, LENGTH = 0x4000
}

SECTIONS
{

  .text : { *(.text) } > FLASH
  .data : { *(.data)  } > FLASH
  .rodata : { *(.rodata)  } > FLASH
}

PROVIDE(_hart_stack_size = 2K);
PROVIDE(_stack_start = ORIGIN(RAM) + LENGTH(RAM));