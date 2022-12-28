use crate::{types::{FixedNumber, SudokuConstraints, SolutionType}, solver::Solver};

#[test]
fn check_classic_6x6_1_solve() {
  let grid_size = 6;
  let fixed_numbers = vec![
    FixedNumber::new(0, 0, 6),
    FixedNumber::new(1, 0, 1),
    FixedNumber::new(1, 1, 4),
    FixedNumber::new(2, 1, 1),
    FixedNumber::new(2, 2, 2),
    FixedNumber::new(2, 3, 5),
    FixedNumber::new(2, 5, 6),
    FixedNumber::new(3, 0, 5),
    FixedNumber::new(3, 2, 6),
    FixedNumber::new(3, 3, 2),
    FixedNumber::new(3, 4, 1),
    FixedNumber::new(4, 4, 2),
    FixedNumber::new(4, 5, 1),
    FixedNumber::new(5, 5, 3),
  ];
  let empty_cells = grid_size * grid_size - fixed_numbers.len();
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);
  let result = solver.intuitive_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(result.solution.unwrap(), vec![
    vec![ 6, 2, 3, 1, 4, 5 ],
    vec![ 1, 4, 5, 3, 6, 2 ],
    vec![ 4, 1, 2, 5, 3, 6 ],
    vec![ 5, 3, 6, 2, 1, 4 ],
    vec![ 3, 5, 4, 6, 2, 1 ],
    vec![ 2, 6, 1, 4, 5, 3 ],
  ]);
  assert_eq!(result.steps.len(), empty_cells);
}
