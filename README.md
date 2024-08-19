# WeesyOS in Rust 🦀

WeensyOS is a tiny kernel that runs on bare-metal x86-64 machines, including QEMU's emulated CPUs, with minimal unsafe code—currenty 15 instances used for handling ports, registers, and interrupts. The initial state of the kernel contains code for bootstrapping kernel, handling exceptions/syscalls, executing user-level program, and helper functions for the System Programing and Computer Organization [CPSC323] exercises. The implementation includes features for displaying and animating physical and virtual memory.

> [!TIP]
> This project comprises two branches: one for `rust-safe` (semisafe) code, serving as an example of a potentially correct OS implementation in Rust that aims to minimize the use of unsafe code, and another for `unsafe` code closely following C conventions, essential for its similarity to the WeensyOS pset.

The project is heavily influenced by the following projects:

    RedoxOS: RedoxOS is an operating system written in Rust, aiming to bring the innovations of Rust to a modern microkernel and full set of applications. It's fair to say it is the most advanced of all Rust OS-es.
    k4dos: k4dos is another hobby-os of that sort, it's fairly cool, with userspace, that can run FreeDoom for example. It has a nice shell implementation: kash
    blog_os: blog_os is a cutting-edge project by Philipp Oppermann that provides a detailed tutorial on building an operating system in Rust. It covers various aspects, including the bootloader, memory management, and device drivers.

## Preview

![WeensyOS](https://github.com/user-attachments/assets/c39b8411-b07c-47ab-bd1d-8ad7098a46c4)
