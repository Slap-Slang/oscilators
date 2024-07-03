use macroquad::{color::*, math::Vec2, shapes::draw_circle_lines};

pub struct Curve {
    pub path: Vec<Vec2>,
}

impl Curve {
    pub fn new() -> Self {
        Self { path: Vec::new() }
    }

    pub fn add_point(&mut self, x: f32, y: f32) {
        self.path.push(Vec2::new(x, y))
    }

    pub fn draw(&self) {
        self.path.iter().for_each(|point| {
            // draw_circle_lines(point.x, point.y, 0.1, 0.1, WHITE);
        });
    }
}
