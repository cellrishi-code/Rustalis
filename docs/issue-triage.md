# Issue triage and labeling guide

This guide keeps the Rustalis issue tracker searchable and helps contributors choose work that matches its risk and scope. Labels are recommendations for maintainers; creating this guide does not automatically apply labels to existing issues.

## Label dimensions

Apply one label from each dimension when the information is known.

| Dimension | Suggested labels | Meaning |
| --- | --- | --- |
| Subsystem | `area:boot`, `area:arch`, `area:memory`, `area:interrupts`, `area:scheduler`, `area:process`, `area:syscall`, `area:filesystem`, `area:storage`, `area:drivers`, `area:network`, `area:security`, `area:userspace`, `area:ci`, `area:docs` | Primary code or documentation ownership |
| Difficulty | `difficulty:good-first-issue`, `difficulty:intermediate`, `difficulty:advanced` | Expected implementation complexity, not contributor ability |
| Verification | `test:host`, `test:qemu`, `test:hardware`, `test:docs` | Evidence needed before the issue can close |
| Contributor status | `help-wanted`, `blocked`, `in-progress`, `needs-design` | Whether outside contributions are welcome and what is blocking progress |

Use existing repository labels where possible; create a label only when it improves filtering. Avoid assigning multiple conflicting values within one dimension.

## Triage workflow

1. **Check for duplicates.** Search open and closed issues and link related discussions.
2. **Classify the scope.** Identify the subsystem, user-visible behavior, and whether the change is implemented behavior or roadmap work.
3. **Make it actionable.** State the problem, constraints, expected behavior, acceptance criteria, and relevant files or interfaces when known.
4. **Identify risks.** Call out hardware assumptions, boot-order dependencies, ABI changes, memory-safety concerns, and `unsafe` invariants.
5. **Choose verification.** Specify host tests for hardware-independent logic and QEMU or hardware checks for architecture paths.
6. **Set contributor suitability.** Use good-first-issue only when the task is bounded, has clear acceptance criteria, and does not require undocumented subsystem knowledge.
7. **Keep status current.** Mark blocked work with the missing dependency or design decision. Link the PR that implements the issue.

## Ready-to-work checklist

- [ ] Problem and expected outcome are clear.
- [ ] Scope is bounded and does not combine unrelated subsystems.
- [ ] Dependencies and initialization order are identified.
- [ ] Acceptance criteria are observable.
- [ ] Required tests and documentation are named.
- [ ] No duplicate or active PR already covers the work.

## Closing checklist

Close an issue only after the implementation is merged and its acceptance criteria are met. Do not treat a design note, placeholder, or unrun QEMU command as completed implementation or verification.
