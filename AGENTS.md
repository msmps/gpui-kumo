# GPUI design-system work

Before designing or implementing components, tokens, themes, or interaction behavior, read [docs/README.md](docs/README.md) and follow its task-specific pointers. Use [CONTEXT.md](CONTEXT.md) for canonical terminology. Recheck version-sensitive APIs against the project's selected dependency before using the source snapshot documented there.

Port supported, recommended Kumo components, APIs and variants. Check the selected source for deprecation before adding a public counterpart; omit deprecated features unless the user explicitly requests compatibility.

## Engineering preferences

Use idiomatic Rust and follow Rust and GPUI best practices: retain durable UI state in entities, use typed semantic APIs, keep Base implementation details within component boundaries, and verify behavior through real rendering and input paths. Run formatting, relevant tests, and Clippy with warnings denied before completing changes.

Complete checks that can run in the current environment and resolve failures before treating implementation work as finished. Record measured results separately from concrete dependency or platform limits; do not leave runnable validation as a generic follow-up.

During every visual review, explicitly inspect alignments: icon/control centres, text baselines, label/control offsets, sibling spacing and edge padding in both themes and narrow layouts. Follow the alignment branch in [verification](docs/gpui-verification.md); default-state screenshots alone do not establish composition alignment.

Inspect rounded corners and layered surfaces during every visual review: child backgrounds must respect the supported clipping contract; borders, fills, focus rings and shadows must agree across nested layers, both themes, resized and narrow layouts. Record unsupported arbitrary-descendant clipping explicitly instead of masking it with a default-case screenshot.

For floating action tabs and overlapping controls, verify pointer activation leaves focus on the intended action before testing Space/Enter; also test owner updates that remove a focused action and subsequent Tab exit.
