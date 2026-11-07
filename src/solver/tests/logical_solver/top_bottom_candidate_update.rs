use crate::{solver::logical::{logical_solver::LogicalSolver, techniques::{TechniqueDefinition, TechniqueKind, candidate_initialization::CandidateInitialization, top_bottom_candidate_update::TopBottomCandidateUpdate}}, types::{Area, CellPosition, FixedNumber, SudokuConstraints}};

#[test]
fn check_top_bottom_candidates_ascending_row1() {
  let constraints = SudokuConstraints::new(4)
    .with_fixed_numbers(
      [ FixedNumber::new(1, 3, 2) ]
    )
    .with_top_bottom();
  let mut solver = LogicalSolver::new(constraints);

  solver.apply_step(&mut CandidateInitialization.run(&solver).first().unwrap());
  let steps = TopBottomCandidateUpdate::new(false).run(&solver);
  assert!(!steps.is_empty());
  let step = steps.into_iter().next().unwrap();
  assert_eq!(step.technique_kind, TechniqueKind::TopBottomCandidateUpdate);
  assert_eq!(step.values, vec![1]);
  assert_eq!(step.areas, vec![Area::Row(0), Area::Row(1)]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(0, 0), CellPosition::new(0, 1) ]);

  assert!(solver.state.candidates[0][0].contains(&1));
  solver.apply_step(&step);
  assert!(!solver.state.candidates[0][0].contains(&1));
  assert_eq!(solver.state.candidates[0][0].len(), 3);
}

#[test]
fn check_top_bottom_candidates_ascending_row4() {
  let constraints = SudokuConstraints::new(4)
    .with_fixed_numbers(
      [
        FixedNumber::new(0, 2, 1), FixedNumber::new(1, 2, 2), FixedNumber::new(2, 2, 3),
      ]
    )
    .with_top_bottom();
  let mut solver = LogicalSolver::new(constraints);

  solver.apply_step(&mut CandidateInitialization.run(&solver).first().unwrap());
  let steps = TopBottomCandidateUpdate::new(false).run(&solver);
  assert!(!steps.is_empty());
  let step = steps.into_iter().next().unwrap();
  assert_eq!(step.technique_kind, TechniqueKind::TopBottomCandidateUpdate);
  assert_eq!(step.values, vec![4]);
  assert_eq!(step.areas, vec![Area::Row(3), Area::Row(2)]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(3, 0) ]);

  assert!(solver.state.candidates[3][0].contains(&4));
  solver.apply_step(&step);
  assert!(!solver.state.candidates[3][0].contains(&4));
  assert_eq!(solver.state.candidates[3][0].len(), 3);
}

#[test]
fn check_top_bottom_candidates_descending_row3() {
  let constraints = SudokuConstraints::new(4)
    .with_fixed_numbers(
      [ FixedNumber::new(3, 3, 1) ]
    )
    .with_top_bottom();
  let mut solver = LogicalSolver::new(constraints);

  solver.apply_step(&mut CandidateInitialization.run(&solver).first().unwrap());
  let steps = TopBottomCandidateUpdate::new(false).run(&solver);
  assert!(!steps.is_empty());
  let step = steps.into_iter().next().unwrap();
  assert_eq!(step.technique_kind, TechniqueKind::TopBottomCandidateUpdate);
  assert_eq!(step.values, vec![2]);
  assert_eq!(step.areas, vec![Area::Row(2), Area::Row(3)]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(2, 0), CellPosition::new(2, 1) ]);

  assert!(solver.state.candidates[2][0].contains(&2));
  solver.apply_step(&step);
  assert!(!solver.state.candidates[2][0].contains(&2));
  assert_eq!(solver.state.candidates[2][0].len(), 3);
}

#[test]
fn check_top_bottom_candidates_descending_row1() {
  let constraints = SudokuConstraints::new(4)
    .with_fixed_numbers(
      [
        FixedNumber::new(1, 0, 3), FixedNumber::new(2, 0, 2), FixedNumber::new(3, 0, 1),
      ]
    )
    .with_top_bottom();
  let mut solver = LogicalSolver::new(constraints);

  solver.apply_step(&mut CandidateInitialization.run(&solver).first().unwrap());
  let steps = TopBottomCandidateUpdate::new(false).run(&solver);
  assert!(!steps.is_empty());
  let step = steps.into_iter().next().unwrap();
  assert_eq!(step.technique_kind, TechniqueKind::TopBottomCandidateUpdate);
  assert_eq!(step.values, vec![4]);
  assert_eq!(step.areas, vec![Area::Row(0), Area::Row(1)]);
  assert_eq!(step.affected_cells, vec![ CellPosition::new(0, 2), CellPosition::new(0, 3) ]);

  assert!(solver.state.candidates[0][2].contains(&4));
  solver.apply_step(&step);
  assert!(!solver.state.candidates[0][2].contains(&4));
  assert_eq!(solver.state.candidates[0][2].len(), 3);
}
