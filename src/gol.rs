use std::collections::HashSet;

pub struct GameOfLife {
    width: usize,
    height:usize,
    alive_cells: HashSet<(usize,usize)>,
}

impl GameOfLife {
    pub fn new(width:usize,height:usize) -> Self{
        let mut alive_cells = HashSet::new();
        if width > 2 && height > 2 {
            alive_cells.insert((0,1));
            alive_cells.insert((1,1));
            alive_cells.insert((2,1));
        }
        Self{
            width,
            height,
            alive_cells,
        }
    }
    pub fn transition(&mut self)->Option<()>{
        let mut next_iteration_set = HashSet::new();
        for x in 0..self.width{
            for y in 0..self.height{
                let current_cell = (x,y);
                self.process_cell(&mut next_iteration_set, current_cell,self.alive_cells.contains(&(x,y)));
            }
        }
        self.alive_cells = next_iteration_set;
        Some(())
    }

    fn process_cell(&self,next_iteration_set:&mut HashSet<(usize,usize)>,current_cell:(usize,usize),current_cell_alive:bool){
        let (x,y) = current_cell;
        let mut neighbours = 0;
        let (x_star,x_end) = self.get_range_for_neighbourhood(current_cell.0);
        let (y_star,y_end) = self.get_range_for_neighbourhood(current_cell.1);
        for i in x_star..x_end{
            for j in y_star..y_end{
                if self.alive_cells.contains(&(i,j)) && (i,j)!=(x,y){
                    neighbours += 1;
                }
            }
        }
        if current_cell_alive && (neighbours == 2 || neighbours == 3){
            next_iteration_set.insert(current_cell);
        }else if !current_cell_alive && neighbours == 3{
            next_iteration_set.insert(current_cell);
        }
    }

    fn get_range_for_neighbourhood(&self,cell_component:usize)->(usize,usize){
        let start_range = match (cell_component).overflowing_sub(1) {
            (n,false) => n,
            (_,true) => cell_component,
        };
        let end_range = match (cell_component).overflowing_add(2) {
            (n,false) => n,
            (_,true) => cell_component,
        };
        (start_range,end_range)
    }

    pub fn print(&self){
        for i in 0..self.width{
            for j in 0..self.height{
                if self.alive_cells.contains(&(i,j)){
                    print!("1" );
                }else{
                    print!("_" );
                }
            }
            print!("\n");
        }
    }
}