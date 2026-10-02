//! Opt-in native frame measurement; excluded from ordinary gallery builds.

use std::{fs::OpenOptions, io::Write, time::Instant};

use gpui_kit::{
    App, Window,
    profiler::{FrameDurationSnapshot, InputLatencySnapshot},
};

gpui_kit::actions!(performance, [Start, Stop, ToggleMotion]);

#[derive(Default)]
pub(super) struct Measurement {
    active: Option<StartPoint>,
}

struct StartPoint {
    at: Instant,
    frames: FrameDurationSnapshot,
    input: InputLatencySnapshot,
    reduced_motion: bool,
}

impl Measurement {
    pub(super) fn start(&mut self, window: &Window, cx: &App) {
        self.active = Some(StartPoint {
            at: Instant::now(),
            frames: window.frame_duration_snapshot(),
            input: window.input_latency_snapshot(),
            reduced_motion: cx.reduce_motion(),
        });
    }

    pub(super) fn stop(&mut self, window: &Window, cx: &App) {
        let Some(start) = self.active.take() else {
            return;
        };
        if let Err(error) = Self::save(start, window, cx) {
            eprintln!("Could not save frame measurements: {error:#}");
        }
    }

    fn save(start: StartPoint, window: &Window, cx: &App) -> gpui_kit::Result<()> {
        if start.reduced_motion != cx.reduce_motion() {
            return Err(std::io::Error::other(
                "Motion mode changed during measurement; repeat with a fixed mode",
            )
            .into());
        }
        let elapsed = start.at.elapsed().as_secs_f64();
        let mut frames = window.frame_duration_snapshot();
        let mut input = window.input_latency_snapshot();
        let mut output = String::new();
        let profile = if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        };
        for (metric, current, previous) in [
            (
                "draw",
                &mut frames.draw_duration_histogram,
                &start.frames.draw_duration_histogram,
            ),
            (
                "dirty_to_present",
                &mut frames.dirty_to_present_histogram,
                &start.frames.dirty_to_present_histogram,
            ),
            (
                "animation_interval",
                &mut frames.present_interval_histogram,
                &start.frames.present_interval_histogram,
            ),
            (
                "input_to_frame",
                &mut input.latency_histogram,
                &start.input.latency_histogram,
            ),
        ] {
            current.subtract(previous)?;
            use std::fmt::Write as _;
            writeln!(
                &mut output,
                "{profile},{},{elapsed:.3},{metric},{},{:.4},{:.4},{:.4},{:.4},{:.4}",
                cx.reduce_motion(),
                current.len(),
                current.mean() / 1e6,
                current.value_at_quantile(0.5) as f64 / 1e6,
                current.value_at_quantile(0.95) as f64 / 1e6,
                current.value_at_quantile(0.99) as f64 / 1e6,
                current.max() as f64 / 1e6
            )?;
        }
        // CSV lives with disposable build artifacts, never in a user's data directory.
        let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target");
        std::fs::create_dir_all(&directory)?;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(directory.join("frame-measurements.csv"))?;
        file.write_all(output.as_bytes())?;
        Ok(())
    }
}
