# Game of Life
This is an implementation of Conway's Game of Life in rust. 

## What is the Game of Life?
Conway's Game of Life is a zero-player game devised by the British mathematician John Horton Conway in 1970. The game follows the transition of 2-dimentional celular automaton throughout different stages according a set of rules across a grid of square cells. The automaton being represented by a group of cells marked as "alive". 

One interacts with the Game of Life only by creating an initial configuration and observing how it evolves. This evolution depends on the neighbourhood of each single cell (meaning its surrounding cells), and how many of this neighbours are "dead" or "alive". More specifically, the evolution goes by the following rules:
- Any live cell with fewer than two live neighbours dies, as if by underpopulation.
- Any live cell with two or three live neighbours lives on to the next generation.
- Any live cell with more than three live neighbours dies, as if by overpopulation.
- Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

## Quick Start

- To run the project, run:
```
make
```
- To build the project, run:
```
make build
```
- To run the tests, run:
```
make test
```

When executing the game you should see a 50x50 grid with three different known patterns evolving (blinker, block and glider). You can pause the transitions hitting ```space``` and resume hitting ```space``` again. When the game is paused, you can **add** or **remove** living cells from the board doing ```left click``` on a cell.

## Motivation behind Game of Life
Cellular automata have found application in various areas, including physics, theoretical biology and microstructure modeling. Conway's initial goal when devising the game, was to define an interesting and unpredictable cellular automaton, developing the game's rules so they would allow for patterns to "apparently" grow without limit, while keeping it difficult to prove that any given pattern would do so. 

As it turn out, this system achieves an impressive diversity of behavior, fluctuating between apparent randomness and order, and one of its most interesting features is the frequent occurrence of _gliders_, arrangements of cells that essentially move themselves across the grid. The thing is, it is possible to arrange the automaton so that this _gliders_ interact to perform computations, so all in all the Game of Life can emulate a universal Turing machine. To sum it all up, theoretically, anything that can be computed algorithmically can be computed within the Game of Life.

## About this project
This is an interactive implementation of the game of life where the user can modify an initial cell population and watch it evolve. The ```macroquad``` crate was used for the user interface. ```macroquad``` is a simple and easy way to use game library for the Rust programming language.

When it comes to the implementation's design, I decided to use a HashSet to model the board and the game, and for the cells I also implemented a Cell struct. The idea is that the set contains the current living cells and with each evolution step, the set gets updated with the new living cells. I chose the HashSet because I considered that given the game's rule of "dying because of overpopulation" the amount of living cells tends to be small compared to the board's size, so the idea is to save space by only keeping track of the living cells with the set, especially in the case that in a new iteration of the project I make the board's size variable.

## Dependencies
- rust 1.85.0
- macroquad 0.4.14

## References
- https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life
- https://en.wikipedia.org/wiki/Cellular_automaton
- https://macroquad.rs/docs/
- https://github.com/not-fl3/macroquad/tree/master
