# Rustalis Architecture

## Milestone 0 boundary

Rustalis currently has three layers:

1. **Host runner** — a normal Rust program that builds/launches QEMU and supplies UEFI firmware.
2. **Bootloader** — the Rust `bootloader` image builder creates a bootable disk image from the kernel ELF.
3. **Kernel** — a `no_std` x86_64 binary receiving `BootInfo` from `bootloader_api`.

The host runner must never become part of the kernel image.

## Kernel boundaries

Hardware-dependent code belongs under `kernel/src/arch` or dedicated driver modules. Higher-level kernel subsystems should depend on abstractions rather than concrete hardware addresses.

Planned boundaries include:

```text
                  +----------------------+
                  |   arch / platform    |
                  +----------+-----------+
                             |
                    interrupts / timers
                             |
                  +----------v-----------+
                  |      scheduler       |
                  +----------+-----------+
                             |
                   process / task state
                             |
                       syscall ABI
                             |
                        user space

  boot info ---> memory manager ---> address spaces
                       |
                 heap allocator

  block drivers ---> block layer ---> filesystem / VFS
                                         |
                                    file descriptors

  NIC drivers ---> network device ---> network protocols
```

These are dependency boundaries and future architecture targets, not claims that every subsystem is implemented.

## Current initialization order

The current kernel entry point performs only the following sequence:

1. Initialize the serial logger.
2. Emit early startup and architecture information.
3. Initialize the x86_64 architecture boundary. At present this keeps maskable interrupts disabled because no IDT/handlers are installed yet.
4. Summarize the bootloader-provided memory map.
5. Exercise the fixed-capacity, allocation-free RAM filesystem with create/write/read.
6. Emit `[BOOT_OK]` only if the RAM filesystem smoke test succeeds.
7. Enter the kernel idle spin loop.

The host runner and bootloader image construction happen before kernel entry. The UEFI/BIOS choice is made by the host runner; it is not a kernel runtime subsystem.

## Dependency and ordering rules for future milestones

| Subsystem | Depends on | Must be ready before |
| --- | --- | --- |
| Architecture tables and exception handlers | Boot information, CPU feature discovery, dedicated stacks | Interrupt-driven device handling |
| Physical frame allocator | Validated boot memory map and reserved-region policy | Page tables and heap backing |
| Page tables / address spaces | Frame allocator, architecture paging definitions | Kernel heap mappings and user processes |
| Kernel heap | Frame allocator and kernel virtual mappings | Heap-backed collections and general allocation |
| APIC timer | CPU/APIC discovery, interrupt routing and calibrated clocksource | Timer-driven preemption |
| Scheduler | Task context, interrupt/timer path, synchronization primitives | Preemptive multitasking |
| Processes | Address spaces, task lifecycle, scheduler | Syscall process operations |
| Syscall layer | IDT/entry mechanism, process context, user-memory validation | User-space ABI |
| Block layer | Device discovery and block-driver registry | Persistent filesystem |
| VFS | Inode/file/directory contracts and mount model | General file-descriptor syscalls |
| Network stack | Network-device interface and packet-buffer ownership | Socket API |

A dependency is not permission to initialize a placeholder. Each subsystem must expose an explicit readiness/failure result before dependents use it.

## Boot invariant

Every milestone must leave a bootable kernel or a clearly documented reason why the milestone temporarily cannot boot. No placeholder subsystem should report successful initialization.
