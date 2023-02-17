use crate::{types::{SudokuConstraints, CellPosition, SolutionType, KropkiDot, FixedNumber}, solver::Solver};

// https://uploads-ssl.webflow.com/62793457876c001d28edf162/6348945a45b06acb414391b7_WSC_2022_IB_v2.1.pdf
#[test]
fn check_kropki_6x6_1_solve() {
  // Added without negative condition, it is not needed
  let grid_size = 6;
  let empty_cells = grid_size * grid_size;
  let mut constraints = SudokuConstraints::new(grid_size, vec![]);
  constraints.kropki_dots = vec![
    KropkiDot::consecutive(CellPosition::new(0, 0), CellPosition::new(0, 1)),
    KropkiDot::consecutive(CellPosition::new(0, 1), CellPosition::new(0, 2)),
    KropkiDot::consecutive(CellPosition::new(0, 2), CellPosition::new(0, 3)),
    KropkiDot::consecutive(CellPosition::new(0, 3), CellPosition::new(0, 4)),
    KropkiDot::consecutive(CellPosition::new(0, 4), CellPosition::new(0, 5)),
    KropkiDot::consecutive(CellPosition::new(0, 2), CellPosition::new(1, 2)),
    KropkiDot::consecutive(CellPosition::new(1, 4), CellPosition::new(1, 5)),
    KropkiDot::consecutive(CellPosition::new(1, 4), CellPosition::new(2, 4)),
    KropkiDot::double(CellPosition::new(0, 3), CellPosition::new(1, 3)),
    KropkiDot::double(CellPosition::new(0, 4), CellPosition::new(1, 4)),
    KropkiDot::double(CellPosition::new(1, 0), CellPosition::new(1, 1)),
    KropkiDot::double(CellPosition::new(1, 0), CellPosition::new(2, 0)),
    KropkiDot::double(CellPosition::new(1, 2), CellPosition::new(1, 3)),
    KropkiDot::consecutive(CellPosition::new(2, 1), CellPosition::new(2, 2)),
    KropkiDot::consecutive(CellPosition::new(2, 4), CellPosition::new(2, 5)),
    KropkiDot::consecutive(CellPosition::new(3, 1), CellPosition::new(3, 2)),
    KropkiDot::consecutive(CellPosition::new(3, 1), CellPosition::new(4, 1)),
    KropkiDot::consecutive(CellPosition::new(3, 3), CellPosition::new(3, 4)),
    KropkiDot::consecutive(CellPosition::new(3, 3), CellPosition::new(4, 3)),
    KropkiDot::consecutive(CellPosition::new(3, 4), CellPosition::new(3, 5)),
    KropkiDot::double(CellPosition::new(2, 1), CellPosition::new(3, 1)),
    KropkiDot::double(CellPosition::new(3, 2), CellPosition::new(3, 3)),
    KropkiDot::consecutive(CellPosition::new(4, 0), CellPosition::new(4, 1)),
    KropkiDot::consecutive(CellPosition::new(4, 2), CellPosition::new(4, 3)),
    KropkiDot::consecutive(CellPosition::new(4, 5), CellPosition::new(5, 5)),
    KropkiDot::consecutive(CellPosition::new(5, 0), CellPosition::new(5, 1)),
    KropkiDot::double(CellPosition::new(4, 1), CellPosition::new(5, 1)),
    KropkiDot::double(CellPosition::new(5, 2), CellPosition::new(5, 3)),
    KropkiDot::double(CellPosition::new(5, 4), CellPosition::new(5, 5)),
  ];

  let mut solver = Solver::new(constraints, None);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(result.solution.unwrap(), vec![
    vec![ 6, 5, 4, 3, 2, 1 ],
    vec![ 2, 1, 3, 6, 4, 5 ],
    vec![ 4, 6, 5, 1, 3, 2 ],
    vec![ 1, 3, 2, 4, 5, 6 ],
    vec![ 3, 2, 6, 5, 1, 4 ],
    vec![ 5, 4, 1, 2, 6, 3 ],
  ]);
  assert!(result.steps.len() >= empty_cells);
}

#[test]
fn check_kropki_6x6_negative_condition_solve() {
  let grid_size = 6;
  let fixed_numbers = vec![ FixedNumber::new(0, 0, 6) ];
  let empty_cells = grid_size * grid_size - fixed_numbers.len();
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.kropki_dots = vec![
    KropkiDot::consecutive(CellPosition::new(0, 1), CellPosition::new(0, 2)),
    KropkiDot::double(CellPosition::new(0, 1), CellPosition::new(1, 1)),
    KropkiDot::consecutive(CellPosition::new(0, 4), CellPosition::new(0, 5)),
    KropkiDot::consecutive(CellPosition::new(1, 1), CellPosition::new(1, 2)),
    KropkiDot::double(CellPosition::new(1, 3), CellPosition::new(1, 4)),
    KropkiDot::double(CellPosition::new(1, 4), CellPosition::new(2, 4)),
    KropkiDot::consecutive(CellPosition::new(2, 0), CellPosition::new(3, 0)),
    KropkiDot::consecutive(CellPosition::new(2, 1), CellPosition::new(2, 2)),
    KropkiDot::double(CellPosition::new(2, 4), CellPosition::new(2, 5)),
    KropkiDot::double(CellPosition::new(3, 1), CellPosition::new(3, 2)),
    KropkiDot::consecutive(CellPosition::new(3, 3), CellPosition::new(3, 4)),
    KropkiDot::consecutive(CellPosition::new(3, 4), CellPosition::new(4, 4)),
    KropkiDot::consecutive(CellPosition::new(4, 0), CellPosition::new(5, 0)),
    KropkiDot::consecutive(CellPosition::new(4, 1), CellPosition::new(4, 2)),
    KropkiDot::consecutive(CellPosition::new(4, 1), CellPosition::new(5, 1)),
    KropkiDot::consecutive(CellPosition::new(4, 4), CellPosition::new(4, 5)),
    KropkiDot::consecutive(CellPosition::new(5, 3), CellPosition::new(5, 4)),
  ];
  constraints.kropki_negative = true;

  let mut solver = Solver::new(constraints, None);
  assert_eq!(solver.constraints.kropki_dots.len(), (grid_size - 1) * grid_size * 2);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(result.solution.unwrap(), vec![
    vec![ 6, 2, 3, 1, 4, 5 ],
    vec![ 1, 4, 5, 3, 6, 2 ],
    vec![ 4, 1, 2, 5, 3, 6 ],
    vec![ 5, 3, 6, 2, 1, 4 ],
    vec![ 3, 5, 4, 6, 2, 1 ],
    vec![ 2, 6, 1, 4, 5, 3 ],
  ]);
  assert!(result.steps.len() >= empty_cells);
}
