use crate::{solver::logical::{logical_solver::LogicalSolver, techniques::{TechniqueDefinition, TechniqueKind, candidate_initialization::CandidateInitialization, palindrome_candidate_update::PalindromeCandidateUpdate}}, types::{Area, CellPosition, FixedNumber, Palindrome, SudokuConstraints}};

#[test]
fn check_palindrome_candidates() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      [FixedNumber::new(3, 0, 1), FixedNumber::new(3, 2, 2)]
    )
    .with_palindromes(
      [
        Palindrome(vec![
          CellPosition::new(0, 0), CellPosition::new(0, 1), CellPosition::new(0, 2),
        ])
      ]
    );
  let mut solver = LogicalSolver::new(constraints);
  solver.apply_step(&mut CandidateInitialization.run(&solver).first().unwrap());

  let steps = PalindromeCandidateUpdate.run(&solver);
  assert_eq!(steps.len(), 2);

  let step = &steps[0];
  assert_eq!(step.technique_kind, TechniqueKind::PalindromeCandidateUpdate);
  assert_eq!(step.affected_cells, vec![CellPosition::new(0, 0)]);
  assert_eq!(step.values, vec![2]);
  assert_eq!(step.areas, vec![ Area::Palindrome(0) ]);
  assert!(solver.state.candidates[0][0].contains(&2));
  solver.apply_step(&step);
  assert!(!solver.state.candidates[0][0].contains(&2));

  let step = &steps[1];
  assert_eq!(step.technique_kind, TechniqueKind::PalindromeCandidateUpdate);
  assert_eq!(step.affected_cells, vec![CellPosition::new(0, 2)]);
  assert_eq!(step.values, vec![1]);
  assert_eq!(step.areas, vec![ Area::Palindrome(0) ]);
  assert!(solver.state.candidates[0][2].contains(&1));
  solver.apply_step(&step);
  assert!(!solver.state.candidates[0][2].contains(&1));
}
