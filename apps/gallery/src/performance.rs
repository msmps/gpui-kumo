//! Opt-in native frame measurement; excluded from ordinary gallery builds.

use std::{cell::Cell, fs::OpenOptions, io::Write, rc::Rc, time::Instant};

use gpui_kit::{
    App, Subscription, Window,
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
    hidden: Rc<Cell<bool>>,
    _visibility_subscription: Subscription,
}

impl Measurement {
    pub(super) fn start(&mut self, window: &Window, cx: &App) {
        let hidden = Rc::new(Cell::new(!window.is_visible()));
        let visibility_subscription = window.observe_window_visibility({
            let hidden = hidden.clone();
            move |visibility, _, _| {
                if !visibility.is_visible() {
                    hidden.set(true);
                }
            }
        });
        self.active = Some(StartPoint {
            at: Instant::now(),
            frames: window.frame_duration_snapshot(),
            input: window.input_latency_snapshot(),
            reduced_motion: cx.reduce_motion(),
            hidden,
            _visibility_subscription: visibility_subscription,
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

    fn validate(start: &StartPoint, window: &Window, cx: &App) -> gpui_kit::Result<()> {
        if start.hidden.get() || !window.is_visible() {
            return Err(std::io::Error::other(
                "Window was hidden during measurement; raise it and repeat while Presentation is Visible",
            )
            .into());
        }
        if start.reduced_motion != cx.reduce_motion() {
            return Err(std::io::Error::other(
                "Motion mode changed during measurement; repeat with a fixed mode",
            )
            .into());
        }
        Ok(())
    }

    fn save(start: StartPoint, window: &Window, cx: &App) -> gpui_kit::Result<()> {
        Self::validate(&start, window, cx)?;
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

#[cfg(test)]
mod tests {
    use gpui_kit::{Empty, IntoElement, Render, TestAppContext, WindowVisibility};

    use super::*;

    struct Probe;

    impl Render for Probe {
        fn render(&mut self, _: &mut Window, _: &mut gpui_kit::Context<Self>) -> impl IntoElement {
            Empty
        }
    }

    #[gpui_kit::test]
    fn measurement_rejects_occlusion_even_after_presentation_resumes(cx: &mut TestAppContext) {
        let window = cx.add_window(|_, _| Probe);
        let mut measurement = Measurement::default();
        window
            .update(cx, |_, window, cx| measurement.start(window, cx))
            .unwrap();
        window
            .update(cx, |_, window, cx| {
                assert!(
                    Measurement::validate(measurement.active.as_ref().unwrap(), window, cx).is_ok()
                );
            })
            .unwrap();

        cx.simulate_window_visibility_change(window.into(), WindowVisibility::Hidden);
        cx.simulate_window_visibility_change(window.into(), WindowVisibility::Visible);
        window
            .update(cx, |_, window, cx| {
                assert!(
                    Measurement::validate(measurement.active.as_ref().unwrap(), window, cx)
                        .is_err()
                );
                // Restarting drops the old interval's subscription and invalidation.
                measurement.start(window, cx);
                assert!(
                    Measurement::validate(measurement.active.as_ref().unwrap(), window, cx).is_ok()
                );
            })
            .unwrap();

        cx.simulate_window_visibility_change(window.into(), WindowVisibility::Hidden);
        window
            .update(cx, |_, window, cx| measurement.start(window, cx))
            .unwrap();
        cx.simulate_window_visibility_change(window.into(), WindowVisibility::Visible);
        window
            .update(cx, |_, window, cx| {
                assert!(
                    Measurement::validate(measurement.active.as_ref().unwrap(), window, cx)
                        .is_err()
                );
            })
            .unwrap();
    }
}
