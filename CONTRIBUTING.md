# Contributing to Rustalis

Rustalis is developed milestone-by-milestone. Contributions should keep the tree buildable and should not introduce fake implementations.

## Before submitting changes

```bash
cargo fmt --all -- --check
cargo check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

For kernel changes, also verify the appropriate QEMU boot path locally when QEMU is available.

## Unsafe Rust

Every new `unsafe` block must document the invariant that makes the operation safe. Keep hardware-specific unsafe code inside the smallest possible module.

## Subsystems

Do not mix unrelated major kernel subsystems in one change. Prefer small changes that establish one invariant at a time.
