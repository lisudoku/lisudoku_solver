use crate::{types::{SudokuConstraints, FixedNumber, CellPosition, Rule}, solver::{Solver, logical_solver::{candidates::Candidates, technique::Technique, empty_reclanges::EmptyRectangles}}};

#[test]
fn check_empty_rectangles_with_affected_cells_1() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(1, 1, 2), FixedNumber::new(2, 3, 1), FixedNumber::new(3, 2, 1),
    FixedNumber::new(1, 6, 7), FixedNumber::new(0, 8, 8), FixedNumber::new(4, 7, 1),
    FixedNumber::new(7, 6, 8), FixedNumber::new(8, 6, 9),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = EmptyRectangles.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.rule, Rule::EmptyRectangles);
  assert_eq!(step.cells, vec![ CellPosition::new(0, 6), CellPosition::new(6, 6) ]);
  assert_eq!(step.values, vec![ 1 ]);
  assert_eq!(solver.candidates[6][0].len(), 9);
  assert!(solver.candidates[6][0].contains(&1));

  solver.apply_rule(&mut step);
  assert_eq!(solver.candidates[6][0].len(), 8);
  assert!(!solver.candidates[6][0].contains(&1));
}

#[test]
fn check_empty_rectangles_with_affected_cells_2() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(1, 1, 2), FixedNumber::new(2, 3, 1), FixedNumber::new(3, 2, 1),
    FixedNumber::new(6, 1, 7), FixedNumber::new(8, 0, 8), FixedNumber::new(7, 4, 1),
    FixedNumber::new(6, 7, 8), FixedNumber::new(6, 8, 9),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = EmptyRectangles.run(&solver);
  assert!(!steps.is_empty());
  let mut step = steps.first().unwrap();
  assert_eq!(step.rule, Rule::EmptyRectangles);
  assert_eq!(step.cells, vec![ CellPosition::new(6, 0), CellPosition::new(6, 6) ]);
  assert_eq!(step.values, vec![ 1 ]);
  assert_eq!(solver.candidates[0][6].len(), 9);
  assert!(solver.candidates[0][6].contains(&1));

  solver.apply_rule(&mut step);
  assert_eq!(solver.candidates[0][6].len(), 8);
  assert!(!solver.candidates[0][6].contains(&1));
}

#[test]
fn check_empty_rectangles_with_no_affected_cells_1() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(1, 1, 2), FixedNumber::new(2, 3, 1), FixedNumber::new(3, 2, 1),
    FixedNumber::new(4, 7, 1), FixedNumber::new(6, 8, 1),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = EmptyRectangles.run(&solver);
  assert!(steps.is_empty());
}

#[test]
fn check_empty_rectangles_with_no_affected_cells_2() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(1, 1, 2), FixedNumber::new(2, 3, 1), FixedNumber::new(3, 2, 1),
    FixedNumber::new(7, 5, 1), FixedNumber::new(8, 6, 1),
  ];
  let constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  let mut solver = Solver::new(constraints, None);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = EmptyRectangles.run(&solver);
  assert!(steps.is_empty());
}
