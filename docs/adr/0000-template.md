# Architecture Decision Record (ADR) Template

Use an ADR when a change affects a subsystem boundary, syscall or userspace ABI, memory layout, interrupt routing, device-driver contract, filesystem semantics, or security boundary.

Copy this file to `docs/adr/NNNN-short-title.md`. Number records sequentially and do not reuse a number.

---

# ADR-NNNN: Short decision title

- **Status:** Proposed | Accepted | Superseded | Deprecated
- **Date:** YYYY-MM-DD
- **Owners:** @github-handle(s)
- **Related issues / PRs:** #...

## Context

Describe the problem, constraints, and the current behavior. Distinguish implemented behavior from planned architecture. Include relevant hardware, boot, compatibility, and performance constraints.

## Decision drivers

- What requirements matter most?
- Which safety, correctness, or compatibility invariants must hold?
- What must remain possible for future milestones?

## Considered options

### Option A — Name

Explain the approach, benefits, and costs.

### Option B — Name

Explain the approach, benefits, and costs.

## Decision

State the chosen option precisely. Include interfaces, ownership, initialization order, and failure behavior where relevant.

## Consequences

### Positive

- ...

### Trade-offs and risks

- ...

## Safety and security

Document trust boundaries, validation rules, privilege transitions, and any `unsafe` invariants. If no unsafe code or security boundary is involved, say so.

## Verification plan

List host tests, property/fuzz tests, CI checks, and QEMU/hardware verification. Do not mark a check complete until it has actually run.

## Follow-up work

- [ ] ...
