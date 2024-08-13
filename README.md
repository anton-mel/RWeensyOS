# 🦀 WeesyOS in Rust 🦀

WeensyOS is a tiny kernel that runs on bare-metal x86-64 machines, including QEMU's emulated CPUs, with minimal unsafe code—currenty 15 instances used for handling ports, registers, and interrupts. The initial state of the kernel contains code for bootstrapping kernel, handling exceptions/syscalls, executing user-level program, and helper functions for the System Programing and Computer Organization [CPSC323] exercises. The implementation includes features for displaying and animating physical and virtual memory layouts based on 4KiB categorized pages for educational purposes.

> [!TIP]
> This project comprises two branches: one for `rust-safe` (semisafe) code, serving as an example of a potentially correct OS implementation in Rust that aims to minimize the use of unsafe code, and another for `unsafe` code closely following C conventions, essential for its similarity to the WeensyOS pset.

## Preview

![WeensyOS](https://github.com/user-attachments/assets/c39b8411-b07c-47ab-bd1d-8ad7098a46c4)
