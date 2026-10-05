# Rustalis Milestones

## M0 — Foundation

- [x] Rust workspace
- [x] `no_std` kernel
- [x] x86_64 target
- [x] Bootloader integration
- [x] Serial logging
- [x] Panic handler
- [x] QEMU runner
- [ ] Verified UEFI boot in CI

## M1 — CPU and Exceptions

- [x] x86_64 architecture module
- [x] Interrupt subsystem boundary
- [ ] GDT
- [ ] IDT
- [ ] Exception handlers
- [ ] Double-fault stack
- [ ] Useful register dumps

## M2 — Memory

- [ ] Boot memory-map abstraction
- [ ] Physical frame allocator
- [ ] Page-table abstraction
- [ ] Mapping/unmapping
- [ ] Page-fault handler
- [ ] Kernel heap

Later milestones remain intentionally unchecked until their implementations are real.
