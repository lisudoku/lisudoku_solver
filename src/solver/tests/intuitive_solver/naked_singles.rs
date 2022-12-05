use crate::{types::{SudokuConstraints, FixedNumber, CellPosition}, solver::Solver};

#[test]
fn check_naked_single_without_candidates() {
  let grid_size = 4;
  let fixed_numbers = vec![
    FixedNumber::new(1, 1, 4),
    FixedNumber::new(1, 3, 2),
    FixedNumber::new(2, 0, 1),
    FixedNumber::new(2, 2, 3),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);

  let step = solver.find_naked_singles();
  assert!(step.is_some());
  let mut step = step.unwrap();
  assert!(step.affected_cells.is_empty());
  let CellPosition { row, col } = step.cells[0];
  let rule_value = step.values[0];
  let initial_value = solver.grid[row][col];
  assert!(initial_value == 0);

  solver.apply_rule(&mut step);
  let final_value = solver.grid[row][col];
  assert!(final_value == rule_value);
}

#[test]
fn check_naked_single_with_candidates() {
  let grid_size = 4;
  let fixed_numbers = vec![
    FixedNumber::new(1, 1, 4),
    FixedNumber::new(1, 3, 2),
    FixedNumber::new(2, 0, 1),
    FixedNumber::new(2, 2, 3),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut solver.find_candidates_step().unwrap());

  let step = solver.find_naked_singles();
  assert!(step.is_some());
  let mut step = step.unwrap();
  let CellPosition { row, col } = step.cells[0];
  let rule_value = step.values[0];
  let initial_value = solver.grid[row][col];
  assert!(initial_value == 0);

  solver.apply_rule(&mut step);
  let final_value = solver.grid[row][col];
  assert!(final_value == rule_value);
  assert!(solver.candidates[row][col].is_empty());
}
