use std::{f32::consts::PI,};

use macroquad::prelude::*;

#[macroquad::main("oscilators")]
async fn main() {
    let mut os = Oscilator::new(
        vec2(screen_width() / 2.0, screen_height() / 2.0),
        screen_height() / 2.0 - 10.0,
        0.01,
        CAlignment::Horizontal,
    );

    let mut os_con = ContainerOscilator::new();
    os_con.add_osci(os);

    loop {
        // os.update();
        os_con.draw();

        next_frame().await;
    }
}

fn circle(center: Vec2, r: f32, start: f32) {
    let p1 = Vec2::from_angle(start) * r + center;
    let p2 = Vec2::from_angle(start + 0.1) * r + center;
    draw_line(p1.x, p1.y, p2.x, p2.y, 1.0, WHITE);
    if start < 2.0 * PI {
        circle(center, r, start + 0.1);
    }
}

enum CAlignment {
    Horizontal,
    Vertical,
}

struct Oscilator {
    center: Vec2,
    size: f32,
    cur_angle: f32,
    pol_cord: Vec2,
    speed: f32,
    ali: CAlignment,
}

impl Oscilator {
    fn new(center: Vec2, size: f32, speed: f32, ali: CAlignment) -> Self {
        Self {
            center,
            size,
            cur_angle: 0.0,
            pol_cord: Vec2::new(0.0, 0.0),
            speed,
            ali,
        }
    }

    fn draw(&self) {
        // draw_circle_lines(self.center.x, self.center.y, self.size * 0.8, 1.0, WHITE);
        circle(self.center, self.size * 0.8, 0.0);
        draw_circle(self.pol_cord.x, self.pol_cord.y, 4.0, WHITE);
        draw_line(
            self.center.x,
            self.center.y,
            self.pol_cord.x,
            self.pol_cord.y,
            1.0,
            WHITE,
        );
    }

    fn update(&mut self) {
        self.cur_angle += self.speed;
        self.pol_cord = Vec2::from_angle(self.cur_angle) * self.size * 0.8 + self.center;
    }
}

struct ContainerOscilator {
    hori_oscis: Vec<Oscilator>,
    verti_oscis: Vec<Oscilator>,
}

impl ContainerOscilator {
    fn new() -> Self {
        Self {
            hori_oscis: Vec::new(),
            verti_oscis: Vec::new(),
        }
    }

    fn add_osci(&mut self, new_osci: Oscilator) {
        match new_osci.ali {
            CAlignment::Horizontal => self.hori_oscis.push(new_osci),
            CAlignment::Vertical => self.verti_oscis.push(new_osci),
        }
    }

    fn draw(&self) {
        for osci in &self.hori_oscis {
            osci.draw();
        }

        for osci in &self.verti_oscis {
            osci.draw();
        }
    }
}
