use crate::{solver::{Solver, logical_solver::{candidates::Candidates, palindrome_candidates::PalindromeCandidates, technique::Technique}}, types::{Area, CellPosition, FixedNumber, Palindrome, Rule, SudokuConstraints}};

#[test]
fn check_palindrome_candidates() {
  let grid_size = 9;
  let fixed_numbers = vec![FixedNumber::new(3, 0, 1), FixedNumber::new(3, 2, 2)];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let palindrome = Palindrome(vec![
    CellPosition::new(0, 0), CellPosition::new(0, 1), CellPosition::new(0, 2),
  ]);
  constraints.palindromes = vec![palindrome.clone()];
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = PalindromeCandidates.run(&solver);
  assert_eq!(steps.len(), 2);

  let step = &steps[0];
  assert_eq!(step.rule, Rule::PalindromeCandidates);
  assert_eq!(step.affected_cells, vec![CellPosition::new(0, 0)]);
  assert_eq!(step.values, vec![2]);
  assert_eq!(step.areas, vec![ Area::Palindrome(0) ]);
  assert!(solver.candidates[0][0].contains(&2));
  solver.apply_rule(&step);
  assert!(!solver.candidates[0][0].contains(&2));

  let step = &steps[1];
  assert_eq!(step.rule, Rule::PalindromeCandidates);
  assert_eq!(step.affected_cells, vec![CellPosition::new(0, 2)]);
  assert_eq!(step.values, vec![1]);
  assert_eq!(step.areas, vec![ Area::Palindrome(0) ]);
  assert!(solver.candidates[0][2].contains(&1));
  solver.apply_rule(&step);
  assert!(!solver.candidates[0][2].contains(&1));
}
