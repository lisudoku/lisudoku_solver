use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use tsify::Tsify;

mod constraints;
mod results;

pub use constraints::*;
pub use results::*;

// Note: Area enum variants order matters!
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Tsify)]
#[tsify(into_wasm_abi)]
pub enum Area {
  Row(usize),
  Column(usize),
  Region(usize),
  Cell(usize, usize),
  Thermo(usize),
  Arrow(usize),
  KillerCage(usize),
  KropkiDot(usize),
  PrimaryDiagonal,
  SecondaryDiagonal,
  Renban(usize),
  Palindrome(usize),
  Adhoc(Vec<CellPosition>),
  Grid,
}

impl Area {
  pub fn to_string(&self) -> String {
    match self {
      Area::Row(row) => format!("row {}", row + 1),
      Area::Column(col) => format!("column {}", col + 1),
      Area::Region(region) => format!("box {}", region + 1),
      Area::Grid | Area::Adhoc(_) | Area::Cell(_, _) |
        Area::Thermo(_) | Area::Arrow(_) |
        Area::KillerCage(_) | Area::KropkiDot(_) |
        Area::PrimaryDiagonal | Area::SecondaryDiagonal |
        Area::Renban(_) | Area::Palindrome(_) => unimplemented!(),
    }
  }

  /// Move `cell` orthogonally to current area
  pub fn translate_cell_to(&self, cell: &CellPosition) -> CellPosition {
    match self {
      &Area::Row(row) => CellPosition { row, col: cell.col },
      &Area::Column(col) => CellPosition { row: cell.row, col },
      _ => unimplemented!(),
    }
  }
}

// https://www.reddit.com/r/sudoku/comments/11kpwbt/fun_puzzle_link_in_comment/
#[test]
fn check_sudoku_constraints_import_string() {
  let fixed_numbers = Grid(vec![
    vec![ 2, 0, 3, 0, 0, 0, 1, 0, 7 ],
    vec![ 9, 1, 0, 0, 4, 0, 0, 2, 6 ],
    vec![ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
    vec![ 3, 0, 8, 0, 1, 0, 7, 0, 9 ],
    vec![ 1, 0, 0, 0, 0, 0, 0, 0, 2 ],
    vec![ 0, 0, 2, 0, 0, 0, 8, 0, 0 ],
    vec![ 0, 0, 4, 1, 6, 7, 2, 0, 0 ],
    vec![ 7, 0, 0, 0, 8, 0, 0, 0, 1 ],
    vec![ 8, 0, 0, 2, 0, 9, 0, 0, 3 ],
  ]).to_fixed_numbers();
  let constraints = SudokuConstraints::new(9).with_fixed_numbers(fixed_numbers);
  assert_eq!(constraints.to_import_string(), String::from("\
    203000107\
    910040026\
    000000000\
    308010709\
    100000002\
    002000800\
    004167200\
    700080001\
    800209003\
  "));
}

#[test]
fn check_sudoku_grid_from_string() {
  let grid = Grid::from_string(String::from("\
    203000107\
    910040026\
    000000000\
    308010709\
    100000002\
    002000800\
    004167200\
    700080001\
    800209003\
  "));
  let expected_grid = Grid(vec![
    vec![ 2, 0, 3, 0, 0, 0, 1, 0, 7 ],
    vec![ 9, 1, 0, 0, 4, 0, 0, 2, 6 ],
    vec![ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
    vec![ 3, 0, 8, 0, 1, 0, 7, 0, 9 ],
    vec![ 1, 0, 0, 0, 0, 0, 0, 0, 2 ],
    vec![ 0, 0, 2, 0, 0, 0, 8, 0, 0 ],
    vec![ 0, 0, 4, 1, 6, 7, 2, 0, 0 ],
    vec![ 7, 0, 0, 0, 8, 0, 0, 0, 1 ],
    vec![ 8, 0, 0, 2, 0, 9, 0, 0, 3 ],
  ]);
  assert_eq!(grid, expected_grid);
}
