use crate::{types::{SudokuConstraints, CellPosition, SolutionType, KropkiDot, FixedNumber}, solver::Solver};

#[test]
fn check_kropki_4x4_1_solve() {
  let grid_size = 4;
  let fixed_numbers = vec![ FixedNumber::new(3, 0, 4) ];
  let empty_cells = grid_size * grid_size - fixed_numbers.len();
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.kropki_dots = vec![
    KropkiDot::consecutive(CellPosition::new(0, 0), CellPosition::new(0, 1)),
    KropkiDot::consecutive(CellPosition::new(0, 1), CellPosition::new(0, 2)),
    KropkiDot::consecutive(CellPosition::new(0, 2), CellPosition::new(0, 3)),
    KropkiDot::consecutive(CellPosition::new(2, 2), CellPosition::new(2, 3)),
    KropkiDot::consecutive(CellPosition::new(3, 1), CellPosition::new(3, 2)),
  ];

  let mut solver = Solver::new(constraints, None);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(result.solution.unwrap(), vec![
    vec![ 1, 2, 3, 4 ],
    vec![ 3, 4, 1, 2 ],
    vec![ 2, 1, 4, 3 ],
    vec![ 4, 3, 2, 1 ],
  ]);
  assert!(result.steps.len() >= empty_cells);
  insta::assert_yaml_snapshot!(result.steps);
}

#[test]
fn check_kropki_4x4_no_solution_solve() {
  let grid_size = 4;
  let fixed_numbers = vec![
    FixedNumber::new(2, 3, 4),
    FixedNumber::new(3, 0, 4),
  ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.kropki_dots = vec![
    KropkiDot::consecutive(CellPosition::new(0, 0), CellPosition::new(0, 1)),
    KropkiDot::consecutive(CellPosition::new(0, 1), CellPosition::new(0, 2)),
    KropkiDot::consecutive(CellPosition::new(0, 2), CellPosition::new(0, 3)),
  ];

  let mut solver = Solver::new(constraints, None);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::None);
}
