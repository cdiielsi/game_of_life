use gol::GameOfLife;

mod gol;

fn main() {
    let mut gol = GameOfLife::new(3,3);
    gol.print();
    gol.transition();
    gol.print()
}
