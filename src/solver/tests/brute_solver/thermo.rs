use crate::{solver::brute::BruteSolver, types::{CellPosition, FixedNumber, Grid, SudokuConstraints, Thermo}};
use ntest::timeout;

fn thermo_9x9_no_solution() -> SudokuConstraints {
  SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![FixedNumber::new(8, 1, 9)]
    )
    .with_thermos(
      vec![
        Thermo(vec![
          CellPosition { row: 0, col: 0 },
          CellPosition { row: 1, col: 0 },
          CellPosition { row: 2, col: 0 },
          CellPosition { row: 3, col: 0 },
          CellPosition { row: 4, col: 0 },
          CellPosition { row: 5, col: 0 },
          CellPosition { row: 6, col: 0 },
          CellPosition { row: 7, col: 0 },
          CellPosition { row: 8, col: 0 },
        ]),
      ]
    )
}

// UK Sudoku Championship 2022 booklet - 9x9 thermo https://ukpuzzles.org/file_download.php?fileid=247&md5=c200e06d8822177932d906103919ceba
fn thermo_9x9_unique_solution() -> (SudokuConstraints, Grid) {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(2, 2, 2),
        FixedNumber::new(2, 6, 4),
        FixedNumber::new(3, 4, 5),
        FixedNumber::new(5, 4, 1),
        FixedNumber::new(6, 2, 9),
        FixedNumber::new(6, 6, 5),
      ]
    )
    .with_thermos(
      vec![
        Thermo(vec![
          CellPosition { row: 0, col: 6 },
          CellPosition { row: 0, col: 5 },
          CellPosition { row: 0, col: 4 },
          CellPosition { row: 0, col: 3 },
          CellPosition { row: 0, col: 2 },
          CellPosition { row: 0, col: 1 },
          CellPosition { row: 0, col: 0 },
        ]),
        Thermo(vec![
          CellPosition { row: 2, col: 0 },
          CellPosition { row: 3, col: 0 },
          CellPosition { row: 4, col: 0 },
          CellPosition { row: 5, col: 0 },
          CellPosition { row: 6, col: 0 },
          CellPosition { row: 7, col: 0 },
          CellPosition { row: 8, col: 0 },
        ]),
        Thermo(vec![
          CellPosition { row: 2, col: 5 },
          CellPosition { row: 2, col: 4 },
          CellPosition { row: 2, col: 3 },
        ]),
        Thermo(vec![
          CellPosition { row: 3, col: 2 },
          CellPosition { row: 4, col: 2 },
          CellPosition { row: 5, col: 2 },
        ]),
        Thermo(vec![
          CellPosition { row: 5, col: 6 },
          CellPosition { row: 4, col: 6 },
          CellPosition { row: 3, col: 6 },
        ]),
        Thermo(vec![
          CellPosition { row: 6, col: 3 },
          CellPosition { row: 6, col: 4 },
          CellPosition { row: 6, col: 5 },
        ]),
        Thermo(vec![
          CellPosition { row: 6, col: 8 },
          CellPosition { row: 5, col: 8 },
          CellPosition { row: 4, col: 8 },
          CellPosition { row: 3, col: 8 },
          CellPosition { row: 2, col: 8 },
          CellPosition { row: 1, col: 8 },
          CellPosition { row: 0, col: 8 },
        ]),
        Thermo(vec![
          CellPosition { row: 8, col: 2 },
          CellPosition { row: 8, col: 3 },
          CellPosition { row: 8, col: 4 },
          CellPosition { row: 8, col: 5 },
          CellPosition { row: 8, col: 6 },
          CellPosition { row: 8, col: 7 },
          CellPosition { row: 8, col: 8 },
        ]),
      ]
    );
  
  let solution = Grid::from_string("
    986542137
    354781296
    172963485
    263857914
    417629853
    598314762
    639478521
    725196348
    841235679
  ");

  (constraints, solution)
}

fn thermo_6x6_unique_solution() -> (SudokuConstraints, Grid) {
  // WSC booklet 6x6 thermo https://uploads-ssl.webflow.com/62793457876c001d28edf162/6348945a45b06acb414391b7_WSC_2022_IB_v2.1.pdf
  let constraints = SudokuConstraints::new(6)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(1, 0, 4),
        FixedNumber::new(2, 0, 5),
        FixedNumber::new(4, 5, 2),
        FixedNumber::new(5, 4, 4),
        FixedNumber::new(5, 5, 3),
      ]
    )
    .with_thermos(
      vec![
        Thermo(vec![
          CellPosition { row: 0, col: 0 },
          CellPosition { row: 0, col: 1 },
          CellPosition { row: 0, col: 2 },
          CellPosition { row: 0, col: 3 },
          CellPosition { row: 0, col: 4 },
          CellPosition { row: 0, col: 5 },
        ]),
        Thermo(vec![
          CellPosition { row: 1, col: 4 },
          CellPosition { row: 2, col: 4 },
          CellPosition { row: 3, col: 4 },
        ]),
        Thermo(vec![
          CellPosition { row: 2, col: 2 },
          CellPosition { row: 3, col: 2 },
          CellPosition { row: 4, col: 2 },
          CellPosition { row: 4, col: 3 },
        ]),
        Thermo(vec![
          CellPosition { row: 3, col: 0 },
          CellPosition { row: 4, col: 0 },
          CellPosition { row: 5, col: 0 },
        ]),
        Thermo(vec![
          CellPosition { row: 3, col: 3 },
          CellPosition { row: 2, col: 3 },
          CellPosition { row: 1, col: 3 },
          CellPosition { row: 1, col: 2 },
        ]),
      ]
    );

  let solution = Grid(vec![
    vec![ 1, 2, 3, 4, 5, 6 ],
    vec![ 4, 5, 6, 3, 2, 1 ],
    vec![ 5, 6, 1, 2, 3, 4 ],
    vec![ 2, 3, 4, 1, 6, 5 ],
    vec![ 3, 4, 5, 6, 1, 2 ],
    vec![ 6, 1, 2, 5, 4, 3 ],
  ]);

  (constraints, solution)
}

#[test]
#[timeout(500)]
fn brute_solver_without_logic_solves_thermo_6x6() {
  let (constraints, solution) = thermo_6x6_unique_solution();
  let mut solver = BruteSolver::new(constraints);
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

#[test]
#[timeout(500)]
fn brute_solver_with_logic_solves_thermo_6x6() {
  let (constraints, solution) = thermo_6x6_unique_solution();
  let mut solver = BruteSolver::new(constraints).with_logic();
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

// Currently at 20s (debug)
#[test]
#[ignore="solver needs more optimizations"]
#[timeout(10_000)]
fn brute_solver_without_logic_solves_thermo_9x9() {
  let (constraints, solution) = thermo_9x9_unique_solution();
  let mut solver = BruteSolver::new(constraints);
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

#[test]
#[timeout(1000)]
fn brute_solver_with_logic_solves_thermo_9x9() {
  let (constraints, solution) = thermo_9x9_unique_solution();
  let mut solver = BruteSolver::new(constraints).with_logic();
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

#[test]
#[ignore="solver needs more optimizations"]
#[timeout(10_000)]
fn brute_solver_without_logic_finds_no_solution_in_thermo_9x9() {
  let mut solver = BruteSolver::new(thermo_9x9_no_solution());
  let result = solver.solve();
  assert_eq!(result.solution_count, 0);
  assert_eq!(result.solution, None);
}

#[test]
#[timeout(500)]
fn brute_solver_with_logic_finds_no_solution_in_thermo_9x9() {
  let mut solver = BruteSolver::new(thermo_9x9_no_solution()).with_logic();
  let result = solver.solve();
  assert_eq!(result.solution_count, 0);
  assert_eq!(result.solution, None);
}
