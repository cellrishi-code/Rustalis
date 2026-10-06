# Rustalis

**Rustalis** is an open-source, Rust-first operating-system project for building a real x86_64 OS from the boot path upward.

> Experimental systems software. Rustalis is not production-ready and must not be treated as a security boundary.

[![CI](https://github.com/cellrishi-code/Rustalis/actions/workflows/rust.yml/badge.svg)](https://github.com/cellrishi-code/Rustalis/actions/workflows/rust.yml)

## Vision

Rustalis aims to provide a complete operating-system stack:

```text
Firmware / Bootloader
        ↓
Rust Kernel
        ↓
Memory + Interrupts
        ↓
Processes + Scheduler
        ↓
Syscalls
        ↓
User Space + Shell
        ↓
VFS + Storage + Drivers
        ↓
Networking + Security + SMP
        ↓
Developer Tools + Package Manager
```

The project is deliberately incremental. A subsystem is only marked complete after a real implementation and an appropriate verification path exist.

## Current status

### Foundation
- [x] Cargo workspace
- [x] `no_std` kernel
- [x] x86_64 target
- [x] Bootloader integration
- [x] Serial logging and panic reporting
- [x] Boot memory-map summary
- [x] QEMU launcher
- [x] Open-source license and contribution documentation

### CPU and exceptions
- [x] x86_64 architecture boundary
- [x] Interrupt subsystem boundary
- [ ] GDT / TSS
- [x] IDT (breakpoint and invalid-opcode entries)
- [ ] Exception handlers
- [ ] Double-fault stack

### Memory
- [ ] Physical frame allocator
- [ ] Page-table abstraction
- [ ] Virtual memory manager
- [ ] Page-fault handling
- [ ] Kernel heap

### Kernel services
- [ ] APIC and timers
- [ ] Threads and context switching
- [ ] Preemptive scheduler
- [ ] Syscall ABI
- [ ] Processes and isolation

### User space
- [ ] Init process
- [ ] Core utilities
- [ ] `ferrosh` shell
- [ ] File descriptors
- [ ] Pipes and redirection

### Storage
- [ ] VFS
- [ ] Initial filesystem
- [ ] Block-device layer
- [ ] Device-driver framework

### Networking
- [ ] Ethernet
- [ ] ARP
- [ ] IPv4
- [ ] ICMP
- [ ] UDP
- [ ] TCP
- [ ] Sockets
- [ ] DHCP / DNS / IPv6

### Security and scalability
- [ ] UID/GID permissions
- [ ] Capability model
- [ ] Kernel/user isolation
- [ ] Syscall validation
- [ ] W^X
- [ ] Stack protection
- [ ] ASLR where practical
- [ ] SMP / per-CPU state

### Developer experience
- [ ] Package manager (`fpm`)
- [ ] Developer tools
- [ ] Automated QEMU integration tests
- [ ] Reproducible release images
- [ ] Optional graphical compositor

## Getting started

### Prerequisites

- Rust nightly
- Cargo
- QEMU x86_64
- Git

The repository contains `rust-toolchain.toml`, so Rustup can install the required toolchain automatically.

### Check the workspace

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

### Build

```bash
cargo build

Optional breakpoint exception smoke test:

```bash
cargo build --features exception-self-test
```
```

### Run in QEMU

UEFI:

```bash
cargo run -- uefi
```

BIOS:

```bash
cargo run -- bios
```

On Windows PowerShell:

```powershell
cargo run -- uefi
```

## Contributing

Rustalis is intended to be a genuine open-source collaboration project. Start with the issue tracker before implementing a large subsystem.

1. Read [CONTRIBUTING.md](CONTRIBUTING.md).
2. Read [docs/architecture.md](docs/architecture.md).
3. Pick an open issue or propose a new one.
4. Create a branch.
5. Implement the smallest coherent subsystem.
6. Add tests and safety documentation.
7. Run formatting, check, clippy, and tests.
8. Add QEMU verification when relevant.
9. Open a pull request using the repository template.

Good first contributions include documentation, tests, architecture diagrams, CI improvements, small kernel abstractions, and carefully scoped subsystem work.

## Project structure

```text
.
├── kernel/
│   └── src/
│       ├── arch/
│       │   └── x86_64/
│       ├── memory/
│       ├── logger.rs
│       └── main.rs
├── docs/
├── scripts/
├── .github/
│   ├── ISSUE_TEMPLATE/
│   └── workflows/
└── src/
    └── main.rs
```

## Engineering principles

- Rust first; assembly only where the architecture requires it.
- `no_std` kernel.
- Minimize and document `unsafe`.
- Never trust user-space memory.
- Prefer explicit `Result` / `Option` over unchecked failure.
- No fake implementations or fake verification.
- Preserve working functionality when adding subsystems.
- Document architectural decisions.
- Verify through unit tests, integration tests, and QEMU where appropriate.

## Roadmap and issues

The GitHub issue tracker is the authoritative implementation backlog. Major kernel, memory, scheduler, syscall, storage, networking, security, SMP, user-space, testing, and release milestones are tracked there.

## License

Rustalis is dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).

## Community

Please use the issue templates for bugs and subsystem proposals. Follow the [Code of Conduct](CODE_OF_CONDUCT.md) and use [SECURITY.md](SECURITY.md) for security reports.

---

**Rustalis is built in the open. If you want to learn operating-system engineering by contributing real code, pick an issue and build with us.**
