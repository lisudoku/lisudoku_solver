use crate::{types::{FixedNumber, SudokuConstraints, CellPosition, SolutionType, Arrow}, solver::Solver};

#[test]
fn check_arrow_4x4_1_solve() {
  let grid_size = 4;
  let fixed_numbers = vec![
    FixedNumber::new(0, 3, 3), FixedNumber::new(2, 0, 1),
  ];
  let empty_cells = grid_size * grid_size - fixed_numbers.len();
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.arrows = vec![
    Arrow {
      circle_cells: vec![ CellPosition::new(3, 0) ],
      arrow_cells: vec![
        CellPosition::new(2, 1), CellPosition::new(1, 2), CellPosition::new(0, 1),
      ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(3, 1) ],
      arrow_cells: vec![ CellPosition::new(3, 2), CellPosition::new(3, 3) ],
    },
  ];
  let mut solver = Solver::new(constraints, None);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(result.solution.unwrap(), vec![
    vec![ 2, 1, 4, 3 ],
    vec![ 3, 4, 1, 2 ],
    vec![ 1, 2, 3, 4 ],
    vec![ 4, 3, 2, 1 ],
  ]);
  assert!(result.steps.len() >= empty_cells);
}
