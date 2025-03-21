# lisudoku_solver

Used by https://lisudoku.xyz to solve sudoku variant puzzles.

14 supported sudoku variants.

## Contribute

Join the [discord server](https://discord.gg/SGV8TQVSeT).

## Running tests

All tests `cargo test -- --nocapture`

Individual test `cargo test check_6x6_solve -- --nocapture`

Manage test snapshots through `cargo insta review`

## Build wasm

`cargo run-script build`

## Publish wasm

```
cd pkg
npm publish
```
