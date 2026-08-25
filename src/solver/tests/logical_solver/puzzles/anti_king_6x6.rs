use crate::{solver::Solver, types::{FixedNumber, Grid, SolutionType, SudokuConstraints}};

#[test]
fn check_anti_king_6x6_1_solve() {
  let constraints = SudokuConstraints::new(6)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 0, 4), FixedNumber::new(1, 1, 3), FixedNumber::new(1, 4, 1),
        FixedNumber::new(3, 0, 6), FixedNumber::new(3, 1, 2),
        FixedNumber::new(2, 4, 6), FixedNumber::new(3, 4, 5),
        FixedNumber::new(5, 0, 3), FixedNumber::new(5, 5, 1),
      ]
    )
    .with_anti_king();
  let mut solver = Solver::new(constraints);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(
    result.solution.unwrap(),
    Grid(vec![
      vec![ 4, 5, 1, 3, 2, 6 ],
      vec![ 2, 3, 6, 4, 1, 5 ],
      vec![ 1, 4, 5, 2, 6, 3 ],
      vec![ 6, 2, 3, 1, 5, 4 ],
      vec![ 5, 1, 4, 6, 3, 2 ],
      vec![ 3, 6, 2, 5, 4, 1 ],
    ])
  );
  insta::assert_yaml_snapshot!(result.steps);
}
