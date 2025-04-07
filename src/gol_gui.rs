use crate::gol::{Cell, GameOfLife, GolErrors};
use macroquad::prelude::*;

pub struct GolUI {
    x_size: f32,
    y_size: f32,
}

impl GolUI {
    pub fn new(screen_width: f32, screen_height: f32, game_of_life: &GameOfLife) -> Self {
        Self {
            x_size: screen_width / game_of_life.width() as f32,
            y_size: screen_height / game_of_life.height() as f32,
        }
    }

    pub fn draw_gol_board(&self, game_of_life: &GameOfLife) {
        for x in 0..game_of_life.width() {
            for y in 0..game_of_life.height() {
                let current_cell = Cell { x, y };
                self.draw_cell(current_cell, game_of_life.is_alive_cell(&current_cell));
            }
        }
    }

    fn get_cell_from_mouse_position(&self, mouse_position: (f32, f32)) -> Cell {
        Cell {
            x: (mouse_position.0 / self.x_size) as usize,
            y: (mouse_position.1 / self.y_size) as usize,
        }
    }

    pub fn toggle_cell(
        &self,
        game_of_life: &mut GameOfLife,
        mouse_position: (f32, f32),
    ) -> Result<(), GolErrors> {
        let current_cell = self.get_cell_from_mouse_position(mouse_position);
        game_of_life.toggle_cell(current_cell)
    }

    fn draw_cell(&self, cell: Cell, is_alive: bool) {
        if is_alive {
            draw_rectangle(
                (cell.x as f32 * self.x_size) + 1.0,
                (cell.y as f32 * self.y_size) + 1.0,
                self.x_size - 1.0,
                self.y_size - 1.0,
                BLACK,
            );
        } else {
            draw_rectangle(
                (cell.x as f32 * self.x_size) + 1.0,
                (cell.y as f32 * self.y_size) + 1.0,
                self.x_size - 1.0,
                self.y_size - 1.0,
                WHITE,
            );
        }
    }
}
