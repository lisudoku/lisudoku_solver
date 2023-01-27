use crate::{types::{SudokuConstraints, FixedNumber, CellPosition, Rule}, solver::Solver};
use itertools::Itertools;

#[test]
fn check_grid_steps_without_candidates() {
  let grid_size = 4;
  let fixed_numbers = vec![
    FixedNumber::new(1, 1, 4),
    FixedNumber::new(1, 3, 2),
    FixedNumber::new(2, 0, 1),
    FixedNumber::new(2, 2, 3),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);

  let steps = solver.find_grid_steps();
  assert!(!steps.is_empty());
  let mut step = steps.into_iter().next().unwrap();
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
fn check_grid_steps_with_candidates() {
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

  let steps = solver.find_grid_steps();
  assert!(!steps.is_empty());
  let mut step = steps.into_iter().next().unwrap();
  assert!(!step.affected_cells.is_empty());
  assert!(step.affected_cells.len() <= 4);
  let CellPosition { row, col } = step.cells[0];
  let CellPosition { row: cand_row, col: cand_col } = step.affected_cells[0];
  let rule_value = step.values[0];
  let initial_value = solver.grid[row][col];
  let initial_candidates = solver.candidates[cand_row][cand_col].clone();
  assert!(initial_value == 0);

  solver.apply_rule(&mut step);
  let final_value = solver.grid[row][col];
  let final_candidates = &solver.candidates[cand_row][cand_col];
  assert!(final_value == rule_value);
  assert!(final_candidates.len() < initial_candidates.len());
}

#[test]
fn check_grid_steps_with_anti_knight_affected_cells() {
  let grid_size = 4;
  let fixed_numbers = vec![
    FixedNumber::new(0, 0, 1),
    FixedNumber::new(0, 1, 2),
    FixedNumber::new(1, 0, 3),
  ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.anti_knight = true;
  let mut solver = Solver::new(constraints, None);
  // TODO: will have to fix version without candidates_active
  solver.apply_rule(&mut solver.find_candidates_step().unwrap());

  let steps = solver.find_grid_steps();
  assert!(!steps.is_empty());
  let mut step = steps.into_iter().next().unwrap();
  assert_eq!(step.values, vec![4]);
  assert_eq!(step.rule, Rule::NakedSingle);
  assert_eq!(step.cells.iter().copied().sorted().collect::<Vec<CellPosition>>(), vec![
    CellPosition::new(0, 2),
  ]);
  assert_eq!(step.affected_cells.iter().copied().sorted().collect::<Vec<CellPosition>>(), vec![
    CellPosition::new(0, 3), CellPosition::new(1, 2), CellPosition::new(1, 3),
    CellPosition::new(2, 1), CellPosition::new(2, 2), CellPosition::new(2, 3),
    CellPosition::new(3, 2),
  ]);
  let CellPosition { row, col } = step.cells[0];
  let CellPosition { row: cand_row, col: cand_col } = step.affected_cells[0];
  let initial_value = solver.grid[row][col];
  let initial_candidates = solver.candidates[cand_row][cand_col].clone();
  assert!(initial_value == 0);

  solver.apply_rule(&mut step);
  let final_value = solver.grid[row][col];
  let final_candidates = &solver.candidates[cand_row][cand_col];
  assert!(final_value == 4);
  assert_eq!(final_candidates.len(), initial_candidates.len() - 1);
}

#[test]
fn check_grid_steps_overlapping_thermos_affected_cells() {
  let grid_size = 6;
  let fixed_numbers = vec![
    FixedNumber::new(5, 0, 1),
    FixedNumber::new(5, 1, 2),
    FixedNumber::new(5, 2, 3),
    FixedNumber::new(4, 0, 4),
    FixedNumber::new(4, 1, 5),
  ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.thermos = vec![
    vec![
      CellPosition::new(4, 2),
      CellPosition::new(3, 1),
    ],
    vec![
      CellPosition::new(4, 2),
      CellPosition::new(3, 3),
    ],
  ];
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut solver.find_candidates_step().unwrap());

  let steps = solver.find_grid_steps();
  assert!(!steps.is_empty());
  let mut step = steps.into_iter().next().unwrap();
  assert_eq!(step.values, vec![6]);
  assert_eq!(step.rule, Rule::NakedSingle);
  assert_eq!(step.cells.iter().copied().sorted().collect::<Vec<CellPosition>>(), vec![
    CellPosition::new(4, 2),
  ]);
  assert_eq!(step.affected_cells.iter().copied().sorted().collect::<Vec<CellPosition>>(), vec![
    CellPosition::new(0, 2), CellPosition::new(1, 2), CellPosition::new(2, 2),
    CellPosition::new(3, 1), CellPosition::new(3, 2), CellPosition::new(3, 3),
    CellPosition::new(4, 3), CellPosition::new(4, 4), CellPosition::new(4, 5),
  ]);
  assert!(solver.candidates[3][1].contains(&6));

  solver.apply_rule(&mut step);
  assert!(!solver.candidates[3][1].contains(&6));
}
