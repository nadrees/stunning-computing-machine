# stunning-computing-machine
Learning how to build OSs from scratch

This is a repository of me messing around and learning how operating systems work deep down. Based off following https://operating-system-in-1000-lines.vercel.app/en/, but using Rust instead of C.

# Setup

We follow the Ubuntu setup from the guide. They've been copied here for convenience, but should they diverge follow the instructions in the guide over the ones here. 

## From the guide

https://operating-system-in-1000-lines.vercel.app/en/01-setting-up-development-environment

Install the needed packages

> sudo apt update && sudo apt install -y clang llvm lld qemu-system-riscv32 curl

Download OpenBSI (UEFI/BIOS)

NOTE: Here we're using the version from the latest stable branch. The master branch is for active development, so you'll need to update the url to whatever the current latest is.

> curl -LO https://github.com/qemu/qemu/blob/stable-9.2/pc-bios/opensbi-riscv32-generic-fw_dynamic.bin

Check that clang supports riskv32:

> clang -print-targets | grep riscv32
