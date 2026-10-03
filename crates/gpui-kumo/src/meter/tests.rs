use super::*;
use gpui_kit::{Context, Element, Render, TestAppContext, test::TestWindowExt};
use std::{cell::RefCell, rc::Rc};

struct Harness {
    value: f64,
    width: f32,
    label: SharedString,
    range: RangeInclusive<f64>,
    custom: Option<SharedString>,
    show: bool,
    nodes: Rc<RefCell<Vec<gpui_kit::accesskit::Node>>>,
}
impl Render for Harness {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut meter = Meter::new("meter", self.label.clone(), self.value)
            .range(self.range.clone())
            .show_value(self.show);
        if let Some(custom) = &self.custom {
            meter = meter.custom_value(custom.clone());
        }
        let element = meter.render(window, cx).into_element();
        let mut node = gpui_kit::accesskit::Node::new(Role::Unknown);
        element.write_a11y_info(&mut node);
        self.nodes.borrow_mut().push(node);
        div().w(gpui_kit::px(self.width)).child(element)
    }
}
#[gpui_kit::test]
fn normalization_semantics_text_precedence_and_actual_geometry(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let nodes = Rc::new(RefCell::new(Vec::new()));
    let (view, cx) = cx.add_window_view(|_, _| Harness {
        value: 65.,
        width: 200.,
        label: "Storage café 🦀".into(),
        range: 0.0..=100.0,
        custom: None,
        show: true,
        nodes: nodes.clone(),
    });
    cx.update(|window, cx| {
        cx.set_reduce_motion(true);
        for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            for (raw, range, value, percentage, text) in [
                (65., 0.0..=100.0, 65., 65., "65%"),
                (2.5, 0.0..=100.0, 2.5, 2.5, "3%"),
                // Preserve the pinned Base UI arithmetic, including huge-number overflow.
                (1e307, 0.0..=1e308, 1e307, 100., "100%"),
                (175., 100.0..=200.0, 175., 75., "75%"),
                (f64::NAN, 10.0..=20.0, 10., 0., "0%"),
                (f64::INFINITY, 10.0..=20.0, 20., 100., "100%"),
                (f64::NEG_INFINITY, 10.0..=20.0, 10., 0., "0%"),
                (10., 10.0..=10.0, 10., 0., "0%"),
                (11., 10.0..=10.0, 10., 100., "100%"),
            ] {
                view.update(cx, |v, _| {
                    v.value = raw;
                    v.range = range.clone();
                });
                window.render_frame(cx);
                let indicator = window.find("indicator").bounds();
                assert!(
                    (f32::from(indicator.size.width) - (200. * percentage / 100.) as f32).abs()
                        <= 0.5
                );
                assert_eq!(indicator.size.height, gpui_kit::px(8.));
                assert_eq!(window.find("meter").role(), Some(Role::Meter));
                assert_eq!(window.find("meter").value(), Some(text));
                let node = nodes.borrow().last().unwrap().clone();
                assert_eq!(node.numeric_value(), Some(value));
                assert_eq!(node.min_numeric_value(), Some(*range.start()));
                assert_eq!(node.max_numeric_value(), Some(*range.end()));
                assert_eq!(node.label(), Some("Storage café 🦀"));
            }
            assert!(
                window.try_find("value").is_none(),
                "default value must not duplicate root semantics"
            );
            view.update(cx, |v, _| {
                v.custom = Some("750 / 1,000".into());
                v.show = false;
            });
            window.render_frame(cx);
            assert_eq!(window.find("value").label(), Some("750 / 1,000"));
            assert_eq!(window.find("meter").value(), Some("100%"));
            view.update(cx, |v, _| v.custom = Some("".into()));
            window.render_frame(cx);
            assert!(window.try_find("value").is_none());
            view.update(cx, |v, _| {
                v.show = true;
                v.custom = None;
            });
            window.render_frame(cx);
        }
    });
}
#[gpui_kit::test]
fn base_width_motion_reversal_semantic_target_and_reduced_motion(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, _| Harness {
        value: 0.,
        width: 200.,
        label: "Storage café 🦀".into(),
        range: 0.0..=100.0,
        custom: None,
        show: true,
        nodes: Rc::new(RefCell::new(Vec::new())),
    });
    cx.update(|window, cx| {
        window.render_frame(cx);
        view.update(cx, |v, _| v.value = 100.);
        window.render_frame(cx);
        assert_eq!(window.find("meter").value(), Some("100%"));
        assert_eq!(
            window.find("indicator").bounds().size.width,
            gpui_kit::px(0.)
        );
        assert!(window.simulate_next_frame(cx) > 0);
    });
    cx.background_executor
        .advance_clock(Duration::from_millis(150));
    cx.update(|window, cx| {
        window.render_frame(cx);
        let midpoint = window.find("indicator").bounds().size.width;
        assert!(midpoint > gpui_kit::px(100.) && midpoint < gpui_kit::px(200.));
        view.update(cx, |v, _| v.value = 0.);
        window.render_frame(cx);
        assert_eq!(window.find("meter").value(), Some("0%"));
        assert_eq!(window.find("indicator").bounds().size.width, midpoint);
    });
    cx.background_executor
        .advance_clock(Duration::from_millis(300));
    cx.update(|window, cx| {
        window.simulate_next_frame(cx);
        window.render_frame(cx);
        assert_eq!(
            window.find("indicator").bounds().size.width,
            gpui_kit::px(0.)
        );
        cx.set_reduce_motion(true);
        view.update(cx, |v, _| v.value = 45.);
        window.render_frame(cx);
        assert_eq!(
            window.find("indicator").bounds().size.width,
            gpui_kit::px(90.)
        );
        window.simulate_next_frame(cx);
        window.render_frame(cx);
        assert_eq!(window.simulate_next_frame(cx), 0);
    });
}

#[gpui_kit::test]
fn narrow_long_unicode_label_and_custom_value_remain_inside_track(cx: &mut TestAppContext) {
    cx.update(crate::init);
    let (view, cx) = cx.add_window_view(|_, _| Harness {
        value: 42.,
        width: 240.,
        label: "Long quota café 🦀 regional archive account".into(),
        range: 0.0..=100.0,
        custom: Some("42 / 100 GB retained across regional archives".into()),
        show: false,
        nodes: Rc::new(RefCell::new(Vec::new())),
    });
    cx.update(|window, cx| {
        for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            for width in [240., 160.] {
                view.update(cx, |v, _| v.width = width);
                window.render_frame(cx);
                let root = window.find("meter").bounds();
                let label = window.find("label").bounds();
                let value = window.find("value").bounds();
                assert!(label.left() >= root.left() && label.right() <= root.right());
                assert!(label.top() >= root.top() && label.bottom() <= root.bottom());
                assert!(value.left() >= root.left() && value.right() <= root.right());
                assert!(value.top() >= root.top() && value.bottom() <= root.bottom());
                assert!(label.right() <= value.left());
            }
        }
    });
}

#[gpui_kit::test]
fn native_formatting_and_spoken_override_keep_numeric_range_authoritative(cx: &mut TestAppContext) {
    struct Formatted;
    impl Render for Formatted {
        fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let meter = Meter::new("temperature", "Temperature", 75.)
                .range(-20.0..=40.0)
                .format(|measurement| {
                    assert_eq!(measurement.raw, 75.);
                    assert_eq!(measurement.value, 40.);
                    assert_eq!(measurement.percentage, 100.);
                    format!("{:.0} °C", measurement.value).into()
                })
                .value_text(|formatted, measurement| {
                    assert_eq!(formatted, "40 °C");
                    assert_eq!(measurement.max, 40.);
                    "Forty degrees Celsius".into()
                });
            let element = meter.render(window, cx).into_element();
            let mut node = gpui_kit::accesskit::Node::new(Role::Unknown);
            element.write_a11y_info(&mut node);
            assert_eq!(node.numeric_value(), Some(40.));
            assert_eq!(node.min_numeric_value(), Some(-20.));
            assert_eq!(node.max_numeric_value(), Some(40.));
            div().w(gpui_kit::px(240.)).child(element)
        }
    }
    cx.update(crate::init);
    let (_, cx) = cx.add_window_view(|_, _| Formatted);
    cx.update(|window, cx| {
        window.render_frame(cx);
        assert_eq!(
            window.find("temperature").value(),
            Some("Forty degrees Celsius")
        );
        assert!(window.try_find("value").is_none());
    });
}

#[gpui_kit::test]
fn caller_track_and_fill_styles_change_paint_without_changing_value(cx: &mut TestAppContext) {
    struct StyledMeter;
    impl Render for StyledMeter {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let t = theme(cx);
            div().w(gpui_kit::px(160.)).child(
                Meter::new("styled", "Quota café", 25.)
                    .track_style(
                        StyleRefinement::default()
                            .h(gpui_kit::px(12.))
                            .rounded(gpui_kit::px(4.))
                            .bg(t.colors.tint),
                    )
                    .indicator_style(
                        StyleRefinement::default()
                            .rounded(gpui_kit::px(4.))
                            .bg(t.colors.success),
                    ),
            )
        }
    }
    cx.update(crate::init);
    let (_, cx) = cx.add_window_view(|_, _| StyledMeter);
    cx.update(|window, cx| {
        cx.set_reduce_motion(true);
        for appearance in [crate::Appearance::Light, crate::Appearance::Dark] {
            crate::set_appearance(appearance, cx);
            window.render_frame(cx);
            let fill = window.find("indicator").bounds();
            assert_eq!(
                fill.size,
                gpui_kit::size(gpui_kit::px(40.), gpui_kit::px(12.))
            );
            assert_eq!(window.find("styled").value(), Some("25%"));
            let t = theme(cx);
            let quads = window.painted_quads();
            let track = quads
                .iter()
                .find(|q| q.background == gpui_kit::Background::from(t.colors.tint))
                .unwrap();
            let paint = quads
                .iter()
                .find(|q| q.background == gpui_kit::Background::from(t.colors.success))
                .unwrap();
            assert_eq!(paint.bounds, fill.scale(window.scale_factor()));
            assert_eq!(
                track.bounds.size.width.as_f32(),
                160. * window.scale_factor()
            );
            assert_eq!(track.corner_radii, paint.corner_radii);
        }
    });
}
