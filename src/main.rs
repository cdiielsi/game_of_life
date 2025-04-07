use gol::{
    Cell, GameOfLife, GolErrors, insert_3_cell_line_vertical_pattern, insert_glider_pattern,
    insert_square_pattern,
};
use gol_gui::{draw_gol_board, toggle_cell};
use macroquad::prelude::*;

mod gol;
mod gol_gui;

const WIDTH: usize = 50;
const HEIGHT: usize = 50;
const SPEED: f64 = 0.3;

const START_FOR_PATTERN_1: Cell = Cell { x: 0, y: 1 };
const START_FOR_PATTERN_2: Cell = Cell {
    x: WIDTH - 2,
    y: HEIGHT - 2,
};
const START_FOR_PATTERN_3: Cell = Cell {
    x: WIDTH / 2,
    y: HEIGHT / 2,
};

#[macroquad::main("BasicShapes")]
async fn main() -> Result<(), GolErrors> {
    let mut last_update = get_time();
    let mut gol = GameOfLife::new(WIDTH, HEIGHT);
    insert_3_cell_line_vertical_pattern(&mut gol, START_FOR_PATTERN_1)?;
    insert_square_pattern(&mut gol, START_FOR_PATTERN_2)?;
    insert_glider_pattern(&mut gol, START_FOR_PATTERN_3)?;
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
