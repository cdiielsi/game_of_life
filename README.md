# Game of Life
This is an implementation of Conway's Game of Life in rust. 

## What is the Game of Life?
Conway's Game of Life is a zero-player game devised by the British mathematician John Horton Conway in 1970. The game follows the transition of 2-dimentional celular automaton throughout different stages according a set of rules across a grid of square cells. The automaton being represented by a group of cells marked as "alive". 

One interacts with the Game of Life only by creating an initial configuration and observing how it evolves. This evolution depends on the neighbourhood of each single cell (meaning its surrounding cells), and how many of this neighbours are "dead" or "alive". More specifically, the evolution goes by the following rules:
- Any live cell with fewer than two live neighbours dies, as if by underpopulation.
- Any live cell with two or three live neighbours lives on to the next generation.
- Any live cell with more than three live neighbours dies, as if by overpopulation.
- Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.


## Motivation behind Game of Life
Cellular automata have found application in various areas, including physics, theoretical biology and microstructure modeling. Conway's initial goal when devising the game, was to define an interesting and unpredictable cellular automaton, developing the game's rules so they would allow for patterns to "apparently" grow without limit, while keeping it difficult to prove that any given pattern would do so. 

As it turn out, this system achieves an impressive diversity of behavior, fluctuating between apparent randomness and order, and one of its most interesting features is the frequent occurrence of _gliders_, arrangements of cells that essentially move themselves across the grid. The thing is, it is possible to arrange the automaton so that this _gliders_ interact to perform computations, so all in all the Game of Life can emulate a universal Turing machine. To sum it all up, theoretically, anything that can be computed algorithmically can be computed within the Game of Life.

## About this project
Work in progress... the idea is to have an interactive implementation of the game where the user can set up an initial cell population and watch it evolve.

## Dependencies
- rust 1.85.0

## References
- https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life
- https://en.wikipedia.org/wiki/Cellular_automaton
