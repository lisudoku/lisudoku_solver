use crate::{types::{SudokuConstraints, FixedNumber, CellPosition, Area}, solver::{Solver, logical_solver::{candidates::Candidates, technique::Technique, hidden_singles::HiddenSingles}}};
use itertools::Itertools;

#[test]
fn check_hidden_single_in_region() {
  let constraints = SudokuConstraints::new(4)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(1, 3, 1),
        FixedNumber::new(3, 1, 1),
      ]
    );
  let mut solver = Solver::new(constraints);

  let steps = HiddenSingles.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.values, vec![1]);
  assert_eq!(step.cells[0], CellPosition::new(0, 0));
  assert_eq!(step.cells.iter().copied().collect::<Vec<CellPosition>>(), vec![
    CellPosition::new(0, 0), CellPosition::new(3, 1), CellPosition::new(1, 3),
  ]);
  assert!(step.affected_cells.is_empty());
  let CellPosition { row, col } = step.cells[0];
  let initial_value = solver.grid[row][col];
  assert!(initial_value == 0);

  solver.apply_rule(&mut step);
  let final_value = solver.grid[row][col];
  assert!(final_value == 1);
}

#[test]
fn check_hidden_single_on_row() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 3, 2),
        FixedNumber::new(0, 4, 3),
        FixedNumber::new(0, 5, 4),
        FixedNumber::new(2, 8, 1),
        FixedNumber::new(5, 2, 1),
        FixedNumber::new(6, 1, 1),
      ]
    );
  let mut solver = Solver::new(constraints);

  let steps = HiddenSingles.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.values, vec![1]);
  assert_eq!(step.areas, vec![Area::Row(0)]);
  assert_eq!(step.cells[0], CellPosition::new(0, 0));
  assert_eq!(step.cells.iter().copied().collect::<Vec<CellPosition>>(), vec![
    CellPosition::new(0, 0), CellPosition::new(6, 1), CellPosition::new(5, 2), CellPosition::new(2, 8),
  ]);
  assert!(step.affected_cells.is_empty());
  let CellPosition { row, col } = step.cells[0];
  let initial_value = solver.grid[row][col];
  assert!(initial_value == 0);

  solver.apply_rule(&mut step);
  let final_value = solver.grid[row][col];
  assert!(final_value == 1);
}

#[test]
fn check_hidden_single_on_col() {
  let constraints = SudokuConstraints::new(6)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 5, 1),
        FixedNumber::new(2, 3, 2),
        FixedNumber::new(3, 3, 3),
        FixedNumber::new(4, 0, 1),
      ]
    );
  let mut solver = Solver::new(constraints);

  let steps = HiddenSingles.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.values, vec![1]);
  assert_eq!(step.areas, vec![Area::Column(3)]);
  assert_eq!(step.cells[0], CellPosition::new(5, 3));
  assert_eq!(step.cells.iter().copied().collect::<Vec<CellPosition>>(), vec![
    CellPosition::new(5, 3), CellPosition::new(0, 5), CellPosition::new(4, 0),
  ]);
  assert_eq!(step.cells.len(), step.cells.iter().unique().count());
  assert!(step.affected_cells.is_empty());
  let CellPosition { row, col } = step.cells[0];
  let initial_value = solver.grid[row][col];
  assert!(initial_value == 0);

  solver.apply_rule(&mut step);
  let final_value = solver.grid[row][col];
  assert!(final_value == 1);
}

#[test]
fn check_hidden_single_in_region_with_candidates() {
  let constraints = SudokuConstraints::new(4)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(1, 3, 1),
        FixedNumber::new(3, 1, 1),
      ]
    );
  let mut solver = Solver::new(constraints);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = HiddenSingles.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.cells.len(), 1);
  let CellPosition { row, col } = step.cells[0];
  let rule_value = step.values[0];
  let initial_value = solver.grid[row][col];
  assert!(initial_value == 0);

  solver.apply_rule(&mut step);
  let final_value = solver.grid[row][col];
  assert!(final_value == rule_value);
}

#[test]
fn check_hidden_single_using_anti_knight_1() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 8, 1),
        FixedNumber::new(2, 3, 2),
        FixedNumber::new(3, 3, 1),
        FixedNumber::new(5, 0, 1),
        FixedNumber::new(6, 2, 1),
      ]
    )
    .with_anti_knight();
  let mut solver = Solver::new(constraints);

  let steps = HiddenSingles.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.areas, vec![Area::Row(2)]);
  assert_eq!(step.values, vec![1]);
  assert_eq!(step.cells[0], CellPosition::new(2, 4));
  assert_eq!(step.cells.iter().copied().collect::<Vec<CellPosition>>(), vec![
    CellPosition::new(2, 4), CellPosition::new(5, 0), CellPosition::new(3, 3),
    CellPosition::new(6, 2), CellPosition::new(0, 8),
  ]);
  assert!(step.affected_cells.is_empty());
  let CellPosition { row, col } = step.cells[0];
  let initial_value = solver.grid[row][col];
  assert!(initial_value == 0);

  solver.apply_rule(&mut step);
  let final_value = solver.grid[row][col];
  assert!(final_value == 1);
}

#[test]
fn check_hidden_single_using_anti_knight_2() {
  let constraints = SudokuConstraints::new(6)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(1, 0, 1),
        FixedNumber::new(1, 1, 2),
        FixedNumber::new(1, 2, 3),
        FixedNumber::new(1, 4, 4),
        FixedNumber::new(3, 2, 5),
      ]
    )
    .with_anti_knight();
  let mut solver = Solver::new(constraints);

  let steps = HiddenSingles.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.areas, vec![Area::Row(1)]);
  assert_eq!(step.values, vec![5]);
  assert_eq!(step.cells[0], CellPosition::new(1, 5));
  assert_eq!(step.cells.iter().copied().collect::<Vec<CellPosition>>(), vec![
    CellPosition::new(1, 5), CellPosition::new(3, 2),
  ]);
  assert!(step.affected_cells.is_empty());
  let CellPosition { row, col } = step.cells[0];
  let initial_value = solver.grid[row][col];
  assert!(initial_value == 0);

  solver.apply_rule(&mut step);
  let final_value = solver.grid[row][col];
  assert!(final_value == 5);
}

#[test]
fn check_hidden_single_anti_king() {
  let constraints = SudokuConstraints::new(6)
    .with_fixed_numbers(
      vec![ FixedNumber::new(3, 1, 2), FixedNumber::new(5, 0, 3) ]
    )
    .with_anti_king();
  let mut solver = Solver::new(constraints);

  let steps = HiddenSingles.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.values, vec![2]);
  assert_eq!(step.cells, vec![ CellPosition::new(5, 2), CellPosition::new(3, 1) ]);
  assert!(step.affected_cells.is_empty());
  let CellPosition { row, col } = step.cells[0];
  let initial_value = solver.grid[row][col];
  assert!(initial_value == 0);

  solver.apply_rule(&mut step);
  let final_value = solver.grid[row][col];
  assert!(final_value == 2);
}
