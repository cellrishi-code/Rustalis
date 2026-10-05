# Boot Process

Rustalis targets x86_64 first and uses the Rust `bootloader` ecosystem to construct bootable images.

```text
UEFI firmware
     |
     v
Bootloader image
     |
     v
Rustalis kernel ELF
     |
     v
kernel_main(BootInfo)
     |
     +--> serial logger
     +--> architecture initialization
```

The host-side launcher is deliberately separate from the kernel. It starts QEMU and selects the UEFI or BIOS image generated during the Cargo build.

## Verification

A local verification should show the kernel's INFO lines over COM1. The repository does not claim a verified boot until QEMU has actually been run against the generated image.
