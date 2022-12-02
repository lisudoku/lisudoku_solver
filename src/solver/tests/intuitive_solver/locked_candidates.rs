use crate::{types::{SudokuConstraints, FixedNumber, CellPosition}, solver::Solver};

#[test]
fn check_pointing_pairs() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 0, 5),
    FixedNumber::new(0, 1, 1),
    FixedNumber::new(0, 2, 4),
    FixedNumber::new(0, 5, 3),
    FixedNumber::new(1, 3, 1),
    FixedNumber::new(1, 4, 6),
    FixedNumber::new(1, 5, 5),
    FixedNumber::new(1, 6, 3),
    FixedNumber::new(2, 0, 3),
    FixedNumber::new(2, 3, 9),
    FixedNumber::new(2, 4, 2),
    FixedNumber::new(2, 5, 4),
    FixedNumber::new(2, 6, 1),
    FixedNumber::new(2, 7, 5),
    FixedNumber::new(3, 1, 5),
    FixedNumber::new(3, 2, 1),
    FixedNumber::new(3, 4, 3),
    FixedNumber::new(3, 6, 4),
    FixedNumber::new(3, 7, 6),
    FixedNumber::new(4, 2, 3),
    FixedNumber::new(4, 3, 5),
    FixedNumber::new(4, 4, 4),
    FixedNumber::new(4, 8, 1),
    FixedNumber::new(5, 2, 2),
    FixedNumber::new(5, 3, 6),
    FixedNumber::new(5, 5, 1),
    FixedNumber::new(5, 6, 7),
    FixedNumber::new(5, 7, 3),
    FixedNumber::new(5, 8, 5),
    FixedNumber::new(6, 1, 7),
    FixedNumber::new(6, 6, 5),
    FixedNumber::new(6, 7, 1),
    FixedNumber::new(6, 8, 3),
    FixedNumber::new(7, 0, 1),
    FixedNumber::new(7, 1, 3),
    FixedNumber::new(7, 4, 5),
    FixedNumber::new(7, 7, 9),
    FixedNumber::new(8, 2, 5),
    FixedNumber::new(8, 3, 3),
    FixedNumber::new(8, 4, 1),
    FixedNumber::new(8, 5, 6),
    FixedNumber::new(8, 7, 7),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut solver.find_candidates_step().unwrap());
  let step = solver.find_locked_candidates();
  assert!(step.is_some());
  let mut step = step.unwrap();
  let CellPosition { row, col } = step.affected_cells[0];
  let initial_candidates = solver.candidates[row][col].clone();
  solver.apply_rule(&mut step);
  let final_candidates = &solver.candidates[row][col];
  assert!(final_candidates.len() < initial_candidates.len());
}
