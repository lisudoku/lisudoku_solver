use crate::{solver::logical::{logical_solver::LogicalSolver, techniques::{TechniqueDefinition, TechniqueKind, candidate_initialization::CandidateInitialization, hidden_single::HiddenSingle}}, types::{Area, CellPosition, FixedNumber, SolutionStep, SudokuConstraints}};
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
  let mut solver = LogicalSolver::new(constraints);

  let steps = HiddenSingle.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.values, vec![1]);
  assert_eq!(step.cells[0], CellPosition::new(0, 0));
  assert_eq!(step.cells, vec![
    CellPosition::new(0, 0), CellPosition::new(3, 1), CellPosition::new(1, 3),
  ]);
  assert!(step.affected_cells.is_empty());
  let cell = step.cells[0];
  let initial_value = solver.state.value_at(&cell);
  assert!(initial_value == 0);

  solver.apply_step(&mut step);
  let final_value = solver.state.value_at(&cell);
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
  let mut solver = LogicalSolver::new(constraints);

  let steps = HiddenSingle.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.values, vec![1]);
  assert_eq!(step.areas, vec![Area::Row(0)]);
  assert_eq!(step.cells[0], CellPosition::new(0, 0));
  assert_eq!(step.cells, vec![
    CellPosition::new(0, 0), CellPosition::new(6, 1), CellPosition::new(5, 2), CellPosition::new(2, 8),
  ]);
  assert!(step.affected_cells.is_empty());
  let cell = step.cells[0];
  let initial_value = solver.state.value_at(&cell);
  assert!(initial_value == 0);

  solver.apply_step(&mut step);
  let final_value = solver.state.value_at(&cell);
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
  let mut solver = LogicalSolver::new(constraints);

  let steps = HiddenSingle.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.values, vec![1]);
  assert_eq!(step.areas, vec![Area::Column(3)]);
  assert_eq!(step.cells[0], CellPosition::new(5, 3));
  assert_eq!(step.cells, vec![
    CellPosition::new(5, 3), CellPosition::new(0, 5), CellPosition::new(4, 0),
  ]);
  assert_eq!(step.cells.len(), step.cells.iter().unique().count());
  assert!(step.affected_cells.is_empty());
  let cell = step.cells[0];
  let initial_value = solver.state.value_at(&cell);
  assert!(initial_value == 0);

  solver.apply_step(&mut step);
  let final_value = solver.state.value_at(&cell);
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
  let mut solver = LogicalSolver::new(constraints);
  solver.apply_step(&mut CandidateInitialization.run(&solver).first().unwrap());

  let steps = HiddenSingle.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.cells.len(), 1);
  let cell = step.cells[0];
  let step_value = step.values[0];
  let initial_value = solver.state.value_at(&cell);
  assert!(initial_value == 0);

  solver.apply_step(&mut step);
  let final_value = solver.state.value_at(&cell);
  assert!(final_value == step_value);
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
        FixedNumber::new(8, 2, 1),
      ]
    )
    .with_anti_knight();
  let mut solver = LogicalSolver::new(constraints);

  let steps = HiddenSingle.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(
    *step,
    SolutionStep::new(
      TechniqueKind::HiddenSingle,
      vec![
        CellPosition::new(1, 1), CellPosition::new(0, 8), CellPosition::new(5, 0),
        CellPosition::new(8, 2), CellPosition::new(3, 3),
      ],
      vec![1],
      vec![Area::Region(0)],
      vec![],
    )
  );
  let cell = step.cells[0];
  let initial_value = solver.state.value_at(&cell);
  assert!(initial_value == 0);

  solver.apply_step(&mut step);
  let final_value = solver.state.value_at(&cell);
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
  let mut solver = LogicalSolver::new(constraints);

  let steps = HiddenSingle.run(&solver);
  assert!(!steps.is_empty());
  let expected_step = SolutionStep::new(
    TechniqueKind::HiddenSingle,
    vec![CellPosition::new(1, 5), CellPosition::new(3, 2)],
    vec![5],
    vec![Area::Row(1)],
    vec![],
  );
  assert!(steps.into_iter().find(|step| *step == expected_step).is_some());
  let mut step = expected_step.clone();
  let cell = step.cells[0];
  let initial_value = solver.state.value_at(&cell);
  assert!(initial_value == 0);

  solver.apply_step(&mut step);
  let final_value = solver.state.value_at(&cell);
  assert!(final_value == 5);
}

#[test]
fn check_hidden_single_anti_king() {
  let constraints = SudokuConstraints::new(6)
    .with_fixed_numbers(
      vec![ FixedNumber::new(3, 1, 2), FixedNumber::new(5, 0, 3) ]
    )
    .with_anti_king();
  let mut solver = LogicalSolver::new(constraints);

  let steps = HiddenSingle.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.values, vec![2]);
  assert_eq!(step.cells, vec![ CellPosition::new(5, 2), CellPosition::new(3, 1) ]);
  assert!(step.affected_cells.is_empty());
  let cell = step.cells[0];
  let initial_value = solver.state.value_at(&cell);
  assert!(initial_value == 0);

  solver.apply_step(&mut step);
  let final_value = solver.state.value_at(&cell);
  assert!(final_value == 2);
}
