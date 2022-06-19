OUTPUT_ARCH(riscv)

ENTRY(_start)

MEMORY
{
  ROM (rx) : ORIGIN = 0x20000000, LENGTH = 60K 
  RAM (rw) : ORIGIN = 0x80000000, LENGTH = 16K
}

SECTIONS
{
  .init : { *(.init); } > ROM
  .rodata : { *(.rodata); } > ROM
  .text : { *(.text.*); } > ROM
  .trap : ALIGN(4) { *(.trap); } > ROM


  .data (NOLOAD): { *(.data.*); } > RAM
  .sdata (NOLOAD) : { *(.sdata); } > RAM
  .bss (NOLOAD) : { *(.bss); } > RAM

}

PROVIDE(_hart_stack_size = 2K);
PROVIDE(_stack_start = ORIGIN(RAM) + LENGTH(RAM));