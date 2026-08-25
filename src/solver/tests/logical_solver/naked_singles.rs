use crate::{solver::{Solver, logical_solver::{candidates::Candidates, naked_singles::NakedSingle, technique::Technique}}, types::{Area, CellPosition, FixedNumber, Grid, Rule, SolutionStep, SudokuConstraints}};

#[test]
fn check_naked_single_without_candidates() {
  let constraints = SudokuConstraints::new(4)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(1, 1, 4),
        FixedNumber::new(1, 3, 2),
        FixedNumber::new(2, 0, 1),
        FixedNumber::new(2, 2, 3),
      ]
    );
  let mut solver = Solver::new(constraints);

  let steps = NakedSingle.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert!(step.affected_cells.is_empty());
  assert_eq!(step.cells, vec![CellPosition::new(1, 0)]);
  assert_eq!(step.values, vec![3]);
  assert_eq!(step.areas, vec![Area::Row(1), Area::Column(0)]);

  let CellPosition { row, col } = step.cells[0];
  let initial_value = solver.grid[row][col];
  assert!(initial_value == 0);
  let rule_value = step.values[0];
  solver.apply_rule(&mut step);
  let final_value = solver.grid[row][col];
  assert!(final_value == rule_value);
  assert!(solver.candidates[row][col].is_empty());
}

#[test]
fn check_naked_single_with_candidates() {
  let constraints = SudokuConstraints::new(4)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(1, 1, 4),
        FixedNumber::new(1, 3, 2),
        FixedNumber::new(2, 0, 1),
        FixedNumber::new(2, 2, 3),
      ]
    );
  let mut solver = Solver::new(constraints);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = NakedSingle.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(
    *step,
    SolutionStep::new(
      Rule::NakedSingle,
      vec![CellPosition::new(1, 0)],
      vec![3],
      vec![Area::Row(1), Area::Column(0)],
      vec![CellPosition::new(0, 0), CellPosition::new(0, 1), CellPosition::new(3, 0)],
    ),
  );

  let CellPosition { row, col } = step.cells[0];
  let initial_value = solver.grid[row][col];
  assert!(initial_value == 0);
  let rule_value = step.values[0];
  solver.apply_rule(&mut step);
  let final_value = solver.grid[row][col];
  assert!(final_value == rule_value);
  assert!(solver.candidates[row][col].is_empty());
}

#[test]
fn check_naked_single_using_anti_knight_without_candidates() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 0, 1),
    FixedNumber::new(0, 1, 2),
    FixedNumber::new(0, 2, 3),
    FixedNumber::new(2, 6, 4),
    FixedNumber::new(2, 7, 5),
    FixedNumber::new(2, 8, 6),
    FixedNumber::new(6, 2, 7),
    FixedNumber::new(4, 3, 8),
  ];
  let constraints = SudokuConstraints::new(grid_size)
    .with_fixed_numbers(fixed_numbers)
    .with_anti_knight();
  let mut solver = Solver::new(constraints);

  let steps = NakedSingle.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.cells, vec![CellPosition::new(2, 2)]);
  assert_eq!(step.values, vec![9]);
  assert_eq!(step.areas, vec![Area::Row(2), Area::Column(2), Area::Region(0), Area::Cell(4, 3)]);

  let CellPosition { row, col } = step.cells[0];
  let initial_value = solver.grid[row][col];
  assert!(initial_value == 0);
  let rule_value = step.values[0];
  solver.apply_rule(&mut step);
  let final_value = solver.grid[row][col];
  assert!(final_value == rule_value);
  assert!(solver.candidates[row][col].is_empty());
}

#[test]
fn check_naked_single_using_anti_knight_lots_of_peers_without_candidates() {
  let grid_size = 4;
  let fixed_numbers = Grid::from_string("
    1243
    3400
    4000
    0000
  ").to_fixed_numbers();
  let constraints = SudokuConstraints::new(grid_size)
    .with_fixed_numbers(fixed_numbers)
    .with_anti_knight();
  let solver = Solver::new(constraints);

  let steps = NakedSingle.run(&solver);
  assert!(!steps.is_empty());
  let step = steps.first().unwrap();
  assert_eq!(
    *step,
    SolutionStep::new(
      Rule::NakedSingle,
      vec![CellPosition::new(1, 2)],
      vec![2],
      vec![Area::Region(1), Area::Cell(0, 0)],
      vec![],
    ),
  );
}
