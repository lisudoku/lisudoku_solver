use crate::{solver::logical::{logical_solver::LogicalSolver, techniques::TechniqueKind}, types::{FixedNumber, SolutionType, SudokuConstraints}};
use itertools::Itertools;

#[test]
fn check_single_step_mode_all_techniques() {
  let constraints = SudokuConstraints::new(4)
    .with_fixed_numbers(
      [
        FixedNumber::new(0, 0, 3), FixedNumber::new(1, 1, 1),
        FixedNumber::new(2, 1, 2), FixedNumber::new(2, 2, 1),
      ]
    );

  let technique_kinds: Vec<TechniqueKind> = vec![TechniqueKind::NakedSingle, TechniqueKind::HiddenSingle];
  let mut solver = LogicalSolver::new(constraints).with_step_count_limit(1).with_techniques(technique_kinds);
  let result = solver.solve();

  assert_eq!(result.solution_type, SolutionType::Partial);
  assert_eq!(result.steps.len(), 7);
  let step_counts = result.steps.iter().counts_by(|step| step.technique_kind);
  assert_eq!(step_counts[&TechniqueKind::NakedSingle], 2);
  assert_eq!(step_counts[&TechniqueKind::HiddenSingle], 5);
}

#[test]
fn check_single_step_mode_hidden_singles() {
  let constraints = SudokuConstraints::new(4)
    .with_fixed_numbers(
      [
        FixedNumber::new(0, 0, 3), FixedNumber::new(1, 1, 1),
        FixedNumber::new(2, 1, 2), FixedNumber::new(2, 2, 1),
      ]
    );

  let technique_kinds: Vec<TechniqueKind> = vec![TechniqueKind::HiddenSingle];
  let mut solver = LogicalSolver::new(constraints).with_step_count_limit(1).with_techniques(technique_kinds);
  let result = solver.solve();

  assert_eq!(result.solution_type, SolutionType::Partial);
  assert_eq!(result.steps.len(), 5);
  assert!(result.steps.iter().all(|step| step.technique_kind == TechniqueKind::HiddenSingle));
}

#[test]
fn check_single_step_mode_naked_singles() {
  let constraints = SudokuConstraints::new(4)
    .with_fixed_numbers(
      [
        FixedNumber::new(0, 0, 3), FixedNumber::new(1, 1, 1),
        FixedNumber::new(2, 1, 2), FixedNumber::new(2, 2, 1),
      ]
    );

  let technique_kinds: Vec<TechniqueKind> = vec![TechniqueKind::NakedSingle];
  let mut solver = LogicalSolver::new(constraints).with_step_count_limit(1).with_techniques(technique_kinds);
  let result = solver.solve();

  assert_eq!(result.solution_type, SolutionType::Partial);
  assert_eq!(result.steps.len(), 2);
  assert!(result.steps.iter().all(|step| step.technique_kind == TechniqueKind::NakedSingle));
}
