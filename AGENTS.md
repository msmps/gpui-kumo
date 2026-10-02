# GPUI design-system work

Before designing or implementing components, tokens, themes, or interaction behavior, read [docs/README.md](docs/README.md) and follow its task-specific pointers. Use [CONTEXT.md](CONTEXT.md) for canonical terminology. Recheck version-sensitive APIs against the project's selected dependency before using the source snapshot documented there.

Port supported, recommended Kumo components, APIs and variants. Check the selected source for deprecation before adding a public counterpart; omit deprecated features unless the user explicitly requests compatibility.

## Engineering preferences

Use idiomatic Rust and follow Rust and GPUI best practices: retain durable UI state in entities, use typed semantic APIs, keep Base implementation details within component boundaries, and verify behavior through real rendering and input paths. Run formatting, relevant tests, and Clippy with warnings denied before completing changes.

Complete checks that can run in the current environment and resolve failures before treating implementation work as finished. Record measured results separately from concrete dependency or platform limits; do not leave runnable validation as a generic follow-up.
