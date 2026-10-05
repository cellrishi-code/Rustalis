# Security Policy

Rustalis is experimental systems software. It should not be used as a security boundary or production operating system yet.

## Reporting vulnerabilities

For vulnerabilities in the kernel, boot path, memory management, syscall layer, or user-space isolation, open a private security report through GitHub when available. Do not publish an exploitable proof-of-concept before maintainers have had a chance to investigate.

## Security principles

- Keep kernel and user address spaces isolated.
- Validate all user-provided addresses and lengths.
- Minimize `unsafe` blocks.
- Document safety invariants.
- Never treat user memory as trusted.
- Do not enable writable+executable memory by default.
- Keep privileged operations behind explicit interfaces.
