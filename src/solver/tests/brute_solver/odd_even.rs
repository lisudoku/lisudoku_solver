use crate::{solver::brute::BruteSolver, types::{CellPosition, FixedNumber, Grid, SudokuConstraints}};
use ntest::timeout;

// https://gp.worldpuzzle.org/sites/default/files/Puzzles/2023/2023_SudokuRound1.pdf
fn odd_even_9x9_unique_solution() -> (SudokuConstraints, Grid) {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 0, 5),
        FixedNumber::new(1, 2, 3),
        FixedNumber::new(2, 1, 1),
        FixedNumber::new(2, 3, 8),
        FixedNumber::new(3, 2, 2),
        FixedNumber::new(3, 4, 9),
        FixedNumber::new(4, 3, 3),
        FixedNumber::new(4, 5, 1),
        FixedNumber::new(5, 4, 4),
        FixedNumber::new(5, 6, 2),
        FixedNumber::new(6, 5, 2),
        FixedNumber::new(6, 7, 3),
        FixedNumber::new(7, 6, 7),
        FixedNumber::new(8, 8, 4),
      ]
    )
    .with_even_cells(
      vec![
        CellPosition::new(0, 5), CellPosition::new(1, 6), CellPosition::new(2, 7),
        CellPosition::new(3, 8), CellPosition::new(3, 0), CellPosition::new(4, 1),
        CellPosition::new(5, 2), CellPosition::new(6, 3), CellPosition::new(7, 4),
        CellPosition::new(8, 5), CellPosition::new(7, 0), CellPosition::new(8, 1),
      ]
    )
    .with_odd_cells(
      vec![
        CellPosition::new(0, 3), CellPosition::new(1, 4), CellPosition::new(2, 5),
        CellPosition::new(3, 6), CellPosition::new(4, 7), CellPosition::new(5, 8),
        CellPosition::new(0, 7), CellPosition::new(1, 8), CellPosition::new(5, 0),
        CellPosition::new(6, 1), CellPosition::new(7, 2), CellPosition::new(8, 3),
      ]
    );
  let solution = Grid(vec![
    vec![ 5, 9, 8, 7, 6, 4, 3, 1, 2 ],
    vec![ 7, 6, 3, 2, 1, 5, 8, 4, 9 ],
    vec![ 2, 1, 4, 8, 3, 9, 5, 6, 7 ],
    vec![ 4, 3, 2, 6, 9, 7, 1, 5, 8 ],
    vec![ 9, 8, 5, 3, 2, 1, 4, 7, 6 ],
    vec![ 1, 7, 6, 5, 4, 8, 2, 9, 3 ],
    vec![ 8, 5, 9, 4, 7, 2, 6, 3, 1 ],
    vec![ 6, 4, 1, 9, 8, 3, 7, 2, 5 ],
    vec![ 3, 2, 7, 1, 5, 6, 9, 8, 4 ],
  ]);

  (constraints, solution)
}

#[test]
#[timeout(500)]
fn brute_solver_without_logic_finds_unique_solution_in_odd_even_9x9() {
  let (constraints, solution) = odd_even_9x9_unique_solution();
  let mut solver = BruteSolver::new(constraints);
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

#[test]
#[timeout(500)]
fn brute_solver_with_logic_finds_unique_solution_in_odd_even_9x9() {
  let (constraints, solution) = odd_even_9x9_unique_solution();
  let mut solver = BruteSolver::new(constraints).with_logic();
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}
