#!/bin/bash
set -e

echo "Assembling boot.asm"
nasm 0f elf64 boot.asm -o boot.o

echo "building rust kernel"
cargo build --release

echo "Linking kernel"
ld -n -o marquays.bin -T linker.ld boot.o target/x86_64-unknown-none/release/libmarquays_kernel.a

echo "launching qemu"
qemu-system-x86_64 \
	-drive format=raw,file=marquays.bin \
	-serial stdio \
	-display none \
	-m 256M
