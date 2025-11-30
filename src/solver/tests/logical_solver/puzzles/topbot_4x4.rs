use crate::{solver::Solver, types::{FixedNumber, Grid, SolutionType, SudokuConstraints}};

#[test]
fn check_topbot_4x4_1() {
  let grid_size = 4;
  let fixed_numbers = vec![
    FixedNumber::new(0, 0, 4), FixedNumber::new(0, 3, 1),
    FixedNumber::new(3, 0, 1), FixedNumber::new(3, 3, 4),
  ];
  let empty_cells = grid_size * grid_size - fixed_numbers.len();
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers).with_top_bottom();
  let mut solver = Solver::new(constraints, None);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(
    result.solution.unwrap(),
    Grid(vec![
      vec![ 4, 2, 3, 1 ],
      vec![ 3, 1, 4, 2 ],
      vec![ 2, 4, 1, 3 ],
      vec![ 1, 3, 2, 4 ],
    ])
  );
  assert!(result.steps.len() >= empty_cells);
  insta::assert_yaml_snapshot!(result.steps);
}

#[test]
fn check_topbot_4x4_no_solution() {
  let grid_size = 4;
  let fixed_numbers = vec![
    FixedNumber::new(0, 0, 1), FixedNumber::new(1, 2, 2),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers).with_top_bottom();
  let mut solver = Solver::new(constraints, None);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::None);
}
