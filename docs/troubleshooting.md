# Troubleshooting Rustalis

This guide separates host setup failures from kernel boot failures. Rustalis is experimental; a successful image build does not prove that the kernel boots correctly.

## Prerequisites

- Use the toolchain specified by `rust-toolchain.toml` (Rust nightly).
- Install Cargo and Git.
- Install QEMU with the `qemu-system-x86_64` executable available on `PATH`.
- For UEFI runs, the runner pins OVMF to EDK2 stable 202508-r1 through `Source::EDK2_STABLE202508_R1`. The `ovmf-prebuilt` crate validates the downloaded archive against its embedded SHA-256 and caches firmware under `target/ovmf`.

Check the toolchain and QEMU executable:

```sh
rustc --version
cargo --version
qemu-system-x86_64 --version
```

## Build or test failures

Run the checks individually to identify the failing stage:

```sh
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build
```

- **Toolchain/target errors:** confirm Rustup selected the repository's toolchain and that the target named in the error is installed.
- **Dependency errors:** check network access and Cargo's registry cache, then retry the failing command.
- **Formatting errors:** run `cargo fmt --all` and inspect the resulting diff before committing.

## Host runner failures

Run either supported mode:

```sh
cargo run -- uefi
cargo run -- bios
```

- **QEMU cannot start:** install the QEMU x86_64 system emulator and ensure `qemu-system-x86_64` is on `PATH`.
- **UEFI firmware acquisition fails:** check network access and write permissions for the repository's `target/ovmf` cache. BIOS mode does not require OVMF.
- **Invalid command:** use exactly one mode argument, `uefi` or `bios`; use `--help` for usage.

## Kernel boot failures

The QEMU runner configures serial output and disables the graphical display. Read the serial log first:

- If no kernel log appears, investigate image creation, boot firmware, and QEMU configuration.
- If initialization logs appear but the final success message is absent, investigate the last subsystem log before the failure.
- A successful `cargo build` only verifies image construction. Treat runtime boot as verified only after the kernel actually runs and its expected serial output is observed.

When reporting a failure, include the command, host OS, Rust toolchain version, QEMU version, and complete relevant serial/error output. Do not include secrets or private environment values.
