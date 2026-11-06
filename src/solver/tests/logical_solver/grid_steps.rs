use crate::{solver::logical::{logical_solver::LogicalSolver, techniques::{TechniqueDefinition, TechniqueKind, candidate_initialization::CandidateInitialization}}, types::{Area, CellPosition, FixedNumber, SolutionStep, SudokuConstraints, Thermo}};

#[test]
fn check_grid_steps_without_candidates() {
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

  let steps = solver.find_grid_steps();
  assert!(!steps.is_empty());
  let mut step = steps.into_iter().next().unwrap();
  assert!(step.affected_cells.is_empty());
  let cell = step.cells[0];
  let step_value = step.values[0];
  let initial_value = solver.state.value_at(&cell);
  assert!(initial_value == 0);

  solver.apply_step(&mut step);

  let final_value = solver.state.value_at(&cell);
  assert!(final_value == step_value);
}

#[test]
fn check_grid_steps_with_candidates() {
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

  let steps = solver.find_grid_steps();
  assert!(!steps.is_empty());
  let mut step = steps.into_iter().next().unwrap();
  assert!(!step.affected_cells.is_empty());
  assert!(step.affected_cells.len() <= 4);
  let cell = step.cells[0];
  let CellPosition { row: cand_row, col: cand_col } = step.affected_cells[0];
  let step_value = step.values[0];
  let initial_value = solver.state.value_at(&cell);
  let initial_candidates = solver.state.candidates[cand_row][cand_col].clone();
  assert!(initial_value == 0);

  solver.apply_step(&mut step);
  let final_value = solver.state.value_at(&cell);
  let final_candidates = &solver.state.candidates[cand_row][cand_col];
  assert!(final_value == step_value);
  assert!(final_candidates.len() < initial_candidates.len());
}

#[test]
fn check_grid_steps_with_anti_knight_affected_cells() {
  let constraints = SudokuConstraints::new(6)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 0, 1),
        FixedNumber::new(0, 1, 2),
        FixedNumber::new(0, 2, 3),
        FixedNumber::new(0, 5, 6),
        FixedNumber::new(1, 2, 4),
      ]
    )
    .with_anti_knight();
  let mut solver = LogicalSolver::new(constraints);
  solver.apply_step(&mut CandidateInitialization.run(&solver).first().unwrap());

  let steps = solver.find_grid_steps();
  assert!(!steps.is_empty());
  let mut step = steps.into_iter().next().unwrap();
  assert_eq!(
    step,
    SolutionStep::new(
      TechniqueKind::HiddenSingle,
      vec![CellPosition::new(0, 3)],
      vec![4],
      vec![Area::Row(0)],
      vec![CellPosition::new(2, 3), CellPosition::new(4, 3), CellPosition::new(5, 3)],
    ),
  );
  let cell = step.cells[0];
  let CellPosition { row: cand_row, col: cand_col } = step.affected_cells[0];
  let initial_value = solver.state.value_at(&cell);
  let initial_candidates = solver.state.candidates[cand_row][cand_col].clone();
  assert!(initial_value == 0);

  solver.apply_step(&mut step);
  let final_value = solver.state.value_at(&cell);
  let final_candidates = &solver.state.candidates[cand_row][cand_col];
  assert!(final_value == 4);
  assert_eq!(final_candidates.len(), initial_candidates.len() - 1);
}

#[test]
fn check_grid_steps_overlapping_thermos_affected_cells() {
  let constraints = SudokuConstraints::new(6)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(5, 0, 1),
        FixedNumber::new(5, 1, 2),
        FixedNumber::new(5, 2, 3),
        FixedNumber::new(4, 0, 4),
        FixedNumber::new(4, 1, 5),
      ]
    )
    .with_thermos(
      vec![
        Thermo(vec![
          CellPosition::new(4, 2),
          CellPosition::new(3, 1),
        ]),
        Thermo(vec![
          CellPosition::new(4, 2),
          CellPosition::new(3, 3),
        ]),
      ]
    );
  let mut solver = LogicalSolver::new(constraints);
  solver.apply_step(&mut CandidateInitialization.run(&solver).first().unwrap());

  let steps = solver.find_grid_steps();
  assert!(!steps.is_empty());
  let mut step = steps.into_iter().next().unwrap();
  assert_eq!(step.values, vec![6]);
  assert_eq!(step.technique_kind, TechniqueKind::HiddenSingle);
  assert_eq!(step.cells, vec![
    CellPosition::new(4, 2),
  ]);
  assert_eq!(step.affected_cells, vec![
    CellPosition::new(4, 3), CellPosition::new(4, 4), CellPosition::new(4, 5),
    CellPosition::new(0, 2), CellPosition::new(1, 2), CellPosition::new(2, 2),
    CellPosition::new(3, 2), CellPosition::new(3, 1), CellPosition::new(3, 3),
  ]);
  assert!(solver.state.candidates[3][1].contains(&6));

  solver.apply_step(&mut step);
  assert!(!solver.state.candidates[3][1].contains(&6));
}
