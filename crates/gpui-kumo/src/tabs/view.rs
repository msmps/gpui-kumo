use super::*;
use gpui_kit::{
    Animation, AnimationExt, Bounds, ColorSpace, Corners, DispatchPhase, MouseButton,
    MouseMoveEvent, MouseUpEvent, PathBuilder, Render, ScrollWheelEvent, canvas, linear_color_stop,
    linear_gradient, point, quad,
};
use std::time::Duration;
impl<T: Clone + Eq + 'static> Render for TabsState<T> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = theme(cx).clone();
        let sm = self.size == Size::Sm;
        let segmented = self.variant == Variant::Segmented;
        let height = if self.items.is_empty() {
            0.
        } else if segmented {
            if sm { 26. } else { 36. }
        } else if sm {
            26.
        } else {
            30.
        };
        let selected = self
            .items
            .iter()
            .position(|i| Some(&i.tab.value) == self.selected.as_ref());
        let (from, generation) = {
            let mut motion = self.indicator.borrow_mut();
            let target = selected.map(|i| self.items[i].tab.id.clone());
            if motion.target != target {
                motion.from = motion.current;
                motion.target = target;
                motion.generation += 1;
            }
            if cx.reduce_motion() {
                motion.from = None;
            }
            (motion.from, motion.generation)
        };
        let indicator = motion::Indicator {
            scroll: self.scroll.clone(),
            index: selected,
            motion: self.indicator.clone(),
            from,
            theme: theme.clone(),
            variant: self.variant,
            size: self.size,
            progress: 1.,
        }
        .element(from.is_some() && selected.is_some(), generation);
        let entry = self.entry(window);
        let overflowing = self.scroll.max_offset().x > px(1.);
        let dragging = self.drag.borrow().as_ref().is_some_and(|drag| drag.moved);
        let mut list = base::Tabs::new("list")
            .aria_label(self.name.clone())
            .flex()
            .relative()
            .min_w_0()
            .max_w_full()
            .h(px(height))
            .font_weight(FontWeight::MEDIUM)
            .overflow_x_scroll()
            .overflow_y_hidden()
            .track_scroll(&self.scroll)
            .capture_any_mouse_down(
                cx.listener(|state, event: &gpui_kit::MouseDownEvent, _, _| {
                    state.begin_drag(event.position.x, event.button)
                }),
            )
            .on_key_down(
                cx.listener(|state, event: &gpui_kit::KeyDownEvent, window, cx| {
                    let m = &event.keystroke.modifiers;
                    if m.control || m.alt || m.platform || m.shift || m.function {
                        return;
                    }
                    if state.navigate(&event.keystroke.key, window, cx) {
                        cx.stop_propagation();
                    }
                }),
            )
            .when(overflowing, |list| {
                list.cursor(if dragging {
                    gpui_kit::CursorStyle::ClosedHand
                } else {
                    gpui_kit::CursorStyle::OpenHand
                })
            })
            .when(segmented, |list| {
                list.bg(theme.colors.recessed)
                    .px(px(2.))
                    .rounded(px(if sm { 6. } else { 8. }))
            })
            .when(!segmented, |list| {
                list.gap(px(16.))
                    .pb(px(8.))
                    .border_b_1()
                    .border_color(theme.colors.hairline)
            })
            .child(indicator);
        for (index, item) in self.items.iter().enumerate() {
            let selected = Some(index) == selected;
            let disabled = self.disabled || item.tab.disabled;
            let owner = cx.entity().downgrade();
            let id = item.tab.id.clone();
            let content = item
                .tab
                .content
                .as_ref()
                .map(|f| f(window, cx))
                .unwrap_or_else(|| div().child(item.tab.label.clone()).into_any_element());
            let focus = item.focus.clone();
            let color = theme.colors.brand;
            let radius = px(if !segmented || sm { 4. } else { 6. });
            let ring = canvas(
                |_, _, _| (),
                move |bounds, _, window, _| {
                    if !disabled && focus.is_focused(window) && window.last_input_was_keyboard() {
                        let bounds = if segmented {
                            bounds
                        } else {
                            bounds.dilate(px(2.))
                        };
                        window.paint_quad(quad(
                            bounds,
                            radius,
                            color.alpha(0.),
                            px(2.),
                            color,
                            Default::default(),
                        ));
                    }
                },
            )
            .absolute()
            .inset_0()
            .size_full();
            let tab = base::Tab::new(item.tab.id.clone())
                .selected(selected)
                .disabled(disabled)
                .set_position(index + 1, self.items.len())
                .accessibility_label(
                    item.tab
                        .accessible_name
                        .clone()
                        .unwrap_or_else(|| item.tab.label.clone()),
                )
                .track_focus(
                    &item
                        .focus
                        .clone()
                        .tab_index(0)
                        .tab_stop(entry == Some(index)),
                )
                .flex_shrink_0()
                .a11y_synthetic_children(move |builder| {
                    if disabled {
                        builder.parent_node().set_disabled();
                    }
                })
                .text_size(px(if sm { 12. } else { 14. }))
                .line_height(px(if sm { 16. } else { 21. }))
                .text_color(if selected {
                    theme.text.default
                } else {
                    theme.text.subtle
                })
                .relative()
                .rounded(radius)
                .px(px(if segmented {
                    if sm { 8. } else { 10. }
                } else if sm {
                    6.
                } else {
                    8.
                }))
                .when(segmented, |tab| tab.my(px(2.)))
                .when(!segmented, |tab| tab.h(px(if sm { 20. } else { 24. })))
                .when(!disabled, |tab| {
                    tab.cursor(if overflowing {
                        if dragging {
                            gpui_kit::CursorStyle::ClosedHand
                        } else {
                            gpui_kit::CursorStyle::OpenHand
                        }
                    } else {
                        gpui_kit::CursorStyle::PointingHand
                    })
                    .hover(|style| {
                        let style = style.text_color(theme.text.default);
                        if segmented {
                            style
                        } else {
                            style.bg(theme.colors.tint)
                        }
                    })
                })
                .when(disabled, |tab| tab.opacity(0.5).cursor_not_allowed())
                .on_click(move |_, window, cx| {
                    let _ = owner.update(cx, |state, cx| state.activate(&id, window, cx));
                })
                .child(content)
                .child(ring);
            list = list.child(tab);
        }
        let visible = self.scroll_edges();
        let mut root = div()
            .id("tabs-surface")
            .relative()
            .min_w_0()
            .max_w_full()
            .h(px(height))
            .child(list);
        let owner = cx.entity().downgrade();
        let scroll = self.scroll.clone();
        let edges = self.edges.clone();
        let valid = self.layout_valid.clone();
        let disabled = self.disabled;
        let empty = self.items.is_empty();
        let background = self.fade_surface.unwrap_or(if segmented {
            theme.colors.recessed
        } else {
            theme.colors.base
        });
        let border = theme.colors.hairline.alpha(0.7);
        root = root.child(
            canvas(
                |_, _, _| (),
                move |bounds, _, window, cx| {
                    valid.set(true);
                    // Derived after the current frame's list prepaint/clamp, including resize/removal.
                    let position = -scroll.offset().x;
                    let max = scroll.max_offset().x;
                    let next = if disabled || !segmented || empty {
                        [false; 2]
                    } else {
                        [
                            max > px(1.) && position > px(1.),
                            max > px(1.) && max - position > px(1.),
                        ]
                    };
                    if edges.replace(next) != next {
                        let weak = owner.clone();
                        window.defer(cx, move |window, cx| {
                            let _ = weak.update(cx, |state, cx| {
                                state.restore_hidden_control(window, cx);
                                cx.notify();
                            });
                        });
                    }
                    if max > px(1.) && !empty {
                        for (side, remaining) in [position, max - position].into_iter().enumerate()
                        {
                            let width = remaining.min(px(48.)).max(px(0.));
                            if width <= px(0.) {
                                continue;
                            }
                            let mut fade = bounds;
                            fade.size.width = width;
                            if side == 1 {
                                fade.origin.x = bounds.right() - width;
                            }
                            let gradient = linear_gradient(
                                if side == 0 { 90. } else { 270. },
                                linear_color_stop(background, 0.),
                                linear_color_stop(background.alpha(0.), 1.),
                            )
                            .color_space(ColorSpace::Oklab);
                            let radius = px(if sm { 6. } else { 8. });
                            let corners = if segmented {
                                if side == 0 {
                                    Corners {
                                        top_left: radius,
                                        bottom_left: radius,
                                        ..Default::default()
                                    }
                                } else {
                                    Corners {
                                        top_right: radius,
                                        bottom_right: radius,
                                        ..Default::default()
                                    }
                                }
                            } else {
                                Corners::default()
                            };
                            window.paint_quad(quad(
                                fade,
                                corners,
                                gradient,
                                px(0.),
                                background,
                                Default::default(),
                            ));
                        }
                    }
                    if segmented && !empty {
                        window.paint_quad(quad(
                            bounds,
                            px(if sm { 6. } else { 8. }),
                            border.alpha(0.),
                            px(1.),
                            border,
                            Default::default(),
                        ));
                    }
                    let weak = owner.clone();
                    window.on_mouse_event(move |event: &MouseMoveEvent, phase, window, cx| {
                        if phase != DispatchPhase::Capture {
                            return;
                        }
                        let _ = weak.update(cx, |state, cx| {
                            if state.move_drag(event.position.x, event.pressed_button, cx) {
                                window.prevent_default();
                                cx.stop_propagation();
                            }
                        });
                    });
                    let weak = owner.clone();
                    window.on_mouse_event(move |event: &MouseUpEvent, phase, _, cx| {
                        if phase != DispatchPhase::Capture || event.button != MouseButton::Left {
                            return;
                        }
                        let _ = weak.update(cx, |state, cx| {
                            if state
                                .drag
                                .borrow_mut()
                                .take()
                                .is_some_and(|drag| drag.moved)
                            {
                                cx.stop_propagation();
                            }
                        });
                    });
                    let weak = owner.clone();
                    window.on_mouse_event(move |_: &ScrollWheelEvent, phase, _, cx| {
                        if phase == DispatchPhase::Capture {
                            let _ = weak.update(cx, |state, cx| {
                                if state.scroll_motion.borrow_mut().take().is_some() {
                                    cx.notify();
                                }
                            });
                        }
                    });
                },
            )
            .absolute()
            .inset_0()
            .size_full(),
        );
        if segmented {
            for (side, shown) in visible.into_iter().enumerate() {
                root = root.child(control(
                    ControlPresentation {
                        side,
                        sm,
                        shown,
                        motion: self.edge_motion.clone(),
                        reduced: cx.reduce_motion(),
                    },
                    &theme,
                    &self.controls[side],
                    if side == 0 {
                        self.labels.scroll_start.clone()
                    } else {
                        self.labels.scroll_end.clone()
                    },
                    cx.entity().downgrade(),
                ));
            }
        }
        let animation = *self.scroll_motion.borrow();
        let graphic = if let Some(animation) = animation {
            let scroll = self.scroll.clone();
            let from = animation.from;
            let target = animation.target;
            if cx.reduce_motion() {
                scroll.set_offset(point(target, px(0.)));
                self.scroll_motion.borrow_mut().take();
                root.into_any_element()
            } else {
                // Animate a separate clock child: the list and its accessible nodes
                // retain their identities when a new scroll action starts.
                root.child(div().absolute().w(px(0.)).h(px(0.)).with_animation(
                    ("tabs-scroll", animation.generation),
                    Animation::new(Duration::from_millis(200)).with_easing(motion::easing),
                    move |clock, progress| {
                        scroll.set_offset(point(from + (target - from) * progress, px(0.)));
                        clock
                    },
                ))
                .into_any_element()
            }
        } else {
            root.into_any_element()
        };
        lifecycle::Mounted {
            inner: div()
                .id("tabs-mount")
                .min_w_0()
                .max_w_full()
                .child(graphic)
                .into_any_element(),
            drag: self.drag.clone(),
            scroll: self.scroll_motion.clone(),
            indicator: self.indicator.clone(),
            valid: self.layout_valid.clone(),
            edges: self.edge_motion.clone(),
        }
    }
}
struct ControlPresentation {
    side: usize,
    sm: bool,
    shown: bool,
    motion: Rc<RefCell<[motion::EdgeMotion; 2]>>,
    reduced: bool,
}
fn control<T: Clone + Eq + 'static>(
    presentation: ControlPresentation,
    theme: &crate::Theme,
    focus: &FocusHandle,
    label: SharedString,
    owner: gpui_kit::WeakEntity<TabsState<T>>,
) -> AnyElement {
    let ControlPresentation {
        side,
        sm,
        shown,
        motion,
        reduced,
    } = presentation;
    let (from, generation) = {
        let mut states = motion.borrow_mut();
        let state = &mut states[side];
        if state.visible != shown {
            state.from = state.current;
            state.visible = shown;
            state.generation += 1;
        }
        if reduced {
            state.current = if shown { 1. } else { 0. };
            state.from = state.current;
        }
        (state.from, state.generation)
    };
    let target = if shown { 1. } else { 0. };
    let background = theme.colors.recessed;
    let gradient = canvas(
        |_, _, _| (),
        move |bounds, _, window, _| {
            let mut outer = bounds;
            outer.size.width /= 2.;
            let mut inner = outer;
            if side == 0 {
                inner.origin.x = outer.right();
            } else {
                outer.origin.x = inner.right();
            }
            let angle = if side == 0 { 90. } else { 270. };
            let radius = px(if sm { 6. } else { 8. });
            for (bounds, a, b, outside) in [
                (outer, background, background.alpha(0.95), true),
                (inner, background.alpha(0.95), background.alpha(0.), false),
            ] {
                let corners = if (side == 0) == outside {
                    Corners {
                        top_left: radius,
                        bottom_left: radius,
                        ..Default::default()
                    }
                } else {
                    Corners {
                        top_right: radius,
                        bottom_right: radius,
                        ..Default::default()
                    }
                };
                window.paint_quad(quad(
                    bounds,
                    corners,
                    linear_gradient(angle, linear_color_stop(a, 0.), linear_color_stop(b, 1.))
                        .color_space(ColorSpace::Oklab),
                    px(0.),
                    background,
                    Default::default(),
                ));
            }
        },
    )
    .absolute()
    .inset_0()
    .size_full();
    let glyph = canvas(
        |_, window, _| window.text_style().color,
        move |bounds, color, window, _| {
            let scale = 14. / 16.;
            let points = if side == 0 {
                [(9.25, 4.25), (5.75, 8.), (9.25, 11.75)]
            } else {
                [(6.75, 4.25), (10.25, 8.), (6.75, 11.75)]
            };
            let points = points.map(|(x, y)| bounds.origin + point(px(x * scale), px(y * scale)));
            let mut path = PathBuilder::stroke(px(2. * scale));
            path.move_to(points[0]);
            path.line_to(points[1]);
            path.line_to(points[2]);
            if let Ok(path) = path.build() {
                window.paint_path(path, color);
            }
            for p in points {
                window.paint_quad(quad(
                    Bounds {
                        origin: p - point(px(scale), px(scale)),
                        size: gpui_kit::size(px(2. * scale), px(2. * scale)),
                    },
                    px(scale),
                    color,
                    px(0.),
                    color,
                    Default::default(),
                ));
            }
        },
    )
    .size(px(14.));
    let ring_focus = focus.clone();
    let color = theme.colors.brand;
    let inner = div()
        .relative()
        .size(px(if sm { 20. } else { 24. }))
        .flex()
        .items_center()
        .justify_center()
        .when(side == 0, |v| v.ml(px(4.)))
        .when(side == 1, |v| v.mr(px(4.)))
        .child(glyph)
        .child(
            canvas(
                |_, _, _| (),
                move |bounds, _, window, _| {
                    if ring_focus.is_focused(window) && window.last_input_was_keyboard() {
                        window.paint_quad(quad(
                            bounds.dilate(px(2.)),
                            px(if sm { 6. } else { 8. }),
                            color.alpha(0.),
                            px(2.),
                            color,
                            Default::default(),
                        ));
                    }
                },
            )
            .absolute()
            .inset_0()
            .size_full(),
        );
    let graphic = div()
        .relative()
        .size_full()
        .flex()
        .items_center()
        .when(side == 0, |v| v.justify_start())
        .when(side == 1, |v| v.justify_end())
        .child(gradient)
        .child(inner);
    let graphic = if !reduced && from != target {
        graphic
            .with_animation(
                ("edge-opacity", generation),
                Animation::new(Duration::from_millis(150)).with_easing(motion::easing),
                move |graphic, progress| {
                    let opacity = from + (target - from) * progress;
                    motion.borrow_mut()[side].current = opacity;
                    graphic.opacity(opacity)
                },
            )
            .into_any_element()
    } else {
        graphic.opacity(target).into_any_element()
    };
    if !shown {
        // Visual fade-out only: no hidden action node, focus target or occlusion.
        return div()
            .absolute()
            .top_0()
            .h_full()
            .w(px(if sm { 32. } else { 40. }))
            .when(side == 0, |v| v.left_0())
            .when(side == 1, |v| v.right_0())
            .text_color(theme.text.subtle)
            .child(graphic)
            .into_any_element();
    }
    base::Button::new(if side == 0 {
        "scroll-start"
    } else {
        "scroll-end"
    })
    .accessibility_label(label)
    .occlude()
    .track_focus(focus)
    .absolute()
    .top_0()
    .h_full()
    .w(px(if sm { 32. } else { 40. }))
    .text_color(theme.text.subtle)
    .hover(|v| v.text_color(theme.text.default))
    .when(side == 0, |v| v.left_0())
    .when(side == 1, |v| v.right_0())
    .on_click(move |_, window, cx| {
        let _ = owner.update(cx, |state, cx| state.scroll_page(side, window, cx));
    })
    .child(graphic)
    .into_any_element()
}
