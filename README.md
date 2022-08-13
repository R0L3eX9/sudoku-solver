# Sudoku Solver

## What is Sudoku?
Sudoku is an old game in which the main goal is to fill all the empty cells with numbers from 1-9 without them
repeating on the row, column, or inner square they are part of.

An inner square is a square of length 3, in a Sudoku puzzle there are 9 inner squares that form a bigger square of length 9.


## How the algorithm works?
The algorithm utilizes the backtracking technique which for every empty cell (denoted as 0) loops through all the values from 1-9.

At each step the algorithm checks weather or not that value is already present in the particular row, column or inner square, continuing with the possible values if the value is already present.

Over 49100 configurations have been tested, the algorithm averaging 0.0018142142847551513 seconds.

## Run locally
You are expected to have the following dependencies already installed:
* [Git](https://git-scm.com/downloads)
* [Rust](https://www.rust-lang.org/tools/install)

```
git clone git@github.com:R0L3eX9/sudoku-solver.git
cd sudoku-solver
cargo run
```

To change the puzzle configuration you can change the "board.txt" file.

Empty cells are denoted by a 0.

An example configuration has already been provided.
