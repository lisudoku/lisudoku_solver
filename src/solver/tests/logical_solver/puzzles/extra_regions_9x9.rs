use crate::{types::{FixedNumber, SudokuConstraints, SolutionType, CellPosition}, solver::Solver};

// https://logicmastersindia.com/live/?contest=SM202301
#[test]
fn check_extra_regions_9x9_1_solve() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(0, 0, 1),
    FixedNumber::new(0, 2, 8),
    FixedNumber::new(0, 3, 3),
    FixedNumber::new(0, 5, 2),
    FixedNumber::new(0, 6, 5),
    FixedNumber::new(0, 8, 6),
    FixedNumber::new(2, 2, 3),
    FixedNumber::new(2, 6, 8),
    FixedNumber::new(4, 0, 4),
    FixedNumber::new(4, 2, 2),
    FixedNumber::new(4, 4, 5),
    FixedNumber::new(4, 6, 7),
    FixedNumber::new(4, 8, 8),
    FixedNumber::new(6, 2, 9),
    FixedNumber::new(6, 6, 4),
    FixedNumber::new(8, 0, 7),
    FixedNumber::new(8, 2, 5),
    FixedNumber::new(8, 3, 4),
    FixedNumber::new(8, 5, 9),
    FixedNumber::new(8, 6, 6),
    FixedNumber::new(8, 8, 2),
  ];
  let empty_cells = grid_size * grid_size - fixed_numbers.len();
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.extra_regions = vec![
    vec![
      CellPosition::new(1, 0), CellPosition::new(1, 1), CellPosition::new(1, 2),
      CellPosition::new(1, 3), CellPosition::new(2, 0), CellPosition::new(2, 3),
      CellPosition::new(3, 0), CellPosition::new(3, 1), CellPosition::new(3, 2),
    ],
    vec![
      CellPosition::new(1, 5), CellPosition::new(1, 6), CellPosition::new(1, 7),
      CellPosition::new(1, 8), CellPosition::new(2, 5), CellPosition::new(2, 8),
      CellPosition::new(3, 6), CellPosition::new(3, 7), CellPosition::new(3, 8),
    ],
    vec![
      CellPosition::new(5, 0), CellPosition::new(5, 1), CellPosition::new(5, 2),
      CellPosition::new(6, 0), CellPosition::new(6, 3), CellPosition::new(7, 0),
      CellPosition::new(7, 1), CellPosition::new(7, 2), CellPosition::new(7, 3),
    ],
    vec![
      CellPosition::new(5, 6), CellPosition::new(5, 7), CellPosition::new(5, 8),
      CellPosition::new(6, 5), CellPosition::new(6, 8), CellPosition::new(7, 5),
      CellPosition::new(7, 6), CellPosition::new(7, 7), CellPosition::new(7, 8),
    ],
  ];
  let mut solver = Solver::new(constraints, None);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(result.solution.unwrap(), vec![
    vec![ 1, 7, 8, 3, 9, 2, 5, 4, 6 ],
    vec![ 9, 4, 6, 5, 1, 8, 2, 7, 3 ],
    vec![ 2, 5, 3, 7, 6, 4, 8, 9, 1 ],
    vec![ 8, 3, 1, 2, 4, 7, 9, 6, 5 ],
    vec![ 4, 6, 2, 9, 5, 1, 7, 3, 8 ],
    vec![ 5, 9, 7, 6, 8, 3, 1, 2, 4 ],
    vec![ 3, 1, 9, 8, 2, 6, 4, 5, 7 ],
    vec![ 6, 2, 4, 1, 7, 5, 3, 8, 9 ],
    vec![ 7, 8, 5, 4, 3, 9, 6, 1, 2 ],
  ]);
  assert!(result.steps.len() >= empty_cells);
}
