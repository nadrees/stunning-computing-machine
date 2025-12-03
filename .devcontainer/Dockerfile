FROM rust:1.91.1

# Install the needed packages
RUN apt update && apt install -y clang-19 llvm-19 lld-19 qemu-system-riscv64 curl lldb-19

# Download OpenBSI (UEFI/BIOS)
# NOTE: Here we're using the version from the latest stable branch. The master branch is for active development, so we'll need to update the url to whatever the current latest is as needed.
RUN curl -LO https://github.com/qemu/qemu/blob/stable-9.2/pc-bios/opensbi-riscv64-generic-fw_dynamic.bin

# Install the needed rust target
RUN rustup target add riscv64gc-unknown-none-elf

# Install recommended cargo utilties
RUN cargo install cargo-binutils && rustup component add llvm-tools