use crate::{solver::Solver, types::{FixedNumber, Grid, SolutionType, SudokuConstraints}};

#[test]
fn check_anti_knight_4x4_1_solve() {
  let constraints = SudokuConstraints::new(4)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 0, 1),
        FixedNumber::new(0, 1, 2),
        FixedNumber::new(1, 0, 3),
      ]
    )
    .with_anti_knight();
  let mut solver = Solver::new(constraints);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(
    result.solution.unwrap(),
    Grid(vec![
      vec![ 1, 2, 4, 3 ],
      vec![ 3, 4, 2, 1 ],
      vec![ 4, 3, 1, 2 ],
      vec![ 2, 1, 3, 4 ],
    ])
  );
  insta::assert_yaml_snapshot!(result.steps);
}
