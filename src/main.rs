use gol::GameOfLife;
use macroquad::prelude::*;

mod gol;

const WIDTH: usize = 5;
const HEIGHT: usize = 5;

#[macroquad::main("BasicShapes")]
async fn main() {
    let speed = 0.3;
    let mut last_update = get_time();
    let mut gol = GameOfLife::new(WIDTH, HEIGHT);
    let mut game_of_life_go = true;
    loop {
        clear_background(WHITE);

        gol.draw_gol_board();

        if is_key_pressed(KeyCode::Space) {
                game_of_life_go = !game_of_life_go;
        } 

        if game_of_life_go && get_time() - last_update > speed {
            last_update = get_time();
            gol.transition();
        }

        next_frame().await
    }
}
