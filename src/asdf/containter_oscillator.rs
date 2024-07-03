use std::f32::consts::PI;

use macroquad::{
    color::{WHITE, YELLOW},
    math::Vec2,
    shapes::draw_circle_lines,
};

use crate::extra_functions::circle;

use super::{callignment::CAlignment, oscillator::Oscilator};

pub struct ContainerOscilator {
    hori_oscis: Vec<Oscilator>,
    verti_oscis: Vec<Oscilator>,
    size: f32,
    pub patterns: Vec<Vec<Vec2>>,
}

impl ContainerOscilator {
    pub fn new(size: f32) -> Self {
        Self {
            hori_oscis: Vec::new(),
            verti_oscis: Vec::new(),
            size,
            patterns: Vec::new(),
        }
    }

    pub fn add_osci(&mut self, new_osci: Oscilator) {
        match new_osci.ali {
            CAlignment::Horizontal => self.hori_oscis.push(new_osci),
            CAlignment::Vertical => self.verti_oscis.push(new_osci),
        }
    }

    pub fn order_oscilators(&mut self) {
        self.hori_oscis.iter_mut().enumerate().for_each(|tup| {
            let i = tup.0 as f32;
            let osci = tup.1;

            osci.size = self.size;
            osci.center = Vec2::new(self.size / 2.0 + self.size * (i + 1.0), self.size / 2.0);
        });

        self.verti_oscis.iter_mut().enumerate().for_each(|tup| {
            let i = tup.0 as f32;
            let osci = tup.1;

            osci.size = self.size;
            osci.center = Vec2::new(self.size / 2.0, self.size / 2.0 + self.size * (i + 1.0));
        });

        self.hori_oscis.iter().for_each(|hor_osci| {
            self.verti_oscis.iter().for_each(|verti_osci| {
                let pattern_size = (2.0 * PI / hor_osci.speed) * (2.0 * PI / verti_osci.speed) / 100.0;
                let new_pattern = vec![Vec2::new(pattern_size, 0.0)];
                self.patterns.push(new_pattern);
                println!("{pattern_size}");
            });
            println!();
        })
    }

    pub fn update(&mut self) {
        for osci in &mut self.hori_oscis {
            osci.update();
        }

        for osci in &mut self.verti_oscis {
            osci.update();
        }
    }

    pub fn draw(&mut self) {
        let i = 0;
        for h_osci in &self.hori_oscis {
            h_osci.draw();
            let j = 0;
            for v_osci in &self.verti_oscis {
                draw_circle_lines(h_osci.pol_cord.x, v_osci.pol_cord.y, 1.0, 1.0, YELLOW);
                if self.patterns[i * j + i][0].x > self.patterns[i * j + i].len() as f32 {
                    self.patterns[i * j + i].push(Vec2::new(h_osci.pol_cord.x, v_osci.pol_cord.y));
                }
            }
        }

        for osci in &self.verti_oscis {
            osci.draw();
        }

        for vecs in &self.patterns {
            for point in vecs {
                draw_circle_lines(point.x, point.y, 0.1, 1.0, WHITE)
            }
        }
    }
}
