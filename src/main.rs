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

    let mut angle = 0.0f32;
    let angle_step = 0.1f32;

    for i in 0..10 {
        hori_os = Oscilator::new(
            Vec2::ZERO,
            0.0,
            0.1 * (i + 1) as f32,
            CAlignment::Horizontal,
        );
        verti_os = Oscilator::new(Vec2::ZERO, 0.0, 0.01 * (i + 1) as f32, CAlignment::Vertical);
        os_con.add_osci(hori_os);
        os_con.add_osci(verti_os);
    }

    os_con.order_oscilators();

    loop {
        os_con.update(angle);
        os_con.draw();
        angle += angle_step;

        draw_text(get_fps().to_string().as_str(), 13.0, 13.0, 25.0, YELLOW);
        // println!("{}", os_con.curves[0][0].path.len());

        next_frame().await;
    }
}
