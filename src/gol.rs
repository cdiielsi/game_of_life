use std::{cmp::min, collections::HashSet};

/// Implementation of game of life. The struct has two field for the boards dimentions and anotherone to register
/// the living cells. After each transition this set is updated with the resulting living cells.
pub struct GameOfLife {
    width: usize,
    height: usize,
    alive_cells: HashSet<(usize, usize)>,
}

impl GameOfLife {
    /// Creates an instance of a Game of Life having the following pattern on the top left corner of the board
    /// if the size is equal or bigger than 3x3:
    /// D A D D
    /// D A D D
    /// D A D D
    /// if the size of the board is usize::MAX x usize::MAX the bottom right corner of the board has a square pattern
    /// as well, this is for testing purposes.
    /// If size is smaller than 3x3 the board created has no living cells.
    pub fn new(width: usize, height: usize) -> Self {
        let mut alive_cells = HashSet::new();
        if width > 2 && height > 2 {
            alive_cells.insert((0, 1));
            alive_cells.insert((1, 1));
            alive_cells.insert((2, 1));
            if width == usize::MAX && height == usize::MAX{
                alive_cells.insert((usize::MAX-1, usize::MAX-1));
                alive_cells.insert((usize::MAX-2, usize::MAX-1));
                alive_cells.insert((usize::MAX-2, usize::MAX-2));
                alive_cells.insert((usize::MAX-1, usize::MAX-2));
            }
        }
        Self {
            width,
            height,
            alive_cells,
        }
    }

    /// Updates alive_cell field with the new living cells.
    pub fn transition(&mut self) -> Option<()> {
        let mut next_iteration_set = HashSet::new();
        for x in 0..self.width {
            for y in 0..self.height {
                let current_cell = (x, y);
                self.process_cell(
                    &mut next_iteration_set,
                    current_cell,
                    self.alive_cells.contains(&(x, y)),
                );
            }
        }
        self.alive_cells = next_iteration_set;
        Some(())
    }

    /// Adds the current cell to the new_iteration_set according the following rules:
    /// - Any live cell with fewer than two live neighbours dies, as if by underpopulation.
    /// - Any live cell with two or three live neighbours lives on to the next generation.
    /// - Any live cell with more than three live neighbours dies, as if by overpopulation.
    /// - Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
    fn process_cell(
        &self,
        next_iteration_set: &mut HashSet<(usize, usize)>,
        current_cell: (usize, usize),
        current_cell_alive: bool,
    ) {
        let (x, y) = current_cell;
        let mut neighbours = 0;
        let (x_start, x_end) = self.get_range_for_neighbourhood(current_cell.0, self.width);
        let (y_start, y_end) = self.get_range_for_neighbourhood(current_cell.1, self.height);
        for i in x_start..x_end {
            for j in y_start..y_end {
                if self.alive_cells.contains(&(i, j)) && (i, j) != (x, y) {
                    neighbours += 1;
                }
            }
        }
        if neighbours == 3 || current_cell_alive && neighbours == 2 {
            next_iteration_set.insert(current_cell);
        }
    }

    /// Provides the indexes that determine a cell's neighbourhood.
    fn get_range_for_neighbourhood(&self, cell_component: usize, limit: usize) -> (usize, usize) {
        let start_range = match (cell_component).overflowing_sub(1) {
            (n, false) => n,
            (_, true) => cell_component,
        };
        let end_range = match (cell_component).overflowing_add(2) {
            (n, false) => min(n,limit),
            (_, true) => limit,
        };
        // cell_component is supposed to work as an index either in the range 0..width or the range 0..height,
        // in that sense start_range should be an inclusive bound and end_range should be an exclusive bound.
        // If cell_component + 2 overflows, it means that limit = usize::MAX and cell_component = usize::MAX - 1,
        // so end_range should be limit.

        (start_range, end_range)
    }

    /// Prints the board on console.
    /// Green square => alive cell.
    /// Black square => dead cell.
    pub fn print(&self) {
        for i in 0..self.width {
            for j in 0..self.height {
                if self.alive_cells.contains(&(i, j)) {
                    print!("🟩");
                } else {
                    print!("⬛");
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn range_of_neighbourhood_for_5x5_board() {
        let gol = GameOfLife::new(5, 5);

        let mut current_cell = (0, 0); // This test covers case of underflow
        let (x_star, x_end) = gol.get_range_for_neighbourhood(current_cell.0, gol.width);
        let (y_star, y_end) = gol.get_range_for_neighbourhood(current_cell.1, gol.height);
        assert_eq!(x_star, 0);
        assert_eq!(x_end, 2);
        assert_eq!(y_star, 0);
        assert_eq!(y_end, 2);

        current_cell = (2, 2);
        let (x_start, x_end) = gol.get_range_for_neighbourhood(current_cell.0, gol.width);
        let (y_start, y_end) = gol.get_range_for_neighbourhood(current_cell.1, gol.height);
        assert_eq!(x_start, 1);
        assert_eq!(x_end, 4);
        assert_eq!(y_start, 1);
        assert_eq!(y_end, 4);

        current_cell = (4, 4);
        let (x_start, x_end) = gol.get_range_for_neighbourhood(current_cell.0, gol.width);
        let (y_start, y_end) = gol.get_range_for_neighbourhood(current_cell.1, gol.height);
        assert_eq!(x_start, 3);
        assert_eq!(x_end, 5);
        assert_eq!(y_start, 3);
        assert_eq!(y_end, 5);
    }

    #[test]
    fn range_of_neighbourhood_overflow_case() {
        let gol = GameOfLife::new(usize::MAX, usize::MAX);

        let current_cell = (usize::MAX-1, usize::MAX-1); // This test covers case of overflow
        let (x_start, x_end) = gol.get_range_for_neighbourhood(current_cell.0, gol.width);
        let (y_start, y_end) = gol.get_range_for_neighbourhood(current_cell.1, gol.height);
        assert_eq!(x_start, usize::MAX-2);
        assert_eq!(x_end, usize::MAX);
        assert_eq!(y_start, usize::MAX-2);
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
        let gol = GameOfLife::new(5, 5);
        let mut new_alive_cells = HashSet::new();

        let current_cell = (0, 0); // Dead cell only has 2 living neighbours => stays dead.
        let current_cell_alive = false;
        gol.process_cell(&mut new_alive_cells, current_cell, current_cell_alive);
        assert!(new_alive_cells.is_empty());

        let current_cell = (0, 1); // Alive cell only has 1 living neighbours => cell dies.
        let current_cell_alive = true;
        gol.process_cell(&mut new_alive_cells, current_cell, current_cell_alive);
        assert!(new_alive_cells.is_empty());

        let current_cell = (1, 1); // Alive cell has 2 living neighbours => stays alive.
        let current_cell_alive = true;
        gol.process_cell(&mut new_alive_cells, current_cell, current_cell_alive);
        assert_eq!(new_alive_cells.len(), 1);
        assert!(new_alive_cells.contains(&current_cell));

        let current_cell = (1, 0); // Dead cell has 3 living neighbours => cell comes to life.
        let current_cell_alive = false;
        gol.process_cell(&mut new_alive_cells, current_cell, current_cell_alive);
        assert_eq!(new_alive_cells.len(), 2);
        assert!(new_alive_cells.contains(&current_cell));
    }

    #[test]
    /// The board for this test has the usual cross pattern as the previous boards and also has a square pattern at the
    /// bottom right corner, this pattern is constant.
    fn process_cell_for_overflow_case() {
        let gol = GameOfLife::new(usize::MAX, usize::MAX);
        let mut new_alive_cells = HashSet::new();

        let current_cell = (usize::MAX-1, usize::MAX-1); // Alive cell has 3 living neighbours => stays alive.
        let current_cell_alive = false;
        gol.process_cell(&mut new_alive_cells, current_cell, current_cell_alive);
        assert_eq!(new_alive_cells.len(), 1);
        assert!(new_alive_cells.contains(&current_cell));

        let current_cell = (usize::MAX-2, usize::MAX-1); // Alive cell only has 3 living neighbours => stays alive.
        let current_cell_alive = true;
        gol.process_cell(&mut new_alive_cells, current_cell, current_cell_alive);
        assert_eq!(new_alive_cells.len(), 2);
        assert!(new_alive_cells.contains(&current_cell));

        let current_cell = (usize::MAX-1, usize::MAX-2); // Alive cell has 3 living neighbours => stays alive.
        let current_cell_alive = true;
        gol.process_cell(&mut new_alive_cells, current_cell, current_cell_alive);
        assert_eq!(new_alive_cells.len(), 3);
        assert!(new_alive_cells.contains(&current_cell));

        let current_cell = (usize::MAX-2, usize::MAX-2); // Alive cell has 3 living neighbours => stays alive.
        let current_cell_alive = false;
        gol.process_cell(&mut new_alive_cells, current_cell, current_cell_alive);
        assert_eq!(new_alive_cells.len(), 4);
        assert!(new_alive_cells.contains(&current_cell));
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
        gol.transition();
        assert_eq!(gol.alive_cells.len(), 3);
        assert!(gol.alive_cells.contains(&(1, 0)));
        assert!(gol.alive_cells.contains(&(1, 1)));
        assert!(gol.alive_cells.contains(&(1, 2)));
    }
}
