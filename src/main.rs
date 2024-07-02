use macroquad::prelude::*;

use callignment::CAlignment;
use containter_oscillator::ContainerOscilator;
use oscilators::{asdf::*, extra_functions::circle};
use oscillator::Oscilator;

#[macroquad::main("oscilators")]
async fn main() {
    let mut os_con = ContainerOscilator::new(50.0);
    let mut hori_os: Oscilator;
    let mut verti_os: Oscilator;

    for i in 0..10 {
        hori_os = Oscilator::new(
            Vec2::ZERO,
            0.0,
            0.01 * (i + 1) as f32,
            CAlignment::Horizontal,
        );
        verti_os = Oscilator::new(Vec2::ZERO, 0.0, 0.01 * (i + 1) as f32, CAlignment::Vertical);
        os_con.add_osci(hori_os);
        os_con.add_osci(verti_os);
    }

    os_con.order_oscilators();

    loop {
        os_con.update();
        os_con.draw();

        next_frame().await;
    }
}
