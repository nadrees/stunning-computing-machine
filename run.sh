#!/bin/bash

# Set or unset values of shell options and positional parameters.
# -e: Exit immediately if a command exits with a non-zero status.
# -u: Treat unset variables as an error when substituting.
# -x: Print commands and their arguments as they are executed.
set -xue 

# QEMU file path
QEMU=qemu-system-riscv32

# build the binary
cargo build

# Start QEMU
$QEMU \
    -cpu rv32 \
    -machine virt \
    -bios default \
    -nographic \
    -serial mon:stdio \
    --no-reboot \
    -device loader,file="target/riscv32imafc-unknown-none-elf/debug/kernel",cpu-num=0