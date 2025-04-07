use gol::{
    GameOfLife, GolErrors, insert_3_cell_line_patter_top_left_corner, insert_glider_patter_middle,
    insert_square_patter_top_left_corner,
};
use gol_gui::{draw_gol_board, toggle_cell};
use macroquad::prelude::*;

mod gol;
mod gol_gui;

const WIDTH: usize = 50;
const HEIGHT: usize = 50;
const SPEED: f64 = 0.3;

#[macroquad::main("BasicShapes")]
async fn main() -> Result<(), GolErrors> {
    let mut last_update = get_time();
    let mut gol = GameOfLife::new(WIDTH, HEIGHT);
    insert_3_cell_line_patter_top_left_corner(&mut gol)?;
    insert_square_patter_top_left_corner(&mut gol)?;
    insert_glider_patter_middle(&mut gol)?;
    let mut is_game_running = true;
    loop {
        clear_background(LIGHTGRAY);

        draw_gol_board(&gol);

        if is_key_pressed(KeyCode::Space) {
            is_game_running = !is_game_running;
        }

        if !is_game_running && is_mouse_button_pressed(MouseButton::Left) {
            let position = mouse_position();
            toggle_cell(&mut gol, position)?;
        }

        if is_game_running && get_time() - last_update > SPEED {
            last_update = get_time();
            gol.transition();
        }

        next_frame().await
    }
}
