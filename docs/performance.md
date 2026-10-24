# Performance validation

Rustalis is an experimental OS, so performance changes should be measured rather
than assumed. The release profile in the workspace enables maximum optimization,
thin link-time optimization, and a single codegen unit. These settings affect
release builds; ordinary development builds remain optimized for iteration speed.

## Build

Use the pinned nightly toolchain from `rust-toolchain.toml`:

```sh
cargo build --release
```

For correctness checks, continue to run:

```sh
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Benchmark protocol

Before claiming a speedup, record a baseline and compare it with the optimized
build on the same host and QEMU configuration.

Record at least:

- Build mode and Rust toolchain version.
- Kernel/boot-image size.
- Time from QEMU launch to the first serial boot message.
- Repeated measurements (at least five runs), reporting median and spread.
- QEMU version, machine type, firmware, CPU settings, and host OS.

Keep the guest workload and emulator arguments identical between runs. Separate
build-time improvements from boot-time and runtime improvements; they measure
different things. Do not treat a smaller binary or a faster host build as proof
of faster kernel execution.

## Next optimization targets

Once the corresponding subsystems exist, benchmark serial logging overhead,
page allocation, context switching, syscall entry/exit, and filesystem/network
I/O independently. Avoid removing diagnostics or weakening safety checks solely
to improve a benchmark. Keep debug builds useful for kernel development.
