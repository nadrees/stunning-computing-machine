# stunning-computing-machine
Learning how to build OSs from scratch

This is a repository of me messing around and learning how operating systems work deep down. Based off following https://operating-system-in-1000-lines.vercel.app/en/, but using Rust instead of C.

# Setup

We follow the Ubuntu setup from the guide. They've been copied here for convenience, but should they diverge follow the instructions in the guide over the ones here. 

## From the guide

https://operating-system-in-1000-lines.vercel.app/en/01-setting-up-development-environment

Install the needed packages

> sudo apt update && sudo apt install -y clang llvm lld qemu-system-riscv32 curl lldb

Download OpenBSI (UEFI/BIOS)

NOTE: Here we're using the version from the latest stable branch. The master branch is for active development, so you'll need to update the url to whatever the current latest is.

> curl -LO https://github.com/qemu/qemu/blob/stable-9.2/pc-bios/opensbi-riscv32-generic-fw_dynamic.bin

Check that clang supports riskv32:

> clang -print-targets | grep riscv32

## Install the needed rust target

> rustup target add riscv32imafc-unknown-none-elf

## Install recommended cargo utilties

> cargo install cargo-binutils

> rustup component add llvm-tools

See https://github.com/rust-embedded/cargo-binutils#readme for details

## Recommended VSCode Extensions

* rust-analyzer: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer
* LinkerScript: https://marketplace.visualstudio.com/items?itemName=ZixuanWang.linkerscript
* Even Better TOML: https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml
* Dependi: https://marketplace.visualstudio.com/items?itemName=fill-labs.dependi

# Rust Embedded Assembly

When we reach https://operating-system-in-1000-lines.vercel.app/en/02-assembly#inline-assembly in the guide, we'll instead be using Rust's inline assembly functionality: https://doc.rust-lang.org/reference/inline-assembly.html

## Linker Scripts

Part of writing operation systems (or embedded code in general) is linker scripts. We can follow the pages at https://docs.rust-embedded.org/embedonomicon/memory-layout.html for instructions and examples on how to use them.

For a primer on what a linker script even is, check out https://mcyoung.xyz/2021/06/01/linker-script/

# Debugging

Debugging the kernel can be difficult, especially if it has issues booting initially. Debugging can be done using 3 tools:

1. QEMU in debugging mode
2. The lldb debugger
3. objdump

## QEMU in debugging mode

https://en.wikibooks.org/wiki/QEMU/Debugging_with_QEMU

In order to debug the kernel, add in the `-s` and `-S` flags to `run.sh` when launching QEMU. This will cause QEMU to stop and wait for a debugger (`-S`) and listend for that debugger on tcp:1234 (`-s`). 

## The lldb debugger

Once QEMU is running and waiting, we need to connect to it with a debugger. We can use `lldb` to achieve this. Run `lldb` and configure the file being debugged, before connecting to the remote debug server exposed by QEMU in the previous step:

```
> lldb
lldb> target create --no-dependents --arch riscv32 target/riscv32imafc-unknown-none-elf/debug/kernel 
lldb> gdb-remote 1234
```

Once connected you should start seeing either assembly or the rust source, depending on where the current frame is.

## objdump

When viewing rust in particular, the symbols are not currently loaded well, so it can be difficult to understand what's being executed in any given frame. It can be helpful to match the current value of the program counter (`pc`) register to the assembly instead.

To see this, run 

```
> cargo objdump -- -d
```

To get a print out of the current program and its related assembly.

# Random Knowledge Dumps

## Why use -device loader,... instead of -kernel or -bios?

We're building a very-nearly base metal kernel, so we want as much control with as little magic as possible. See:

https://stackoverflow.com/questions/58420670/qemu-bios-vs-kernel-vs-device-loader-file