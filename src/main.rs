use macroquad::{miniquad::window::set_window_size, prelude::*};

const SCREEN_SIZE: f32 = 640f32;
const SQUARE_SIZE: f32 = SCREEN_SIZE / 8f32;
#[macroquad::main("Chess")]
async fn main() {
    set_window_size(SCREEN_SIZE as u32, SCREEN_SIZE as u32);
    loop {
        clear_background(BLACK);
        for i in 0..8 {
            for j in 0..8 {
                let color = match (i % 2 == 0, j % 2 == 0) {
                    (true, true) => WHITE,
                    (true, false) => BLACK,
                    (false, true) => BLACK,
                    (false, false) => WHITE,
                };
                draw_rectangle(
                    SQUARE_SIZE * j as f32,
                    SQUARE_SIZE * i as f32,
                    SQUARE_SIZE,
                    SQUARE_SIZE,
                    color,
                );
            }
        }
        draw_rectangle_lines(0., 0., SCREEN_SIZE, SCREEN_SIZE, 1., WHITE);
        next_frame().await
    }
}
