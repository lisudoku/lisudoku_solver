use crate::{solver::logical::{logical_solver::LogicalSolver, techniques::{TechniqueDefinition, TechniqueKind, candidate_initialization::CandidateInitialization, naked_single::NakedSingle}}, types::{Area, CellPosition, FixedNumber, Grid, SolutionStep, SudokuConstraints}};

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
  let mut solver = LogicalSolver::new(constraints);

  let steps = NakedSingle.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert!(step.affected_cells.is_empty());
  assert_eq!(step.cells, vec![CellPosition::new(1, 0)]);
  assert_eq!(step.values, vec![3]);
  assert_eq!(step.areas, vec![Area::Row(1), Area::Column(0)]);

  let cell = step.cells[0];
  let initial_value = solver.state.value_at(&cell);
  assert!(initial_value == 0);
  let step_value = step.values[0];
  solver.apply_step(&mut step);
  let final_value = solver.state.value_at(&cell);
  assert!(final_value == step_value);
  assert!(solver.state.candidates[cell.row][cell.col].is_empty());
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
  let mut solver = LogicalSolver::new(constraints);
  solver.apply_step(&mut CandidateInitialization.run(&solver).first().unwrap());

  let steps = NakedSingle.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(
    *step,
    SolutionStep::new(
      TechniqueKind::NakedSingle,
      vec![CellPosition::new(1, 0)],
      vec![3],
      vec![Area::Row(1), Area::Column(0)],
      vec![CellPosition::new(0, 0), CellPosition::new(0, 1), CellPosition::new(3, 0)],
    ),
  );

  let cell = step.cells[0];
  let initial_value = solver.state.value_at(&cell);
  assert!(initial_value == 0);
  let step_value = step.values[0];
  solver.apply_step(&mut step);
  let final_value = solver.state.value_at(&cell);
  assert!(final_value == step_value);
  assert!(solver.state.candidates[cell.row][cell.col].is_empty());
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
  let mut solver = LogicalSolver::new(constraints);

  let steps = NakedSingle.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.cells, vec![CellPosition::new(2, 2)]);
  assert_eq!(step.values, vec![9]);
  assert_eq!(step.areas, vec![Area::Row(2), Area::Column(2), Area::Region(0), Area::Cell(4, 3)]);

  let cell = step.cells[0];
  let initial_value = solver.state.value_at(&cell);
  assert!(initial_value == 0);
  let step_value = step.values[0];
  solver.apply_step(&mut step);
  let final_value = solver.state.value_at(&cell);
  assert!(final_value == step_value);
  assert!(solver.state.candidates[cell.row][cell.col].is_empty());
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
  let solver = LogicalSolver::new(constraints);

  let steps = NakedSingle.run(&solver);
  assert!(!steps.is_empty());
  let step = steps.first().unwrap();
  assert_eq!(
    *step,
    SolutionStep::new(
      TechniqueKind::NakedSingle,
      vec![CellPosition::new(1, 2)],
      vec![2],
      vec![Area::Region(1), Area::Cell(0, 0)],
      vec![],
    ),
  );
}
