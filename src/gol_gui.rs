use crate::gol::{Cell, GameOfLife, GolErrors};
use macroquad::prelude::*;

pub fn draw_gol_board(game_of_life: &GameOfLife) {
    let x_padding = screen_width() / game_of_life.width() as f32;
    let y_padding = screen_height() / game_of_life.height() as f32;
    for x in 0..game_of_life.width() {
        for y in 0..game_of_life.height() {
            let current_cell = Cell { x, y };
            if game_of_life.is_alive_cell(&current_cell) {
                draw_rectangle(
                    (x as f32 * x_padding) + 1.0,
                    (y as f32 * y_padding) + 1.0,
                    x_padding - 1.0,
                    y_padding - 1.0,
                    BLACK,
                );
            } else {
                draw_rectangle(
                    (x as f32 * x_padding) + 1.0,
                    (y as f32 * y_padding) + 1.0,
                    x_padding - 1.0,
                    y_padding - 1.0,
                    WHITE,
                );
            }
        }
    }
}

pub fn toggle_cell(game_of_life: &mut GameOfLife, position: (f32, f32)) -> Result<(), GolErrors> {
    let x_padding = screen_width() / game_of_life.width() as f32;
    let y_padding = screen_height() / game_of_life.height() as f32;
    let x_position = (position.0 / x_padding) as usize;
    let y_position = (position.1 / y_padding) as usize;

    let current_cell = Cell {
        x: x_position,
        y: y_position,
    };
    if game_of_life.is_alive_cell(&current_cell) {
        game_of_life.is_alive_cell(&current_cell);
        draw_rectangle(
            (x_position as f32 * x_padding) + 1.0,
            (y_position as f32 * y_padding) + 1.0,
            x_padding - 1.0,
            y_padding - 1.0,
            WHITE,
        );
    } else {
        game_of_life.add_living_cell(current_cell)?;
        draw_rectangle(
            (x_position as f32 * x_padding) + 1.0,
            (y_position as f32 * y_padding) + 1.0,
            x_padding - 1.0,
            y_padding - 1.0,
            BLACK,
        );
    }
    Ok(())
}
