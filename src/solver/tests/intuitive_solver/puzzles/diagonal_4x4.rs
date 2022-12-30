use crate::{types::{FixedNumber, SudokuConstraints, SolutionType}, solver::Solver};

#[test]
fn check_diagonal_4x4_1_solve() {
  let grid_size = 4;
  let fixed_numbers = vec![
    FixedNumber::new(0, 0, 2),
    FixedNumber::new(1, 2, 1),
    FixedNumber::new(3, 0, 4),
  ];
  let empty_cells = grid_size * grid_size - fixed_numbers.len();
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.primary_diagonal = true;
  let mut solver = Solver::new(constraints, None);
  let result = solver.intuitive_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(result.solution.unwrap(), vec![
    vec![ 2, 1, 4, 3 ],
    vec![ 3, 4, 1, 2 ],
    vec![ 1, 2, 3, 4 ],
    vec![ 4, 3, 2, 1 ],
  ]);
  assert_eq!(result.steps.len(), empty_cells);
}

#[test]
fn check_diagonal_4x4_2_solve() {
  let grid_size = 4;
  let fixed_numbers = vec![
    FixedNumber::new(0, 0, 2),
    FixedNumber::new(1, 2, 1),
    FixedNumber::new(3, 0, 4),
  ];
  let empty_cells = grid_size * grid_size - fixed_numbers.len();
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.secondary_diagonal = true;
  let mut solver = Solver::new(constraints, None);
  let result = solver.intuitive_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(result.solution.unwrap(), vec![
    vec![ 2, 1, 4, 3 ],
    vec![ 3, 4, 1, 2 ],
    vec![ 1, 2, 3, 4 ],
    vec![ 4, 3, 2, 1 ],
  ]);
  assert_eq!(result.steps.len(), empty_cells);
}
