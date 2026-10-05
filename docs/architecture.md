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
arch -> interrupt -> scheduler -> process -> syscall -> user space
                  |
                memory
                  |
             filesystem/storage
                  |
              networking
```

These are architectural targets, not claims that those subsystems are implemented yet.

## Boot invariant

Every milestone must leave a bootable kernel or a clearly documented reason why the milestone temporarily cannot boot. No placeholder subsystem should report successful initialization.
