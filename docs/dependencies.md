# Dependency compatibility and known limitations

For normal setup and a minimal application, see the [installation guide](../README.md#installation). This document covers the dependency family, the behavior received with published dependencies, and optional corrections.

## Compatible dependency family

| Package | Selected baseline |
| --- | --- |
| `gpui-kumo` | `0.1.0-rc.2` (unreleased; published RC: `0.1.0-rc.1`) |
| `gpui-kit` | Exactly `0.7.0` |
| `gpui-pre` and the matching GPUI/platform family | Exactly `0.3.7`, selected by Kit |
| `gpui-base` | Validated with `0.7.0`; Kit's manifest accepts compatible `0.7` versions |
| `accesskit_atspi_common` on Linux | Validated with `0.19.1`, selected transitively |

Commit your application's `Cargo.lock` and inspect resolutions when updating dependencies. Kumo's repository lockfile does not pin a consuming application's dependency graph.

An existing application using the same crates.io `gpui-kit 0.7.0` shares its GPUI types with Kumo. Cargo unifies compatible dependencies and their enabled features; `default-features = false` here does not disable features another dependency enables. A different GPUI package, incompatible version or separate Git source can produce a second framework instance with incompatible `App`, `Window`, `Entity` and element types. Official Zed Git GPUI, the `gpui` package and GPUI CE are not interchangeable with this selected family. Renaming an import does not convert those types. See [Cargo dependency sources and names](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html).

If existing code imports `gpui::`, a direct alias to the same published package can share Kit's types:

```toml
gpui = { package = "gpui-pre", version = "=0.3.7" }
```

Here `gpui` is only the name your code imports; Cargo still resolves the `gpui-pre` package. Use `gpui_kit` for new integration code. This alias does not make a different GPUI implementation compatible.

Check the graph with:

```sh
cargo tree -i gpui-kit
cargo tree -i gpui-pre
cargo tree -i gpui-base
cargo tree -d
```

## What the unpatched RC provides

Normal installation uses published dependencies. It does not require vendor directories or Cargo patches. The following known dependency limitations remain:

| Area | Observable limitation without patches |
| --- | --- |
| Keyboard focus | Repeated registration of an editor's focus handle can leave reverse-Tab stuck in the editor rather than moving to the preceding action/help button. This affects compositions such as Field, InputGroup and Dialog |
| Textarea automatic growth | Changing row policy and existing content in the same owner update can leave InputArea at its minimum height instead of growing to the configured cap. The native reproduction holds five lines but displays one with a 1–3-row policy. Automatic height after wrapping-width/font changes also has a documented dependency correction; fixed-row layout is available |
| Linux disabled state | The AT-SPI adapter can report disabled controls as Enabled/Sensitive. Kumo's pointer/keyboard activation guards still reject unavailable actions; the defect concerns exported accessibility state |
| Linux busy state | Authored loading/busy state is not exported as AT-SPI Busy |
| Linux expansion state | Authored expanded/collapsed state is not exported as AT-SPI Expandable/Expanded |
| Animation tests | Published duration animations sample host time instead of the controlled test clock. This prevents deterministic intermediate-frame assertions; it is not evidence that normal native animations fail |

The unpatched macOS library gate measured 236 passing and 10 failing tests, compared with 246 passing in the patched workspace. Focus traversal and the same-update Textarea case were also reproduced natively. This macOS experiment did not validate the Linux adapter. The RC is experimental; account for these limitations before relying on its keyboard or platform accessibility behavior.

## Optional dependency corrections

The repository retains exact-version corrections for upstreaming and development. They are outside the library package and are not automatically applied when an application depends on Kumo. Cargo reads patches from the consuming workspace root, where they can replace the package used throughout that dependency graph. See [Cargo's patch rules](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html#the-patch-section).

To opt in, obtain the retained source in your application repository. This commit contains the documented vendor corrections:

```sh
git clone https://github.com/msmps/gpui-kumo.git third_party/gpui-kumo
git -C third_party/gpui-kumo checkout --detach 460077aaf53d1284d1e3a3500cfacb6faf01641f
```

Keep that checkout at the pinned revision (or copy the required vendor directories and their licenses). Put the following in your application's **workspace-root** `Cargo.toml`, alongside its ordinary dependencies. Paths are relative to that manifest:

```toml
[patch.crates-io]
gpui-pre = { path = "third_party/gpui-kumo/vendor/gpui-pre-0.3.7" }
gpui-base = { path = "third_party/gpui-kumo/vendor/gpui-base-0.7.0" }
accesskit_atspi_common = { path = "third_party/gpui-kumo/vendor/accesskit_atspi_common-0.19.1" }
```

Then run `cargo check` to resolve the overrides and update your lockfile; confirm the path sources with `cargo tree`. The AccessKit override is relevant to Linux targets. You may choose just the corrections you need: GPUI provides the focus/test-clock fixes, Base provides Textarea growth, and the adapter provides Linux state export.

For a compatible application, this changes the shared dependency resolution:

```text
application ──┬── gpui-kit 0.7.0 ── gpui-pre 0.3.7 (published or patched)
              └── gpui-kumo ────── gpui-kit 0.7.0 (the same package)
```

The patch replaces the existing GPUI package; it does not create a Kumo-specific GPUI that the app must bridge to. It also affects other libraries using that same patched package. It does not patch a separately sourced Git GPUI or adapt incompatible versions. Preserve the matching family and validate your entire application when opting in.

Patch details and removal conditions: [focus registration](https://github.com/msmps/gpui-kumo/blob/main/docs/gpui-tab-registration-patch.md), [animation clock](https://github.com/msmps/gpui-kumo/blob/main/docs/gpui-animation-clock-patch.md), [Textarea growth](https://github.com/msmps/gpui-kumo/blob/main/docs/gpui-base-textarea-growth-patch.md), [Linux disabled](https://github.com/msmps/gpui-kumo/blob/main/docs/linux-disabled-state-patch.md), [busy](https://github.com/msmps/gpui-kumo/blob/main/docs/linux-busy-state-validation.md) and [expansion](https://github.com/msmps/gpui-kumo/blob/main/docs/linux-expansion-state-validation.md).
