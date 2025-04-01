use gol::GameOfLife;
use macroquad::prelude::*;

mod gol;

const WIDTH: usize = 5;
const HEIGHT: usize = 5;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut gol = GameOfLife::new(WIDTH, HEIGHT);
    loop {
        clear_background(WHITE);

        gol.draw_gol_board();
        gol.transition();

        next_frame().await
    }
}
