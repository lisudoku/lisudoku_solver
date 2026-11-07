use crate::{solver::brute::BruteSolver, types::{CellPosition, FixedNumber, Grid, KropkiDot, SudokuConstraints}};
use ntest::timeout;

// 2023 Sudoku GP Round 1
fn kropki_9x9_unique_solution() -> (SudokuConstraints, Grid) {
  let constraints = SudokuConstraints::new(9)
    .with_kropki_dots(
      [
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
      ]
    )
    .with_kropki_negative();
  let solution = Grid(vec![
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

  (constraints, solution)
}

#[test]
#[ignore="solver needs more optimizations"]
#[timeout(10_000)]
fn brute_solver_without_logic_finds_unique_solution_in_kropki_9x9() {
  let (constraints, solution) = kropki_9x9_unique_solution();
  let mut solver = BruteSolver::new(constraints);
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

#[test]
#[timeout(1_000)]
fn brute_solver_with_logic_finds_unique_solution_in_kropki_9x9() {
  let (constraints, solution) = kropki_9x9_unique_solution();
  let mut solver = BruteSolver::new(constraints).with_logic();
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

fn kropki_4x4_unique_solution() -> (SudokuConstraints, Grid) {
  let constraints = SudokuConstraints::new(4)
    .with_fixed_numbers(
      [ FixedNumber::new(3, 0, 4) ]
    )
    .with_kropki_dots(
      [
        KropkiDot::consecutive(CellPosition::new(0, 0), CellPosition::new(0, 1)),
        KropkiDot::consecutive(CellPosition::new(0, 1), CellPosition::new(0, 2)),
        KropkiDot::consecutive(CellPosition::new(0, 2), CellPosition::new(0, 3)),
        KropkiDot::consecutive(CellPosition::new(2, 2), CellPosition::new(2, 3)),
        KropkiDot::consecutive(CellPosition::new(3, 1), CellPosition::new(3, 2)),
      ]
    );
  let solution = Grid(vec![
    vec![ 1, 2, 3, 4 ],
    vec![ 3, 4, 1, 2 ],
    vec![ 2, 1, 4, 3 ],
    vec![ 4, 3, 2, 1 ],
  ]);

  (constraints, solution)
}

#[test]
#[timeout(500)]
fn brute_solver_without_logic_finds_unique_solution_in_kropki_4x4() {
  let (constraints, solution) = kropki_4x4_unique_solution();
  let mut solver = BruteSolver::new(constraints);
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}
