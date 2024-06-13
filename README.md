# lisudoku_solver

Used by https://lisudoku.xyz to solve sudoku variant puzzles.

12 supported sudoku variants.

## Contribute

Join the [discord server](https://discord.gg/SGV8TQVSeT).

## Running tests

All tests `cargo test -- --nocapture`

Individual test `cargo test check_6x6_solve -- --nocapture`

## Build wasm

`cargo run-script build`

## Publish wasm

```
cd pkg
npm publish
```
