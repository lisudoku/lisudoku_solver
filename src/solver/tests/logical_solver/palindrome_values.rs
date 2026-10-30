use crate::{solver::logical::{logical_solver::LogicalSolver, techniques::{Technique, palindrome_values::PalindromeValues}}, types::{Area, CellPosition, FixedNumber, Palindrome, Rule, SudokuConstraints}};

#[test]
fn check_palindrome_values() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![FixedNumber::new(0, 0, 1), FixedNumber::new(0, 3, 2)]
    )
    .with_palindromes(
      vec![
        Palindrome(vec![
          CellPosition::new(0, 0), CellPosition::new(0, 1), CellPosition::new(0, 2),
          CellPosition::new(0, 3), CellPosition::new(0, 4),
        ])
      ]
    );
  let mut solver = LogicalSolver::new(constraints);

  let steps = PalindromeValues.run(&solver);
  assert_eq!(steps.len(), 2);

  let step = &steps[0];
  assert_eq!(step.rule, Rule::PalindromeValues);
  assert_eq!(step.cells, vec![CellPosition::new(0, 4)]);
  assert_eq!(step.values, vec![1]);
  assert_eq!(step.areas, vec![ Area::Palindrome(0) ]);
  assert_eq!(solver.state.value_at(&CellPosition::new(0, 4)), 0);
  solver.apply_rule(&step);
  assert_eq!(solver.state.value_at(&CellPosition::new(0, 4)), 1);

  let step = &steps[1];
  assert_eq!(step.rule, Rule::PalindromeValues);
  assert_eq!(step.cells, vec![CellPosition::new(0, 1)]);
  assert_eq!(step.values, vec![2]);
  assert_eq!(step.areas, vec![ Area::Palindrome(0) ]);
  assert_eq!(solver.state.value_at(&CellPosition::new(0, 1)), 0);
  solver.apply_rule(&step);
  assert_eq!(solver.state.value_at(&CellPosition::new(0, 1)), 2);
}
