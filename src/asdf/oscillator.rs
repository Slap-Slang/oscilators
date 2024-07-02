use macroquad::prelude::*;

use crate::extra_functions::circle;

use super::callignment::CAlignment;

pub struct Oscilator {
    pub center: Vec2,
    pub size: f32,
    pub cur_angle: f32,
    pub pol_cord: Vec2,
    pub speed: f32,
    pub ali: CAlignment,
}

impl Oscilator {
    pub fn new(center: Vec2, size: f32, speed: f32, ali: CAlignment) -> Self {
        Self {
            center,
            size,
            cur_angle: 0.0,
            pol_cord: Vec2::new(0.0, 0.0),
            speed,
            ali,
        }
    }

    pub fn draw(&self) {
        circle(
            self.center,
            self.size * 0.5 * 0.8,
            Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 0.15,
            },
        );

        draw_circle(self.pol_cord.x, self.pol_cord.y, 2.0, WHITE);

        draw_line(
            self.center.x,
            self.center.y,
            self.pol_cord.x,
            self.pol_cord.y,
            1.0,
            Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 0.15,
            },
        );

        match self.ali {
            CAlignment::Horizontal => draw_line(
                self.pol_cord.x,
                self.pol_cord.y,
                self.pol_cord.x,
                screen_height(),
                1.0,
                Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 0.15,
                },
            ),
            CAlignment::Vertical => draw_line(
                self.pol_cord.x,
                self.pol_cord.y,
                screen_width(),
                self.pol_cord.y,
                1.0,
                Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 0.15,
                },
            ),
        }
    }

    pub fn update(&mut self) {
        self.cur_angle += self.speed;
        self.pol_cord = Vec2::from_angle(self.cur_angle) * self.size * 0.8 * 0.5 + self.center;
    }
}
