# KUMO-060: Duration-animation clock consistency

GitHub: [#45](https://github.com/msmps/gpui-kumo/issues/45). Status: Implementation verified by full remote gate at85871b7; close after the final documentation push repeats that unchanged source gate. The resolution comment records the repeated run.

[Narrow patch, provenance, regression contract, skeptical review and removal/downstream obligations](../gpui-animation-clock-patch.md). Reproduced Switch failure in [run37115097659](https://github.com/msmps/gpui-kumo/actions/runs/37115097659/job/111180268250) at documentation-onlye06c866. #43 reopened.

Acceptance: consistent actual AnimationElement executor-clock initialization/elapsed/chained restart; preserve every Switch/Loader paint/reversal/reduced-motion assertion and native production monotonic-time contract; exercise a host stall and actual rendered chained bounds; workspace/exact adapter formatting/tests/doctests/warnings-denied Clippy/locked builds; successful repeated remote gate; review and published evidence. Native motion/preference/speech acceptance remains separate. Local formatting passes; Linux tests are dependency/proxy blocked. Full remote run37115645935 passes170 library tests, one gallery test, nine doctests, adapter12default/14all-feature tests, formatting/warnings-denied Clippy/locked builds. [Raw evidence](../evidence/animation-clock/README.md).
