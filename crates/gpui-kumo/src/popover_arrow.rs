//! Arrow geometry from Cloudflare Kumo's MIT-licensed Popover at
//! 3fd5b648df578cb1ba214dedd30f475009f6a668.
use crate::Theme;
use gpui_kit::{
    AnyElement, Bounds, IntoElement, PathBuilder, Pixels, Styled, base, canvas, point, px,
};
use std::{cell::Cell, rc::Rc};

pub(crate) fn element(
    trigger: Rc<Cell<Bounds<Pixels>>>,
    resolved: Rc<Cell<Option<base::ResolvedPosition>>>,
    theme: &Theme,
) -> AnyElement {
    element_with_inset(trigger, resolved, theme, px(18.))
}

pub(crate) fn element_with_inset(
    trigger: Rc<Cell<Bounds<Pixels>>>,
    resolved: Rc<Cell<Option<base::ResolvedPosition>>>,
    theme: &Theme,
    inset: Pixels,
) -> AnyElement {
    let colors = [
        theme.colors.base,
        theme.colors.arrow_edge,
        theme.colors.arrow_stroke,
    ];
    canvas(
        |_, _, _| (),
        move |_, _, window, _| {
            let Some(position) = resolved.get() else {
                return;
            };
            let Some(side) = position.placement else {
                return;
            };
            let bounds = position.bounds;
            let trigger = trigger.get();
            // Clamp the arrow clear of rounded corners after cross-axis snapping.
            if bounds.size.width < inset * 2. || bounds.size.height < inset * 2. {
                return;
            }
            let x = (trigger.center().x - bounds.left()).clamp(inset, bounds.size.width - inset);
            let y = (trigger.center().y - bounds.top()).clamp(inset, bounds.size.height - inset);
            let at = |u: f32, v: f32| match side {
                base::Placement::Bottom => {
                    point(bounds.left() + x + px(u - 10.), bounds.top() + px(v - 8.))
                }
                base::Placement::Top => point(
                    bounds.left() + x + px(10. - u),
                    bounds.bottom() + px(8. - v),
                ),
                base::Placement::Left => {
                    point(bounds.right() + px(8. - v), bounds.top() + y + px(u - 10.))
                }
                base::Placement::Right => {
                    point(bounds.left() + px(v - 8.), bounds.top() + y + px(10. - u))
                }
            };
            {
                let mut path = PathBuilder::fill();
                path.move_to(at(9.66437, 2.60207));
                path.line_to(at(4.80758, 6.97318));
                path.cubic_bezier_to(at(2.13172, 8.0), at(4.07308, 7.63423), at(3.11989, 8.0));
                path.line_to(at(0.0, 8.0));
                path.line_to(at(0.0, 10.0));
                path.line_to(at(20.0, 10.0));
                path.line_to(at(20.0, 8.0));
                path.line_to(at(18.5349, 8.0));
                path.cubic_bezier_to(at(15.8591, 6.97318), at(17.5468, 8.0), at(16.5936, 7.63423));
                path.line_to(at(11.0023, 2.60207));
                path.cubic_bezier_to(
                    at(9.66437, 2.60207),
                    at(10.622, 2.2598),
                    at(10.0447, 2.25979),
                );
                path.close();
                if let Ok(path) = path.build() {
                    window.paint_path(path, colors[0]);
                }
            }
            {
                let mut path = PathBuilder::fill();
                path.move_to(at(8.99542, 1.85876));
                path.cubic_bezier_to(
                    at(11.6713, 1.85878),
                    at(9.75604, 1.17425),
                    at(10.9106, 1.17422),
                );
                path.line_to(at(16.5281, 6.22989));
                path.cubic_bezier_to(
                    at(18.5349, 7.00001),
                    at(17.0789, 6.72568),
                    at(17.7938, 7.00001),
                );
                path.line_to(at(15.89, 7.0));
                path.line_to(at(11.0023, 2.60207));
                path.cubic_bezier_to(
                    at(9.66436, 2.60207),
                    at(10.622, 2.2598),
                    at(10.0447, 2.2598),
                );
                path.line_to(at(4.77734, 7.0));
                path.line_to(at(2.13171, 7.00001));
                path.cubic_bezier_to(
                    at(4.13861, 6.22989),
                    at(2.87284, 7.00001),
                    at(3.58774, 6.72568),
                );
                path.line_to(at(8.99542, 1.85876));
                path.close();
                if let Ok(path) = path.build() {
                    window.paint_path(path, colors[1]);
                }
            }
            {
                let mut path = PathBuilder::fill();
                path.move_to(at(10.3333, 3.34539));
                path.line_to(at(5.47654, 7.71648));
                path.cubic_bezier_to(at(2.13172, 9.0), at(4.55842, 8.54279), at(3.36693, 9.0));
                path.line_to(at(0.0, 9.0));
                path.line_to(at(0.0, 8.0));
                path.line_to(at(2.13172, 8.0));
                path.cubic_bezier_to(at(4.80758, 6.97318), at(3.11989, 8.0), at(4.07308, 7.63423));
                path.line_to(at(9.66437, 2.60207));
                path.cubic_bezier_to(
                    at(11.0023, 2.60207),
                    at(10.0447, 2.25979),
                    at(10.622, 2.2598),
                );
                path.line_to(at(15.8591, 6.97318));
                path.cubic_bezier_to(at(18.5349, 8.0), at(16.5936, 7.63423), at(17.5468, 8.0));
                path.line_to(at(20.0, 8.0));
                path.line_to(at(20.0, 9.0));
                path.line_to(at(18.5349, 9.0));
                path.cubic_bezier_to(at(15.1901, 7.71648), at(17.2998, 9.0), at(16.1083, 8.54278));
                path.line_to(at(10.3333, 3.34539));
                path.close();
                if let Ok(path) = path.build() {
                    window.paint_path(path, colors[2]);
                }
            }
        },
    )
    .absolute()
    .top_0()
    .left_0()
    .size_full()
    .into_any_element()
}
