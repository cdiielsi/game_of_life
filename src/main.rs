use gol::GameOfLife;
use std::{thread, time};

mod gol;

fn main() {
    let mut gol = GameOfLife::new(5, 5);
    gol.print();
    for i in 0..10 {
        thread::sleep(time::Duration::from_millis(1000));
        print!("{esc}c", esc = 27 as char);
        println!("Transicion: {i}");
        gol.transition();
        gol.print();
    }
}
