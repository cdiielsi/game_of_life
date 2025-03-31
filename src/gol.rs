pub struct GameOfLife {
    width: usize,
    height:usize,
    alive_cells: HashMap((usize,usize)),
}

impl GameOfLife {
    fn new(width:usize,height:usize) -> Self{
        let mut alive_cells = HashSet::new();
        if usize > 2 && height > 2 {
            alive_cells.insert((0,1));
            alive_cells.insert((1,1));
            live_cells.insert((2,1));
        }
        Self{
            width,
            height,
            alive_cells,
        }
    }
    
}