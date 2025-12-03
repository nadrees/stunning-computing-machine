# stunning-computing-machine
Learning how to build OSs from scratch

This is a repository of me messing around and learning how operating systems work deep down. Based off following https://operating-system-in-1000-lines.vercel.app/en/, https://osblog.stephenmarz.com/ch0.html, and others but using Rust instead of C.

# Setup

We have configured a devcontainer which should configure everything that's needed to get started. We have also configured vscode to recommend the dev containers extension. When prompted, install the extension on your host machine, reload, and then when prompted again select the option to rebuild the container and reopen.

Inside the container, we have pre-installed the cargo-binutils tool to make inspecting the compiled objects easier. See https://github.com/rust-embedded/cargo-binutils#readme for details

## Recommended VSCode Extensions

We have configured VSCode to automatically recommend the needed extensions for this project. You should install all when prompted.

# Rust Embedded Assembly

When we reach https://operating-system-in-1000-lines.vercel.app/en/02-assembly#inline-assembly in the guide, we'll instead be using Rust's inline assembly functionality: https://doc.rust-lang.org/reference/inline-assembly.html

## Linker Scripts

Part of writing operation systems (or embedded code in general) is linker scripts. We can follow the pages at https://docs.rust-embedded.org/embedonomicon/memory-layout.html for instructions and examples on how to use them.

For a primer on what a linker script even is, check out https://mcyoung.xyz/2021/06/01/linker-script/

# Running

`cargo run` has been configured to issue the correct command to start the OS in QEMU. Once QEUM is running, you 
can drop into the console by using `Ctrl + A, c`.

You can quit from the console by using the command `quit`. 

You can force quit QEMU at any time using `Ctrl + A, x`.

# Debugging

A launch configuration for debugging has been configured for VS Code. Use the configuration to launch QEMU in debug
mode and attach the lldb debugger to it. 

## Debugging Details

Debugging the kernel can be difficult, especially if it has issues booting initially. Debugging can be done using 3 tools:

1. QEMU in debugging mode
2. The lldb debugger
3. objdump

### QEMU in debugging mode

https://en.wikibooks.org/wiki/QEMU/Debugging_with_QEMU

In order to debug the kernel, add in the `-s` and `-S` flags to `run.sh` when launching QEMU. This will cause QEMU to stop and wait for a debugger (`-S`) and listend for that debugger on tcp:1234 (`-s`). 

NOTE: This setup has been completed already as part of the debug task.

### The lldb debugger

Once QEMU is running and waiting, we need to connect to it with a debugger. We can use `lldb` to achieve this. Run `lldb` and configure the file being debugged, before connecting to the remote debug server exposed by QEMU in the previous step:

```
> lldb
lldb> target create --no-dependents --arch riscv64 target/riscv64gc-unknown-none-elf/debug/kernel 
lldb> gdb-remote 1234
```

Once connected you should start seeing either assembly or the rust source, depending on where the current frame is.

NOTE: This setup has been completed already as part of the debug task.

### objdump

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