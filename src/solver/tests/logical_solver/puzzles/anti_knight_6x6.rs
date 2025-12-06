use crate::{solver::Solver, types::{FixedNumber, Grid, SolutionType, SudokuConstraints}};

#[test]
fn check_anti_knight_6x6_1_solve() {
  let constraints = SudokuConstraints::new(6)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 1, 5),
        FixedNumber::new(0, 2, 1),
        FixedNumber::new(0, 3, 3),
        FixedNumber::new(0, 4, 6),
        FixedNumber::new(1, 2, 2),
        FixedNumber::new(1, 3, 4),
        FixedNumber::new(5, 0, 2),
        FixedNumber::new(5, 5, 4),
      ]
    )
    .with_anti_knight();
  let mut solver = Solver::new(constraints);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(
    result.solution.unwrap(),
    Grid(vec![
      vec![ 4, 5, 1, 3, 6, 2 ],
      vec![ 3, 6, 2, 4, 5, 1 ],
      vec![ 1, 2, 6, 5, 4, 3 ],
      vec![ 5, 4, 3, 1, 2, 6 ],
      vec![ 6, 3, 4, 2, 1, 5 ],
      vec![ 2, 1, 5, 6, 3, 4 ],
    ])
  );
  insta::assert_yaml_snapshot!(result.steps);
}
