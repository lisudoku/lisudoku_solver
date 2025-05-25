use crate::{types::{FixedNumber, SudokuConstraints, SolutionType}, solver::Solver};

#[test]
fn check_anti_knight_6x6_1_solve() {
  let grid_size = 6;
  let fixed_numbers = vec![
    FixedNumber::new(0, 1, 5),
    FixedNumber::new(0, 2, 1),
    FixedNumber::new(0, 3, 3),
    FixedNumber::new(0, 4, 6),
    FixedNumber::new(1, 2, 2),
    FixedNumber::new(1, 3, 4),
    FixedNumber::new(5, 0, 2),
    FixedNumber::new(5, 5, 4),
  ];
  let empty_cells = grid_size * grid_size - fixed_numbers.len();
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers).with_anti_knight();
  let mut solver = Solver::new(constraints, None);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(result.solution.unwrap(), vec![
    vec![ 4, 5, 1, 3, 6, 2 ],
    vec![ 3, 6, 2, 4, 5, 1 ],
    vec![ 1, 2, 6, 5, 4, 3 ],
    vec![ 5, 4, 3, 1, 2, 6 ],
    vec![ 6, 3, 4, 2, 1, 5 ],
    vec![ 2, 1, 5, 6, 3, 4 ],
  ]);
  assert_eq!(result.steps.len(), empty_cells);
  insta::assert_yaml_snapshot!(result.steps);
}
