use crate::{solver::logical::{logical_solver::LogicalSolver, techniques::{TechniqueDefinition, TechniqueKind, candidate_initialization::CandidateInitialization, hidden_set::HiddenSet}}, types::{CellPosition, FixedNumber, SudokuConstraints}};

#[test]
fn check_hidden_triples() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      [
        FixedNumber::new(3, 1, 1),
        FixedNumber::new(4, 1, 2),
        FixedNumber::new(5, 1, 3),
        FixedNumber::new(6, 2, 1),
        FixedNumber::new(7, 2, 2),
        FixedNumber::new(8, 2, 3),
        FixedNumber::new(0, 3, 1),
      ]
    );
  let mut solver = LogicalSolver::new(constraints);
  solver.apply_step(&mut CandidateInitialization.run(&solver).first().unwrap());

  let steps = HiddenSet::new(3).run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.technique_kind, TechniqueKind::HiddenTriple);
  assert_eq!(step.cells, vec![ CellPosition::new(0, 0), CellPosition::new(1, 0), CellPosition::new(2, 0) ]);
  assert_eq!(step.values, vec![ 1, 2, 3 ]);
  assert_eq!(solver.state.candidates[0][0].len(), 8);
  assert_eq!(solver.state.candidates[1][0].len(), 9);
  assert_eq!(solver.state.candidates[2][0].len(), 9);

  solver.apply_step(&mut step);
  assert_eq!(solver.state.candidates[0][0].len(), 2);
  assert_eq!(solver.state.candidates[1][0].len(), 3);
  assert_eq!(solver.state.candidates[2][0].len(), 3);
}

#[test]
fn check_hidden_triples_no_affected_cells() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      [
        FixedNumber::new(0, 3, 5),
        FixedNumber::new(0, 4, 4),
        FixedNumber::new(0, 5, 2),
        FixedNumber::new(0, 6, 1),
        FixedNumber::new(0, 7, 3),
        FixedNumber::new(0, 8, 7),
        FixedNumber::new(1, 0, 3),
        FixedNumber::new(1, 1, 5),
        FixedNumber::new(1, 2, 4),
        FixedNumber::new(1, 3, 7),
        FixedNumber::new(1, 4, 8),
        FixedNumber::new(1, 5, 1),
        FixedNumber::new(1, 6, 2),
        FixedNumber::new(1, 7, 9),
        FixedNumber::new(1, 8, 6),
        FixedNumber::new(2, 0, 1),
        FixedNumber::new(2, 1, 7),
        FixedNumber::new(2, 2, 2),
        FixedNumber::new(2, 3, 9),
        FixedNumber::new(2, 4, 6),
        FixedNumber::new(2, 5, 3),
        FixedNumber::new(2, 6, 4),
        FixedNumber::new(2, 7, 8),
        FixedNumber::new(2, 8, 5),
      ]
    );
  let mut solver = LogicalSolver::new(constraints);
  solver.apply_step(&mut CandidateInitialization.run(&solver).first().unwrap());

  let steps = HiddenSet::new(3).run(&solver);
  assert!(steps.is_empty());
}
