use crate::constraint::{XConstraint, YConstraint};
use concept::svg_init::{regex::RegexSet, RegexPatterns};
use guppies::{glam::Mat4, primitives::Rect, winit::dpi::PhysicalSize};

pub fn get_svg_size(svg_scale: Rect) -> Mat4 {
    Mat4::from_scale([svg_scale.size.x as f32, svg_scale.size.y as f32, 1.].into())
}

pub fn get_screen_size(size: PhysicalSize<u32>) -> Mat4 {
    Mat4::from_scale([size.width as f32, size.height as f32, 1.].into())
}

pub fn get_y_constraint(id: &str) -> YConstraint {
    YConstraint::Center(0.)
}

pub fn get_x_constraint(id: &str) -> XConstraint {
    let mut regex_patterns = RegexPatterns::default();
    let menu = regex_patterns.add(r"Menu #transform");
    let grab = regex_patterns.add(r"Grab #transform");
    let undo = regex_patterns.add(r"Undo #transform");
    let constraint_regex =
        RegexSet::new(regex_patterns.inner.iter().map(|r| &r.regex_pattern)).unwrap();
    let matches = constraint_regex.matches(id);
    if matches.matched(menu.index) {
        XConstraint::Left(15.)
    } else if matches.matched(grab.index) {
        XConstraint::Center(0.)
    } else if matches.matched(undo.index) {
        XConstraint::Right(-15.)
    } else {
        XConstraint::Scale
    }
}
