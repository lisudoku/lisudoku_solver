use crate::{types::{SudokuConstraints, FixedNumber, CellPosition}, solver::Solver};

#[test]
fn check_hidden_single_in_region() {
  let grid_size = 4;
  let fixed_numbers = vec![
    FixedNumber::new(1, 3, 1),
    FixedNumber::new(3, 1, 1),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);

  let step = solver.find_hidden_singles();
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
fn check_hidden_single_on_row() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 3, 2),
    FixedNumber::new(0, 4, 3),
    FixedNumber::new(0, 5, 4),
    FixedNumber::new(2, 8, 1),
    FixedNumber::new(5, 2, 1),
    FixedNumber::new(6, 1, 1),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);

  let step = solver.find_hidden_singles();
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
fn check_hidden_single_on_col() {
  let grid_size = 6;
  let fixed_numbers = vec![
    FixedNumber::new(0, 5, 1),
    FixedNumber::new(2, 3, 2),
    FixedNumber::new(3, 3, 3),
    FixedNumber::new(4, 0, 1),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);

  let step = solver.find_hidden_singles();
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
fn check_hidden_single_in_region_with_candidates() {
  let grid_size = 4;
  let fixed_numbers = vec![
    FixedNumber::new(1, 3, 1),
    FixedNumber::new(3, 1, 1),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut solver.find_candidates_step().unwrap());

  let step = solver.find_hidden_singles();
  assert!(step.is_some());
  let mut step = step.unwrap();
  assert_eq!(step.cells.len(), 1);
  let CellPosition { row, col } = step.cells[0];
  let rule_value = step.values[0];
  let initial_value = solver.grid[row][col];
  assert!(initial_value == 0);

  solver.apply_rule(&mut step);
  let final_value = solver.grid[row][col];
  assert!(final_value == rule_value);
}
