# Switch acceptance

Pinned Kumo Switch source, full demo and documentation inspected at `3fd5b648df578cb1ba214dedd30f475009f6a668`. Installed Base 0.7.0 Switch/Track/Thumb own binary activation, focus, checked metadata and composable parts. Group is presentation-only in Kumo, so each Rust Switch child preserves its own controlled callback.

| Area | Acceptance |
| --- | --- |
| API | sm/base/lg, default/neutral, checked/disabled/transitioning, optional, bare and rich decorative label, control-first/label-first; Group child Switch is native Item, custom/hidden legend, both error and helper |
| Paint | Actual render recipe: 32×16, 36×18, 40×20; thumb is square track height, travels one height; source 5px fallback radius and edge/drop shadows; palette centralized in Theme.switch |
| States | Off/on, pointer and keyboard focus, disabled single control versus whole group item opacity; busy metadata; invalid visual variant is N/A (source's prose says error but actual variant catalog has only default/neutral) |
| Motion | 150ms CSS ease-out travel and colour transition; rapid reversals start from presented progress; reduced motion snaps to target; presentation progress never becomes checked state |
| Input | Label/track pointer and Space/Return propose once; rejected proposals leave value unchanged; disabled cannot activate from retained focus; Tab follows Base focus behavior |
| Groups | 16px outer gaps, 8px item gaps, per-item controlled callbacks, group/item disabled, stable distinct IDs, simultaneous helper/error |
| Alignment/layers | Control vertically centered against wrapped labels, source track/thumb endpoints, border/ring and shadow radii; wide/narrow both themes; no child background leak or unnecessary clipping mask |
| Semantics | Actual Base Switch role/name/toggled state; synthetic parent disabled/busy flags; native spoken verification remains pending. Fieldset relationship/tooltip remain dependency gaps |
| N/A | Open/dismiss/trap, selection/clipboard/Unicode editing/IME, loading indicator, aggregate group value |

Native policies: squircle is unsupported, so use the explicit source 5px rounded fallback. Pinned classes refer to neutral-150/850 although the binding defines kumo-prefixed intermediate neutral tokens; native theme maps the intended .935/.24 values explicitly, pending browser comparison of that source discrepancy. Base root encompasses label and control to route activation once, rather than emulating DOM label forwarding. CSS colour transition uses sRGB blending; exact browser colour-space comparison remains pending. Initial/uncontrolled values remain consumer-owned. Transitioning authors busy metadata and does not suppress interaction, matching source.

## Evidence and review

86 workspace tests/eight doctests pass. Formatting, locked all-target/all-feature builds and Clippy with warnings denied pass. Five Switch regressions verify pointer/Space/Return exactly-once proposals, rejected controlled updates, disabled retained-focus paths, source geometry/wrapping, simultaneous group messages, repeated group/item identity and independent callbacks. Actual painted thumb travel and colour are measured between endpoints, rapid reversal preserves presented position, completion reaches the endpoint, and reduced motion snaps. Paint asserts source 5px corner radii; wall time drives GPUI's unsynced animation clock.

Light (capture removed), dark (capture removed), narrow light (capture removed) and narrow dark (capture removed) were inspected for track/thumb corners, fill/ring/shadow agreement and text/control alignment. Keyboard capture (capture removed) shows visible brand focus and exactly three changes following separately settled pointer/Space/Return. Native captures establish endpoint appearance; timed motion is supported by paint tests, not inferred from screenshots. All live native test applications terminated.

Independent review repaired duplicate Label role/name by using content-only labels under the Base semantic root. No additional implementation blocker found; browser squircle/colour interpolation and OS spoken semantics remain unverified.
