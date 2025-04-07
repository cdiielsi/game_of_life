use gol::{
    Cell, GameOfLife, GolError, insert_glider_pattern, insert_line_vertical_pattern,
    insert_square_pattern,
};
use gol_gui::GolUI;
use macroquad::prelude::*;

mod gol;
mod gol_gui;

const WIDTH: usize = 50;
const HEIGHT: usize = 50;
const SPEED: f64 = 0.3;

const START_FOR_PATTERN_1: Cell = Cell { x: 0, y: 1 }; // Top left corner.
const START_FOR_PATTERN_2: Cell = Cell {
    // Bottom right corner.
    x: WIDTH - 2,
    y: HEIGHT - 2,
};
const START_FOR_PATTERN_3: Cell = Cell {
    // Middle of the board.
    x: WIDTH / 2,
    y: HEIGHT / 2,
};
const START_FOR_PATTERN_4: Cell = Cell { x: WIDTH - 3, y: 1 }; // Top right corner.

const LINE_PATTERN_SIZE: i32 = 3;
const SQUARE_PATTERN_SIZE: (usize, usize) = (2, 2);

#[macroquad::main("BasicShapes")]
async fn main() -> Result<(), GolError> {
    let mut last_update = get_time();
    let mut gol = GameOfLife::new(WIDTH, HEIGHT);
    let gol_ui = GolUI::new(screen_width(), screen_height(), &gol);
    insert_line_vertical_pattern(&mut gol, START_FOR_PATTERN_1, LINE_PATTERN_SIZE)?;
    insert_square_pattern(&mut gol, START_FOR_PATTERN_2, SQUARE_PATTERN_SIZE)?;
    insert_glider_pattern(&mut gol, START_FOR_PATTERN_3)?;
    insert_line_vertical_pattern(&mut gol, START_FOR_PATTERN_4, LINE_PATTERN_SIZE)?;
    let mut is_game_running = true;
    loop {
        clear_background(LIGHTGRAY);

        gol_ui.draw_gol_board(&gol);

        if is_key_pressed(KeyCode::Space) {
            is_game_running = !is_game_running;
        }

        if !is_game_running && is_mouse_button_pressed(MouseButton::Left) {
            let position = mouse_position();
            gol_ui.toggle_cell(&mut gol, position)?;
        }

        if is_game_running && get_time() - last_update > SPEED {
            last_update = get_time();
            gol.transition();
        }

        next_frame().await
    }
}
