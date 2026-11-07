use crate::{solver::brute::BruteSolver, types::{FixedNumber, Grid, SudokuConstraints}};
use ntest::timeout;

fn anti_king_6x6_unique_solution() -> (SudokuConstraints, Grid) {
  let constraints = SudokuConstraints::new(6)
    .with_fixed_numbers(
      [
        FixedNumber::new(0, 0, 4), FixedNumber::new(1, 1, 3), FixedNumber::new(1, 4, 1),
        FixedNumber::new(3, 0, 6), FixedNumber::new(3, 1, 2),
        FixedNumber::new(2, 4, 6), FixedNumber::new(3, 4, 5),
        FixedNumber::new(5, 0, 3), FixedNumber::new(5, 5, 1),
      ]
    )
    .with_anti_king();
  let solution = Grid(vec![
    vec![ 4, 5, 1, 3, 2, 6 ],
    vec![ 2, 3, 6, 4, 1, 5 ],
    vec![ 1, 4, 5, 2, 6, 3 ],
    vec![ 6, 2, 3, 1, 5, 4 ],
    vec![ 5, 1, 4, 6, 3, 2 ],
    vec![ 3, 6, 2, 5, 4, 1 ],
  ]);
  (constraints, solution)
}

#[test]
#[timeout(500)]
fn brute_solver_without_logic_finds_unique_solution_in_anti_king_6x6() {
  let (constraints, solution) = anti_king_6x6_unique_solution();
  let mut solver = BruteSolver::new(constraints);
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

#[test]
#[timeout(500)]
fn brute_solver_with_logic_finds_unique_solution_in_anti_king_6x6() {
  let (constraints, solution) = anti_king_6x6_unique_solution();
  let mut solver = BruteSolver::new(constraints).with_logic();
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

fn anti_king_9x9_unique_solution() -> (SudokuConstraints, Grid) {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers_grid(
      &Grid::from_string("
        406070901
        080000000
        000000000
        003010020
        600402000
        100030007
        060000090
        090000060
        307060802
      ")
    )
    .with_anti_king();
  let solution = Grid::from_string("
    436275981
    589341276
    721896534
    953617428
    678452319
    142938657
    865724193
    294183765
    317569842
  ");
  (constraints, solution)
}

#[test]
#[timeout(500)]
fn brute_solver_without_logic_finds_unique_solution_in_anti_king_9x9() {
  let (constraints, solution) = anti_king_9x9_unique_solution();
  let mut solver = BruteSolver::new(constraints);
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

#[test]
#[timeout(1_000)]
fn brute_solver_with_logic_finds_unique_solution_in_anti_king_9x9() {
  let (constraints, solution) = anti_king_9x9_unique_solution();
  let mut solver = BruteSolver::new(constraints).with_logic();
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}
