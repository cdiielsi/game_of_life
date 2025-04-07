use macroquad::prelude::*;
use std::error::Error;
use std::{cmp::min, collections::HashSet};

/// Implementation of game of life. The struct has two fields for the boards dimentions and another one that
/// stores a set to register the living cells. After each transition this set is updated with the resulting
/// living cells.
pub struct GameOfLife {
    width: usize,
    height: usize,
    alive_cells: HashSet<Cell>,
}

/// Cell is a strcut with two fields that reference the cell's coordinates on the board.
#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Cell {
    x: usize,
    y: usize,
}
#[derive(PartialEq, Debug)]
pub enum GolErrors {
    IndexOutOfBound,
}

impl GameOfLife {
    /// Creates an instance of a Game of Life with an empty set of alive cells and the dimentions of the board.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            alive_cells: HashSet::new(),
        }
    }
    /// Adds a cell to the alive cells set.
    /// TODO: handle coordinates outiside the boards range with error
    fn add_living_cell(&mut self, cell: Cell) -> Result<(), GolErrors> {
        if cell.x < self.width && cell.y < self.height {
            self.alive_cells.insert(cell);
            return Ok(());
        }
        Err(GolErrors::IndexOutOfBound)
    }

    /// Updates alive_cell field with the new living cells.
    pub fn transition(&mut self) -> Option<()> {
        let mut next_iteration_set = HashSet::new();
        for x in 0..self.width {
            for y in 0..self.height {
                let current_cell = Cell { x, y };
                if self.cell_next_state_is_alive(&current_cell){
                    next_iteration_set.insert(current_cell);
                }
            }
        }
        self.alive_cells = next_iteration_set;
        Some(())
    }

    /// Returns true if in the next iteration the cell in question is alive according to the following rules:
    /// - Any live cell with fewer than two live neighbours dies, as if by underpopulation.
    /// - Any live cell with two or three live neighbours lives on to the next generation.
    /// - Any live cell with more than three live neighbours dies, as if by overpopulation.
    /// - Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
    fn cell_next_state_is_alive(
        &self,
        current_cell: &Cell,
    )->bool {
        let neighbours = self.count_cell_living_neighbours(current_cell);
        if neighbours == 3 || self.alive_cells.contains(current_cell) && neighbours == 2 {
            return true;
        }
        false
    }

    fn count_cell_living_neighbours(&self, current_cell: &Cell)->i32{
        let mut neighbours = 0;
        let (x_start, x_end) = self.get_range_for_neighbourhood(current_cell.x, self.width);
        let (y_start, y_end) = self.get_range_for_neighbourhood(current_cell.y, self.height);
        for x in x_start..x_end {
            for y in y_start..y_end {
                let neighbour_cell = Cell { x, y };
                if self.alive_cells.contains(&neighbour_cell) && neighbour_cell != *current_cell {
                    neighbours += 1;
                }
            }
        }
        neighbours
    }

    /// Provides the indexes that determine a cell's neighbourhood.
    fn get_range_for_neighbourhood(&self, cell_component: usize, limit: usize) -> (usize, usize) {
        let start_range = match (cell_component).overflowing_sub(1) {
            (n, false) => n,
            (_, true) => cell_component,
        };
        let end_range = match (cell_component).overflowing_add(2) {
            (n, false) => min(n, limit),
            (_, true) => limit,
        };
        // cell_component is supposed to work as an index either in the range 0..width or the range 0..height,
        // in that sense start_range should be an inclusive bound and end_range should be an exclusive bound.
        // If cell_component + 2 overflows, it means that limit = usize::MAX and cell_component = usize::MAX - 1,
        // so end_range should be limit.

        (start_range, end_range)
    }

    pub fn draw_gol_board(&self) {
        let x_padding = screen_width() / self.width as f32;
        let y_padding = screen_height() / self.height as f32;
        for i in 0..self.width {
            for j in 0..self.height {
                let current_cell = Cell { x: i, y: j };
                if self.alive_cells.contains(&current_cell) {
                    draw_rectangle(
                        (i as f32 * x_padding) + 1.0,
                        (j as f32 * y_padding) + 1.0,
                        x_padding - 1.0,
                        y_padding - 1.0,
                        BLACK,
                    );
                } else {
                    draw_rectangle(
                        (i as f32 * x_padding) + 1.0,
                        (j as f32 * y_padding) + 1.0,
                        x_padding - 1.0,
                        y_padding - 1.0,
                        WHITE,
                    );
                }
            }
        }
    }

    pub fn toggle_cell(&mut self, position: (f32, f32)) {
        let x_padding = screen_width() / self.width as f32;
        let y_padding = screen_height() / self.height as f32;
        let x_position = (position.0 / x_padding) as usize;
        let y_position = (position.1 / y_padding) as usize;

        let current_cell = Cell {
            x: x_position,
            y: y_position,
        };
        if self.alive_cells.contains(&current_cell) {
            self.alive_cells.remove(&current_cell);
            draw_rectangle(
                (x_position as f32 * x_padding) + 1.0,
                (y_position as f32 * y_padding) + 1.0,
                x_padding - 1.0,
                y_padding - 1.0,
                WHITE,
            );
        } else {
            self.add_living_cell(current_cell);
            draw_rectangle(
                (x_position as f32 * x_padding) + 1.0,
                (y_position as f32 * y_padding) + 1.0,
                x_padding - 1.0,
                y_padding - 1.0,
                BLACK,
            );
        }
    }
}

pub fn insert_3_cell_line_patter_top_left_corner(gol: &mut GameOfLife) -> Result<(), GolErrors> {
    if gol.width > 2 && gol.height > 2 {
        gol.add_living_cell(Cell { x: 0, y: 1 })?;
        gol.add_living_cell(Cell { x: 1, y: 1 })?;
        gol.add_living_cell(Cell { x: 2, y: 1 })?;
    }
    Ok(())
}

pub fn insert_square_patter_top_left_corner(gol: &mut GameOfLife) -> Result<(), GolErrors> {
    if gol.width > 1 && gol.height > 1 {
        gol.add_living_cell(Cell {
            x: gol.width - 1,
            y: gol.height - 1,
        })?;
        gol.add_living_cell(Cell {
            x: gol.width - 2,
            y: gol.height - 1,
        })?;
        gol.add_living_cell(Cell {
            x: gol.width - 2,
            y: gol.height - 2,
        })?;
        gol.add_living_cell(Cell {
            x: gol.width - 1,
            y: gol.height - 2,
        })?;
    }
    Ok(())
}

pub fn insert_glider_patter_middle(gol: &mut GameOfLife) -> Result<(), GolErrors> {
    if gol.width > 2 && gol.height > 2 {
        gol.add_living_cell(Cell {
            x: gol.width / 2,
            y: gol.height / 2 + 1,
        })?;
        gol.add_living_cell(Cell {
            x: gol.width / 2 + 1,
            y: gol.height / 2 + 2,
        })?;
        gol.add_living_cell(Cell {
            x: gol.width / 2 + 1,
            y: gol.height / 2 + 3,
        })?;
        gol.add_living_cell(Cell {
            x: gol.width / 2 + 2,
            y: gol.height / 2 + 2,
        })?;
        gol.add_living_cell(Cell {
            x: gol.width / 2 + 2,
            y: gol.height / 2 + 1,
        })?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn range_of_neighbourhood_for_5x5_board() {
        let mut gol = GameOfLife::new(5, 5);
        insert_3_cell_line_patter_top_left_corner(&mut gol);
        let mut current_cell = Cell { x: 0, y: 0 }; // This test covers case of underflow
        let (x_star, x_end) = gol.get_range_for_neighbourhood(current_cell.x, gol.width);
        let (y_star, y_end) = gol.get_range_for_neighbourhood(current_cell.y, gol.height);
        assert_eq!(x_star, 0);
        assert_eq!(x_end, 2);
        assert_eq!(y_star, 0);
        assert_eq!(y_end, 2);

        current_cell = Cell { x: 2, y: 2 };
        let (x_start, x_end) = gol.get_range_for_neighbourhood(current_cell.x, gol.width);
        let (y_start, y_end) = gol.get_range_for_neighbourhood(current_cell.y, gol.height);
        assert_eq!(x_start, 1);
        assert_eq!(x_end, 4);
        assert_eq!(y_start, 1);
        assert_eq!(y_end, 4);

        current_cell = Cell { x: 4, y: 4 };
        let (x_start, x_end) = gol.get_range_for_neighbourhood(current_cell.x, gol.width);
        let (y_start, y_end) = gol.get_range_for_neighbourhood(current_cell.y, gol.height);
        assert_eq!(x_start, 3);
        assert_eq!(x_end, 5);
        assert_eq!(y_start, 3);
        assert_eq!(y_end, 5);
    }

    #[test]
    fn range_of_neighbourhood_overflow_case() {
        let mut gol = GameOfLife::new(usize::MAX, usize::MAX);
        insert_3_cell_line_patter_top_left_corner(&mut gol);
        insert_square_patter_top_left_corner(&mut gol);

        let current_cell = Cell {
            x: usize::MAX - 1,
            y: usize::MAX - 1,
        }; // This test covers case of overflow
        let (x_start, x_end) = gol.get_range_for_neighbourhood(current_cell.x, gol.width);
        let (y_start, y_end) = gol.get_range_for_neighbourhood(current_cell.y, gol.height);
        assert_eq!(x_start, usize::MAX - 2);
        assert_eq!(x_end, usize::MAX);
        assert_eq!(y_start, usize::MAX - 2);
        assert_eq!(y_end, usize::MAX);
    }

    #[test]
    /// The board for this test has a default pattern inserted. Representing the dead state
    /// with D and the alive state with A the board would look like this:
    /// D A D D D
    /// D A D D D
    /// D A D D D
    /// D D D D D
    /// D D D D D
    /// So the living cells are (0,1), (1,1), (2,1)
    fn process_cell_for_5x5_board() {
        let mut gol = GameOfLife::new(5, 5);
        insert_3_cell_line_patter_top_left_corner(&mut gol);

        let current_cell = Cell { x: 0, y: 0 }; // Dead cell only has 2 living neighbours => stays dead.
        assert_eq!(gol.count_cell_living_neighbours(&current_cell),2);
        assert!(!gol.cell_next_state_is_alive(&current_cell));

        let current_cell = Cell { x: 0, y: 1 }; // Alive cell only has 1 living neighbours => cell dies.
        assert_eq!(gol.count_cell_living_neighbours(&current_cell),1);
        assert!(!gol.cell_next_state_is_alive(&current_cell));

        let current_cell = Cell { x: 1, y: 1 }; // Alive cell has 2 living neighbours => stays alive.
        assert_eq!(gol.count_cell_living_neighbours(&current_cell),2);
        assert!(gol.cell_next_state_is_alive(&current_cell));

        let current_cell = Cell { x: 1, y: 0 }; // Dead cell has 3 living neighbours => cell comes to life.
        assert_eq!(gol.count_cell_living_neighbours(&current_cell),3);
        assert!(gol.cell_next_state_is_alive(&current_cell));
    }

    #[test]
    /// The board for this test has the usual cross pattern as the previous boards and also has a square pattern at the
    /// bottom right corner, this pattern is constant.
    fn process_cell_for_overflow_case() {
        let mut gol = GameOfLife::new(usize::MAX, usize::MAX);
        insert_square_patter_top_left_corner(&mut gol);

        let current_cell = Cell {
            x: usize::MAX - 1,
            y: usize::MAX - 1,
        }; // Alive cell has 3 living neighbours => stays alive.
        assert_eq!(gol.count_cell_living_neighbours(&current_cell),3);
        assert!(gol.cell_next_state_is_alive(&current_cell));

        let current_cell = Cell {
            x: usize::MAX - 2,
            y: usize::MAX - 1,
        }; // Alive cell only has 3 living neighbours => stays alive.
        assert_eq!(gol.count_cell_living_neighbours(&current_cell),3);
        assert!(gol.cell_next_state_is_alive(&current_cell));

        let current_cell = Cell {
            x: usize::MAX - 1,
            y: usize::MAX - 2,
        }; // Alive cell has 3 living neighbours => stays alive.
        assert_eq!(gol.count_cell_living_neighbours(&current_cell),3);
        assert!(gol.cell_next_state_is_alive(&current_cell));

        let current_cell = Cell {
            x: usize::MAX - 2,
            y: usize::MAX - 2,
        }; // Alive cell has 3 living neighbours => stays alive.
        assert_eq!(gol.count_cell_living_neighbours(&current_cell),3);
        assert!(gol.cell_next_state_is_alive(&current_cell));
    }

    #[test]
    /// The board for this test has a default pattern inserted. Representing the dead state
    /// with D and the alive state with A the board would look like this:
    /// D A D D D
    /// D A D D D
    /// D A D D D
    /// D D D D D
    /// D D D D D
    /// After one transition it would look like this:
    /// D D D D D
    /// A A A D D
    /// D D D D D
    /// D D D D D
    /// D D D D D
    /// And then back again to the original disposition
    fn transition() {
        let mut gol = GameOfLife::new(5, 5);
        insert_3_cell_line_patter_top_left_corner(&mut gol);
        gol.transition();
        assert_eq!(gol.alive_cells.len(), 3);
        assert!(gol.alive_cells.contains(&Cell { x: 1, y: 0 }));
        assert!(gol.alive_cells.contains(&Cell { x: 1, y: 1 }));
        assert!(gol.alive_cells.contains(&Cell { x: 1, y: 2 }));
    }

    #[test]
    fn add_living_cell_in_and_out_of_bounds() {
        let mut gol = GameOfLife::new(10, 15);
        assert_eq!(gol.add_living_cell(Cell { x: 5, y: 14 }), Ok(()));
        assert!(gol.alive_cells.contains(&Cell { x: 5, y: 14 }));
        assert_eq!(
            gol.add_living_cell(Cell { x: 5, y: 15 }),
            Err(GolErrors::IndexOutOfBound)
        );
        assert!(!gol.alive_cells.contains(&Cell { x: 5, y: 15 }));
    }
}
