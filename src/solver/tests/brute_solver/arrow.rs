use crate::{solver::brute::BruteSolver, types::{Arrow, CellPosition, FixedNumber, Grid, SudokuConstraints}};
use ntest::timeout;

// https://gp.worldpuzzle.org/sites/default/files/Puzzles/2023/2023_SudokuRound7.pdf puzzle 8
// https://lisudoku.xyz/p/9q70tmwBmkpjvXIBck8j
fn arrow_9x9_unique_solution() -> (SudokuConstraints, Grid) {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      [ FixedNumber::new(0, 3, 1), FixedNumber::new(5, 8, 1) ]
    )
    .with_arrows(
      [
        Arrow {
          circle_cells: vec![ CellPosition::new(0, 0) ],
          arrow_cells: vec![
            CellPosition::new(1, 1), CellPosition::new(2, 2), CellPosition::new(3, 3),
            CellPosition::new(4, 4), CellPosition::new(5, 5),
          ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(1, 2) ],
          arrow_cells: vec![ CellPosition::new(2, 3) ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(1, 7) ],
          arrow_cells: vec![
            CellPosition::new(2, 8), CellPosition::new(2, 7), CellPosition::new(2, 6),
          ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(2, 4) ],
          arrow_cells: vec![ CellPosition::new(3, 4), CellPosition::new(4, 5) ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(3, 2), CellPosition::new(4, 2) ],
          arrow_cells: vec![
            CellPosition::new(4, 3), CellPosition::new(5, 3), CellPosition::new(5, 4),
            CellPosition::new(6, 5),
          ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(5, 1) ],
          arrow_cells: vec![ CellPosition::new(6, 2) ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(5, 6) ],
          arrow_cells: vec![ CellPosition::new(5, 7), CellPosition::new(4, 8) ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(6, 0) ],
          arrow_cells: vec![
            CellPosition::new(5, 0), CellPosition::new(4, 0), CellPosition::new(3, 0),
          ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(7, 5) ],
          arrow_cells: vec![ CellPosition::new(6, 6) ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(7, 6) ],
          arrow_cells: vec![ CellPosition::new(6, 7), CellPosition::new(7, 8) ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(7, 7) ],
          arrow_cells: vec![
            CellPosition::new(8, 8), CellPosition::new(8, 7), CellPosition::new(8, 6),
          ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(8, 2) ],
          arrow_cells: vec![
            CellPosition::new(8, 3), CellPosition::new(7, 2), CellPosition::new(7, 1),
            CellPosition::new(7, 0),
          ],
        },
      ]
    );
  let grid = Grid(vec![
    vec![ 9, 4, 3, 1, 2, 7, 6, 5, 8 ],
    vec![ 5, 2, 8, 4, 6, 3, 1, 9, 7 ],
    vec![ 7, 6, 1, 8, 9, 5, 2, 4, 3 ],
    vec![ 1, 8, 2, 3, 5, 6, 4, 7, 9 ],
    vec![ 3, 9, 5, 7, 1, 4, 8, 6, 2 ],
    vec![ 4, 7, 6, 9, 8, 2, 5, 3, 1 ],
    vec![ 8, 3, 7, 5, 4, 1, 9, 2, 6 ],
    vec![ 2, 1, 4, 6, 3, 9, 7, 8, 5 ],
    vec![ 6, 5, 9, 2, 7, 8, 3, 1, 4 ],
  ]);
  
  (constraints, grid)
}

#[test]
#[ignore="solver needs more optimizations"]
#[timeout(10_000)]
fn brute_solver_without_logic_solves_arrow_9x9() {
  let (constraints, solution) = arrow_9x9_unique_solution();
  let mut solver = BruteSolver::new(constraints);
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}

#[test]
#[timeout(2_000)]
fn brute_solver_with_logic_solves_arrow_9x9() {
  let (constraints, solution) = arrow_9x9_unique_solution();
  let mut solver = BruteSolver::new(constraints).with_logic();
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}


fn arrow_4x4_unique_solution() -> (SudokuConstraints, Grid) {
  let constraints = SudokuConstraints::new(4)
    .with_fixed_numbers(
      [
        FixedNumber::new(0, 3, 3), FixedNumber::new(2, 0, 1),
      ]
    )
    .with_arrows(
      [
        Arrow {
          circle_cells: vec![ CellPosition::new(3, 0) ],
          arrow_cells: vec![
            CellPosition::new(2, 1), CellPosition::new(1, 2), CellPosition::new(0, 1),
          ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(3, 1) ],
          arrow_cells: vec![ CellPosition::new(3, 2), CellPosition::new(3, 3) ],
        },
      ]
    );
  let solution = Grid(vec![
    vec![ 2, 1, 4, 3 ],
    vec![ 3, 4, 1, 2 ],
    vec![ 1, 2, 3, 4 ],
    vec![ 4, 3, 2, 1 ],
  ]);
  (constraints, solution)
}

#[test]
#[timeout(500)]
fn brute_solver_without_logic_solves_arrow_4x4() {
  let (constraints, solution) = arrow_4x4_unique_solution();
  let mut solver = BruteSolver::new(constraints);
  let result = solver.solve();
  assert_eq!(result.solution_count, 1);
  assert_eq!(result.solution.unwrap(), solution);
}
