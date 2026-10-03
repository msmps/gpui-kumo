# Kumo pinned patch

Exact crates.io0.19.1 archive: SHA256023da0e5097f46df7092d5280b02efb9bbf8d93298daeced42652463e357d636; source revision c88605b96d04431f9c3c792464a0f2f253480e94/platforms/atspi-common.

Production diff: src/node.rs guards Enabled/Sensitive with !state.is_disabled(). Existing supported-role ReadOnly mapping is unchanged. Authored is_busy maps independently to State::Busy. Actual optional expansion maps None to neither state, Some(false) to Expandable, Some(true) to Expandable+Expanded. Five observable state/event regressions added to src/adapter.rs. All other published source/configuration files unchanged. License files are added from the exact source revision because the published archive omits them.

See ../../docs/linux-disabled-state-patch.md for acceptance, evidence, downstream root patch setup and removal policy. No upstream release or publication is implied.

Busy set/clear and enabled/disabled policies: ../../docs/linux-busy-state-validation.md.

Expansion tri-state/events/native disclosure checks: ../../docs/linux-expansion-state-validation.md.
