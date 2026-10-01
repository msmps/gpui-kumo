# GPUI design-system work

Before designing or implementing components, tokens, themes, or interaction behavior, read [docs/README.md](docs/README.md) and follow its task-specific pointers. Use [CONTEXT.md](CONTEXT.md) for canonical terminology. Recheck version-sensitive APIs against the project's selected dependency before using the source snapshot documented there.

## Engineering preferences

Use idiomatic Rust and follow Rust and GPUI best practices: retain durable UI state in entities, use typed semantic APIs, keep Base implementation details within component boundaries, and verify behavior through real rendering and input paths. Run formatting, relevant tests, and Clippy with warnings denied before completing changes.
