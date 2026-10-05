# Rustalis

**Rustalis** is a Rust-first operating system project targeting a real, bootable x86_64 system, with QEMU as the primary development environment.

The project is being built incrementally. Each milestone must compile and remain understandable before the next kernel subsystem is introduced.

## Current milestone: 0 — Foundation

Implemented:

- Rust workspace with a dedicated kernel crate
- `no_std` / `no_main` kernel
- x86_64 bare-metal target
- bootloader API entry point
- boot information reception
- serial logging
- kernel panic handler
- x86_64 architecture boundary
- UEFI/BIOS disk-image generation through the Rust `bootloader` crate
- QEMU runner with OVMF support

The next verification step is to build and boot the generated UEFI image locally and capture the serial output:

```text
[INFO] Rustalis kernel starting
[INFO] Architecture: x86_64
[INFO] Boot information received
[INFO] Memory map is available
[INFO] Rustalis booted successfully.
```

## Run

Install QEMU with `qemu-system-x86_64`, then:

```bash
cargo build
cargo run -- uefi
```

For legacy BIOS testing:

```bash
cargo run -- bios
```

The UEFI path uses OVMF through `ovmf-prebuilt`.

## Architecture

```text
UEFI
  |
Bootloader
  |
Rustalis Kernel
  +-- arch/x86_64
  +-- memory
  +-- interrupt
  +-- scheduler
  +-- process
  +-- syscall
  +-- ipc
  +-- filesystem
  +-- networking
  +-- drivers
  +-- security
  |
User Space
  +-- init
  +-- ferrosh
  +-- core utilities
```

Only subsystems that are actually implemented should be advertised as supported.

## Current boot diagnostics

At boot the kernel now inspects the bootloader memory map and reports:

- number of memory regions
- usable physical memory in bytes
- reserved/non-usable memory in bytes

This is the first kernel-owned memory abstraction. It does not yet allocate frames or manipulate page tables.

## Roadmap

1. Foundation and verified boot
2. GDT, IDT and exception handling
3. Physical frame allocator
4. x86_64 virtual memory
5. Kernel heap
6. Interrupts and timers
7. Threads and context switching
8. Preemptive scheduler
9. Syscalls and user space
10. VFS and storage
11. Device drivers
12. Networking
13. Security hardening
14. SMP
15. Optional graphics
16. Package manager
17. Developer tooling

## Development principles

- Rust first; assembly only where architecture requires it.
- Every `unsafe` block must have a safety argument.
- No fake implementations or fake success messages.
- Kernel and user space remain separated.
- Hardware-dependent code stays behind clear interfaces.
- Keep the OS bootable at every milestone.
- Prefer explicit errors and documented invariants.
- Run `cargo fmt`, `cargo clippy`, and tests continuously.

## License

Dual-licensed under MIT OR Apache-2.0.
