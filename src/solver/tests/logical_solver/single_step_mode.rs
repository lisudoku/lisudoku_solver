use crate::{types::{SudokuConstraints, SolutionType, FixedNumber, Rule}, solver::{Solver, logical_solver::{hidden_singles::HiddenSingles, naked_singles::NakedSingle, technique::Technique}}};
use std::rc::Rc;
use itertools::Itertools;

#[test]
fn check_single_step_mode_all_techniques() {
  let constraints = SudokuConstraints::new(4)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 0, 3), FixedNumber::new(1, 1, 1),
        FixedNumber::new(2, 1, 2), FixedNumber::new(2, 2, 1),
      ]
    );

  let techniques: Vec<Rc<dyn Technique>> = vec![ Rc::new(NakedSingle), Rc::new(HiddenSingles) ];
  let mut solver = Solver::new(constraints).with_step_count_limit(1).with_techniques(techniques);
  let result = solver.logical_solve();

  assert_eq!(result.solution_type, SolutionType::Partial);
  assert_eq!(result.steps.len(), 7);
  let step_counts = result.steps.iter().counts_by(|step| step.rule);
  assert_eq!(step_counts[&Rule::NakedSingle], 2);
  assert_eq!(step_counts[&Rule::HiddenSingle], 5);
}

#[test]
fn check_single_step_mode_hidden_singles() {
  let constraints = SudokuConstraints::new(4)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 0, 3), FixedNumber::new(1, 1, 1),
        FixedNumber::new(2, 1, 2), FixedNumber::new(2, 2, 1),
      ]
    );

  let techniques: Vec<Rc<dyn Technique>> = vec![ Rc::new(HiddenSingles) ];
  let mut solver = Solver::new(constraints).with_step_count_limit(1).with_techniques(techniques);
  let result = solver.logical_solve();

  assert_eq!(result.solution_type, SolutionType::Partial);
  assert_eq!(result.steps.len(), 5);
  assert!(result.steps.iter().all(|step| step.rule == Rule::HiddenSingle));
}

#[test]
fn check_single_step_mode_naked_singles() {
  let constraints = SudokuConstraints::new(4)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 0, 3), FixedNumber::new(1, 1, 1),
        FixedNumber::new(2, 1, 2), FixedNumber::new(2, 2, 1),
      ]
    );

  let techniques: Vec<Rc<dyn Technique>> = vec![ Rc::new(NakedSingle) ];
  let mut solver = Solver::new(constraints).with_step_count_limit(1).with_techniques(techniques);
  let result = solver.logical_solve();

  assert_eq!(result.solution_type, SolutionType::Partial);
  assert_eq!(result.steps.len(), 2);
  assert!(result.steps.iter().all(|step| step.rule == Rule::NakedSingle));
}
