use std::f32::consts::PI;

use macroquad::{color::*, prelude::Vec2, shapes::draw_line};

pub fn circle(center: Vec2, r: f32, colour: Color) {
    let p1 = Vec2::from_angle(0.0) * r + center;
    let p2 = Vec2::from_angle(0.1) * r + center;
    draw_line(p1.x, p1.y, p2.x, p2.y, 1.0, colour);
    continue_circle(center, r, p2, 0.1, colour);
}

fn continue_circle(center: Vec2, r: f32, prev_point: Vec2, cur_angle: f32, colour: Color) {
    let next_point = Vec2::from_angle(cur_angle) * r + center;
    draw_line(
        prev_point.x,
        prev_point.y,
        next_point.x,
        next_point.y,
        1.0,
        colour,
    );
    if cur_angle < 2.0 * PI {
        continue_circle(center, r, next_point, cur_angle + 0.1, colour);
    }
}
