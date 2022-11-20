
build:
	cargo build --target="riscv32imc-unknown-none-elf" --bin kernel

kernel: build
	riscv64-unknown-elf-objcopy -O binary target/riscv32imc-unknown-none-elf/release/kernel target/riscv32imc-unknown-none-elf/release/kernel.bin

# Installs the bitstream in the fpga
install: kernel
	openocd -f openocd.cfg -c "program target/riscv32imc-unknown-none-elf/release/kernel.bin verify 0x20000000 reset exit"

clean:
	rm -fr target

