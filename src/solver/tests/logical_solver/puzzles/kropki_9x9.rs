use ntest::timeout;

use crate::{types::{SudokuConstraints, CellPosition, SolutionType, KropkiDot, FixedNumber}, solver::Solver};

// https://gp.worldpuzzle.org/sites/default/files/Puzzles/2023/2023_SudokuRound1_IB.pdf
#[test]
#[timeout(5000)]
fn check_kropki_9x9_1_solve() {
  let grid_size = 9;
  let empty_cells = grid_size * grid_size;
  let mut constraints = SudokuConstraints::new(grid_size, vec![]);
  constraints.kropki_dots = vec![
    KropkiDot::consecutive(CellPosition::new(0, 2), CellPosition::new(0, 3)),
    KropkiDot::double(CellPosition::new(1, 0), CellPosition::new(2, 0)),
    KropkiDot::double(CellPosition::new(2, 1), CellPosition::new(3, 1)),
    KropkiDot::consecutive(CellPosition::new(0, 4), CellPosition::new(1, 4)),
    KropkiDot::consecutive(CellPosition::new(2, 3), CellPosition::new(2, 4)),
    KropkiDot::double(CellPosition::new(0, 3), CellPosition::new(0, 4)),
    KropkiDot::double(CellPosition::new(2, 3), CellPosition::new(3, 3)),
    KropkiDot::consecutive(CellPosition::new(0, 7), CellPosition::new(1, 7)),
    KropkiDot::consecutive(CellPosition::new(1, 8), CellPosition::new(2, 8)),
    KropkiDot::consecutive(CellPosition::new(2, 6), CellPosition::new(2, 7)),
    KropkiDot::consecutive(CellPosition::new(2, 6), CellPosition::new(3, 6)),
    KropkiDot::consecutive(CellPosition::new(4, 1), CellPosition::new(4, 2)),
    KropkiDot::consecutive(CellPosition::new(4, 2), CellPosition::new(4, 3)),
    KropkiDot::consecutive(CellPosition::new(5, 0), CellPosition::new(5, 1)),
    KropkiDot::double(CellPosition::new(3, 2), CellPosition::new(3, 3)),
    KropkiDot::double(CellPosition::new(5, 0), CellPosition::new(6, 0)),
    KropkiDot::consecutive(CellPosition::new(4, 3), CellPosition::new(5, 3)),
    KropkiDot::consecutive(CellPosition::new(5, 3), CellPosition::new(5, 4)),
    KropkiDot::consecutive(CellPosition::new(5, 5), CellPosition::new(5, 6)),
    KropkiDot::consecutive(CellPosition::new(5, 5), CellPosition::new(6, 5)),
    KropkiDot::double(CellPosition::new(4, 3), CellPosition::new(4, 4)),
    KropkiDot::double(CellPosition::new(5, 4), CellPosition::new(5, 5)),
    KropkiDot::consecutive(CellPosition::new(3, 7), CellPosition::new(4, 7)),
    KropkiDot::consecutive(CellPosition::new(5, 6), CellPosition::new(6, 6)),
    KropkiDot::double(CellPosition::new(3, 8), CellPosition::new(4, 8)),
    KropkiDot::double(CellPosition::new(4, 7), CellPosition::new(4, 8)),
    KropkiDot::double(CellPosition::new(4, 7), CellPosition::new(5, 7)),
    KropkiDot::consecutive(CellPosition::new(6, 0), CellPosition::new(6, 1)),
    KropkiDot::consecutive(CellPosition::new(7, 0), CellPosition::new(7, 1)),
    KropkiDot::consecutive(CellPosition::new(8, 1), CellPosition::new(8, 2)),
    KropkiDot::consecutive(CellPosition::new(6, 4), CellPosition::new(6, 5)),
    KropkiDot::consecutive(CellPosition::new(6, 5), CellPosition::new(6, 6)),
    KropkiDot::consecutive(CellPosition::new(7, 3), CellPosition::new(7, 4)),
    KropkiDot::consecutive(CellPosition::new(7, 5), CellPosition::new(8, 5)),
    KropkiDot::double(CellPosition::new(6, 4), CellPosition::new(7, 4)),
    KropkiDot::double(CellPosition::new(6, 5), CellPosition::new(7, 5)),
    KropkiDot::double(CellPosition::new(7, 5), CellPosition::new(7, 6)),
    KropkiDot::consecutive(CellPosition::new(6, 6), CellPosition::new(7, 6)),
    KropkiDot::consecutive(CellPosition::new(6, 7), CellPosition::new(7, 7)),
    KropkiDot::consecutive(CellPosition::new(7, 6), CellPosition::new(8, 6)),
    KropkiDot::consecutive(CellPosition::new(8, 7), CellPosition::new(8, 8)),
    KropkiDot::double(CellPosition::new(6, 6), CellPosition::new(6, 7)),
  ];
  constraints.kropki_negative = true;

  let mut solver = Solver::new(constraints, None);
  assert_eq!(solver.constraints.kropki_dots.len(), (grid_size - 1) * grid_size * 2);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(result.solution.unwrap(), vec![
    vec![ 7, 5, 2, 3, 6, 8, 1, 4, 9 ],
    vec![ 4, 9, 6, 1, 7, 2, 8, 5, 3 ],
    vec![ 8, 3, 1, 4, 5, 9, 6, 7, 2 ],
    vec![ 1, 6, 4, 2, 9, 5, 7, 3, 8 ],
    vec![ 5, 8, 7, 6, 3, 1, 9, 2, 4 ],
    vec![ 3, 2, 9, 7, 8, 4, 5, 1, 6 ],
    vec![ 6, 7, 5, 9, 2, 3, 4, 8, 1 ],
    vec![ 2, 1, 8, 5, 4, 6, 3, 9, 7 ],
    vec![ 9, 4, 3, 8, 1, 7, 2, 6, 5 ],
  ]);
  assert!(result.steps.len() >= empty_cells);
  insta::assert_yaml_snapshot!("check_kropki_9x9_1_solve", result.steps);
}

// https://ukpuzzles.org/file_download.php?fileid=247&md5=c200e06d8822177932d906103919ceba
#[test]
fn check_kropki_9x9_2_solve() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 3, 4),
    FixedNumber::new(0, 4, 3),
    FixedNumber::new(3, 4, 4),
    FixedNumber::new(3, 8, 1),
    FixedNumber::new(4, 0, 8),
    FixedNumber::new(4, 3, 6),
    FixedNumber::new(4, 5, 3),
    FixedNumber::new(4, 8, 4),
    FixedNumber::new(5, 0, 4),
    FixedNumber::new(5, 4, 5),
    FixedNumber::new(8, 4, 6),
    FixedNumber::new(8, 5, 5),
  ];
  let empty_cells = grid_size * grid_size - fixed_numbers.len();
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.kropki_dots = vec![
    KropkiDot::consecutive(CellPosition::new(1, 0), CellPosition::new(1, 1)),
    KropkiDot::consecutive(CellPosition::new(1, 1), CellPosition::new(1, 2)),
    KropkiDot::consecutive(CellPosition::new(1, 5), CellPosition::new(2, 5)),
    KropkiDot::consecutive(CellPosition::new(2, 5), CellPosition::new(3, 5)),
    KropkiDot::consecutive(CellPosition::new(0, 7), CellPosition::new(1, 7)),
    KropkiDot::consecutive(CellPosition::new(1, 7), CellPosition::new(2, 7)),
    KropkiDot::consecutive(CellPosition::new(3, 1), CellPosition::new(3, 2)),
    KropkiDot::consecutive(CellPosition::new(3, 2), CellPosition::new(3, 3)),
    KropkiDot::consecutive(CellPosition::new(5, 3), CellPosition::new(6, 3)),
    KropkiDot::consecutive(CellPosition::new(6, 3), CellPosition::new(7, 3)),
    KropkiDot::consecutive(CellPosition::new(5, 5), CellPosition::new(5, 6)),
    KropkiDot::consecutive(CellPosition::new(5, 6), CellPosition::new(5, 7)),
    KropkiDot::consecutive(CellPosition::new(6, 1), CellPosition::new(7, 1)),
    KropkiDot::consecutive(CellPosition::new(7, 1), CellPosition::new(8, 1)),
    KropkiDot::consecutive(CellPosition::new(7, 6), CellPosition::new(7, 7)),
    KropkiDot::consecutive(CellPosition::new(7, 7), CellPosition::new(7, 8)),
  ];

  let mut solver = Solver::new(constraints, None);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(result.solution.unwrap(), vec![
    vec![ 7, 6, 9, 4, 3, 2, 1, 5, 8 ],
    vec![ 1, 2, 3, 5, 8, 6, 9, 4, 7 ],
    vec![ 5, 4, 8, 9, 1, 7, 6, 3, 2 ],
    vec![ 9, 5, 6, 7, 4, 8, 3, 2, 1 ],
    vec![ 8, 1, 7, 6, 2, 3, 5, 9, 4 ],
    vec![ 4, 3, 2, 1, 5, 9, 8, 7, 6 ],
    vec![ 6, 9, 5, 2, 7, 1, 4, 8, 3 ],
    vec![ 2, 8, 1, 3, 9, 4, 7, 6, 5 ],
    vec![ 3, 7, 4, 8, 6, 5, 2, 1, 9 ],
  ]);
  assert!(result.steps.len() >= empty_cells);
  insta::assert_yaml_snapshot!(result.steps);
}

// 2023 Sudoku GP Round 1
#[test]
#[timeout(4000)]
fn check_kropki_9x9_3_solve() {
  let grid_size = 9;
  let empty_cells = grid_size * grid_size;
  let mut constraints = SudokuConstraints::new(grid_size, vec![]);
  constraints.kropki_dots = vec![
    KropkiDot::consecutive(CellPosition::new(1, 1), CellPosition::new(2, 1)),
    KropkiDot::double(CellPosition::new(2, 0), CellPosition::new(2, 1)),
    KropkiDot::double(CellPosition::new(2, 0), CellPosition::new(3, 0)),
    KropkiDot::consecutive(CellPosition::new(2, 3), CellPosition::new(3, 3)),
    KropkiDot::consecutive(CellPosition::new(2, 5), CellPosition::new(3, 5)),
    KropkiDot::double(CellPosition::new(1, 4), CellPosition::new(2, 4)),
    KropkiDot::consecutive(CellPosition::new(0, 7), CellPosition::new(1, 7)),
    KropkiDot::consecutive(CellPosition::new(1, 7), CellPosition::new(1, 8)),
    KropkiDot::consecutive(CellPosition::new(2, 7), CellPosition::new(3, 7)),
    KropkiDot::double(CellPosition::new(0, 6), CellPosition::new(0, 7)),
    KropkiDot::double(CellPosition::new(1, 6), CellPosition::new(2, 6)),
    KropkiDot::double(CellPosition::new(1, 8), CellPosition::new(2, 8)),
    KropkiDot::consecutive(CellPosition::new(3, 0), CellPosition::new(4, 0)),
    KropkiDot::consecutive(CellPosition::new(4, 1), CellPosition::new(5, 1)),
    KropkiDot::consecutive(CellPosition::new(5, 0), CellPosition::new(5, 1)),
    KropkiDot::consecutive(CellPosition::new(5, 1), CellPosition::new(6, 1)),
    KropkiDot::consecutive(CellPosition::new(5, 2), CellPosition::new(5, 3)),
    KropkiDot::double(CellPosition::new(3, 2), CellPosition::new(4, 2)),
    KropkiDot::double(CellPosition::new(5, 2), CellPosition::new(6, 2)),
    KropkiDot::consecutive(CellPosition::new(4, 3), CellPosition::new(5, 3)),
    KropkiDot::consecutive(CellPosition::new(4, 5), CellPosition::new(5, 5)),
    KropkiDot::consecutive(CellPosition::new(5, 5), CellPosition::new(5, 6)),
    KropkiDot::double(CellPosition::new(3, 4), CellPosition::new(4, 4)),
    KropkiDot::double(CellPosition::new(5, 4), CellPosition::new(6, 4)),
    KropkiDot::consecutive(CellPosition::new(3, 7), CellPosition::new(4, 7)),
    KropkiDot::consecutive(CellPosition::new(5, 7), CellPosition::new(5, 8)),
    KropkiDot::consecutive(CellPosition::new(5, 8), CellPosition::new(6, 8)),
    KropkiDot::double(CellPosition::new(3, 6), CellPosition::new(4, 6)),
    KropkiDot::consecutive(CellPosition::new(6, 1), CellPosition::new(7, 1)),
    KropkiDot::consecutive(CellPosition::new(7, 2), CellPosition::new(7, 3)),
    KropkiDot::consecutive(CellPosition::new(8, 1), CellPosition::new(8, 2)),
    KropkiDot::consecutive(CellPosition::new(8, 2), CellPosition::new(8, 3)),
    KropkiDot::double(CellPosition::new(6, 2), CellPosition::new(6, 3)),
    KropkiDot::double(CellPosition::new(7, 2), CellPosition::new(8, 2)),
    KropkiDot::consecutive(CellPosition::new(6, 5), CellPosition::new(6, 6)),
    KropkiDot::consecutive(CellPosition::new(8, 3), CellPosition::new(8, 4)),
    KropkiDot::consecutive(CellPosition::new(6, 7), CellPosition::new(7, 7)),
    KropkiDot::consecutive(CellPosition::new(8, 6), CellPosition::new(8, 7)),
    KropkiDot::double(CellPosition::new(7, 7), CellPosition::new(7, 8)),
    KropkiDot::double(CellPosition::new(7, 8), CellPosition::new(8, 8)),
  ];
  constraints.kropki_negative = true;

  let mut solver = Solver::new(constraints, None);
  assert_eq!(solver.constraints.kropki_dots.len(), (grid_size - 1) * grid_size * 2);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(result.solution.unwrap(), vec![
    vec![ 6, 1, 5, 3, 7, 2, 8, 4, 9 ],
    vec![ 8, 3, 7, 1, 4, 9, 2, 5, 6 ],
    vec![ 2, 4, 9, 5, 8, 6, 1, 7, 3 ],
    vec![ 4, 9, 1, 6, 2, 7, 3, 8, 5 ],
    vec![ 3, 5, 2, 8, 1, 4, 6, 9, 7 ],
    vec![ 7, 6, 8, 9, 3, 5, 4, 1, 2 ],
    vec![ 5, 7, 4, 2, 6, 8, 9, 3, 1 ],
    vec![ 1, 8, 6, 7, 9, 3, 5, 2, 4 ],
    vec![ 9, 2, 3, 4, 5, 1, 7, 6, 8 ],
  ]);
  assert!(result.steps.len() >= empty_cells);
  insta::assert_yaml_snapshot!("check_kropki_9x9_3_solve", result.steps);
}
