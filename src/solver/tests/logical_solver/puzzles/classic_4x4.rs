use crate::{solver::Solver, types::{FixedNumber, Grid, SolutionType, SudokuConstraints}};

#[test]
fn check_classic_4x4_1_solve() {
  let constraints = SudokuConstraints::new(4)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(1, 1, 4),
        FixedNumber::new(1, 3, 2),
        FixedNumber::new(2, 0, 1),
        FixedNumber::new(2, 2, 3),
      ]
    );
  let mut solver = Solver::new(constraints);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(
    result.solution.unwrap(),
    Grid(vec![
      vec![ 2, 1, 4, 3 ],
      vec![ 3, 4, 1, 2 ],
      vec![ 1, 2, 3, 4 ],
      vec![ 4, 3, 2, 1 ],
    ])
  );
  insta::assert_yaml_snapshot!(result.steps);
}
