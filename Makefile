
build:
	cargo build --release

bin: build
	riscv64-unknown-elf-objcopy -O binary target/riscv32imc-unknown-none-elf/release/os target/riscv32imc-unknown-none-elf/release/os.bin

# Installs the bitstream in the fpga
program: bin
	openocd -f openocd.cfg -c "program target/riscv32imc-unknown-none-elf/release/os.bin verify 0x20000000 reset exit"

clean:
	rm -fr target

