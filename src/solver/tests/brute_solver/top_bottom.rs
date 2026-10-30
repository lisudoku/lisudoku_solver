use crate::{solver::brute::BruteSolver, types::{FixedNumber, Grid, SudokuConstraints}};
use ntest::timeout;

fn top_bottom_9x9_unique_solution() -> (SudokuConstraints, Grid) {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(1, 1, 9), FixedNumber::new(1, 5, 7), FixedNumber::new(1, 7, 6),
        FixedNumber::new(2, 0, 4), FixedNumber::new(3, 2, 2), FixedNumber::new(3, 3, 8),
        FixedNumber::new(4, 4, 7), FixedNumber::new(4, 6, 1), FixedNumber::new(5, 8, 3),
        FixedNumber::new(6, 0, 5), FixedNumber::new(6, 3, 1), FixedNumber::new(7, 1, 4),
        FixedNumber::new(7, 8, 1),
      ]
    )
    .with_top_bottom();
  let solution = Grid(vec![
    vec![ 6, 7, 5, 4, 8, 1, 9, 3, 2 ],
    vec![ 1, 9, 3, 5, 2, 7, 8, 6, 4 ],
    vec![ 4, 2, 8, 6, 9, 3, 7, 1, 5 ],
    vec![ 3, 5, 2, 8, 1, 6, 4, 9, 7 ],
    vec![ 9, 6, 4, 3, 7, 5, 1, 2, 8 ],
    vec![ 8, 1, 7, 9, 4, 2, 6, 5, 3 ],
    vec![ 5, 8, 9, 1, 3, 4, 2, 7, 6 ],
    vec![ 7, 4, 6, 2, 5, 9, 3, 8, 1 ],
    vec![ 2, 3, 1, 7, 6, 8, 5, 4, 9 ],
  ]);
  (constraints, solution)
}

#[test]
#[ignore="solver needs more optimizations"]
#[timeout(10_000)]
fn brute_solver_without_logic_finds_unique_solution_in_top_bottom_9x9() {
  let (constraints, solution) = top_bottom_9x9_unique_solution();
  let mut solver = BruteSolver::new(constraints);
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

#[test]
#[timeout(1_000)]
fn brute_solver_with_logic_finds_unique_solution_in_top_bottom_9x9() {
  let (constraints, solution) = top_bottom_9x9_unique_solution();
  let mut solver = BruteSolver::new(constraints).with_logic();
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

fn top_bottom_4x4_unique_solution() -> (SudokuConstraints, Grid) {
  let constraints = SudokuConstraints::new(4)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 0, 4), FixedNumber::new(0, 3, 1),
        FixedNumber::new(3, 0, 1), FixedNumber::new(3, 3, 4),
      ]
    )
    .with_top_bottom();
  let solution = Grid(vec![
    vec![ 4, 2, 3, 1 ],
    vec![ 3, 1, 4, 2 ],
    vec![ 2, 4, 1, 3 ],
    vec![ 1, 3, 2, 4 ],
  ]);
  (constraints, solution)
}

#[test]
#[timeout(500)]
fn brute_solver_without_logic_finds_unique_solution_in_top_bottom_4x4() {
  let (constraints, solution) = top_bottom_4x4_unique_solution();
  let mut solver = BruteSolver::new(constraints);
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}
