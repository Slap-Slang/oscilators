use std::f32::consts::PI;

use macroquad::{color::*, math::Vec2, shapes::draw_circle_lines};

use crate::extra_functions::circle;

use super::{callignment::CAlignment, curve::Curve, oscillator::Oscilator};

pub struct ContainerOscilator {
    hori_oscis: Vec<Oscilator>,
    verti_oscis: Vec<Oscilator>,
    size: f32,
    pub curves: Vec<Vec<Curve>>,
}

impl ContainerOscilator {
    pub fn new(size: f32) -> Self {
        Self {
            hori_oscis: Vec::new(),
            verti_oscis: Vec::new(),
            size,
            curves: Vec::new(),
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

        (0..self.hori_oscis.len()).into_iter().for_each(|_| {
            let mut col = Vec::new();
            (0..self.verti_oscis.len())
                .into_iter()
                .for_each(|_| col.push(Curve::new()));
            self.curves.push(col);
        })
    }

    pub fn update(&mut self, angle: f32) {
        for osci in &mut self.hori_oscis {
            osci.update(angle);
            
        }

        for osci in &mut self.verti_oscis {
            osci.update(angle);
        }
        println!();

        self.hori_oscis.iter().enumerate().for_each(|h_info| {
            let (i, h_osci) = h_info;

            let x = h_osci.pol_cord.x;
            self.verti_oscis.iter().enumerate().for_each(|v_info| {
                let (j, v_osci) = v_info;

                let y = v_osci.pol_cord.y;
                self.curves[i][j].add_point(x, y);
            })
        })
    }

    pub fn draw(&mut self) {
        for h_osci in &self.hori_oscis {
            h_osci.draw();
        }

        for osci in &self.verti_oscis {
            osci.draw();
        }

        self.curves
            .iter()
            .for_each(|curve_list| curve_list.iter().for_each(|curve| curve.draw()));

        // for vecs in &self.patterns {
        //     for point in vecs {
        //         draw_circle_lines(point.x, point.y, 0.1, 1.0, WHITE)
        //     }
        // }
    }
}
