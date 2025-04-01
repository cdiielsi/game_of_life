use gol::GameOfLife;

mod gol;

fn main() {
    let mut gol = GameOfLife::new(3,3);
    gol.print();
    for _ in 0..10{
        gol.transition();
        gol.print();
    }
}
