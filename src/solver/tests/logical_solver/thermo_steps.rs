use crate::{solver::{Solver, logical_solver::{technique::Technique, thermo_steps::Thermo}}, types::{CellPosition, FixedNumber, SudokuConstraints}};

#[test]
fn check_thermo_steps() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(4, 3, 7),
    FixedNumber::new(6, 3, 3),
  ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.thermos = vec![
    crate::types::Thermo(vec![
      CellPosition::new(8, 0),
      CellPosition::new(7, 0),
      CellPosition::new(6, 0),
      CellPosition::new(5, 0),
      CellPosition::new(4, 0),
      CellPosition::new(3, 0),
      CellPosition::new(2, 0),
    ]),
  ];
  let mut solver = Solver::new(constraints, None);

  let steps = Thermo.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.cells, vec![ CellPosition::new(6, 0) ]);
  assert_eq!(step.values, vec![4]);
  let CellPosition { row, col } = step.cells[0];
  let rule_value = step.values[0];
  let initial_value = solver.grid[row][col];
  assert!(initial_value == 0);

  solver.apply_rule(&mut step);

  let final_value = solver.grid[row][col];
  assert!(final_value == rule_value);
}

#[test]
fn check_full_thermo() {
  let grid_size = 4;
  let fixed_numbers = vec![
    FixedNumber::new(0, 0, 1),
    FixedNumber::new(1, 0, 2),
    FixedNumber::new(2, 0, 3),
    FixedNumber::new(3, 0, 4),
  ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.thermos = vec![
    crate::types::Thermo(vec![
      CellPosition::new(0, 0),
      CellPosition::new(1, 0),
      CellPosition::new(2, 0),
      CellPosition::new(3, 0),
    ]),
  ];
  let solver = Solver::new(constraints, None);

  let steps = Thermo.run(&solver);
  assert!(steps.is_empty());
}
