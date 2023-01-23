# lisudoku_solver

Used by https://lisudoku.xyz to solve sudoku variant puzzles.

Supported variants
1. Classic
2. Killer
3. Thermo
4. Kropki
5. Diagonal
6. Anti Knight
7. Irregular
8. Any combination of the above.

## Contribute

Join the [discord server](https://discord.gg/SGV8TQVSeT).

## Build

`cargo run-script build`

## Running tests

All tests `cargo test -- --nocapture`

Individual test `cargo test check_6x6_solve -- --nocapture`
