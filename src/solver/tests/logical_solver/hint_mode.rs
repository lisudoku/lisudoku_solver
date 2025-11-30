use crate::{solver::Solver, types::{CellPosition, FixedNumber, KropkiDot, Palindrome, Rule, SolutionType, SudokuConstraints, Thermo}};

#[test]
fn check_hint_mode_kropki() {
  let grid_size = 4;
  let mut constraints = SudokuConstraints::new(grid_size, vec![]);
  constraints.kropki_dots = vec![
    KropkiDot::consecutive(CellPosition::new(0, 0), CellPosition::new(0, 1)),
    KropkiDot::consecutive(CellPosition::new(0, 0), CellPosition::new(1, 0)),
    KropkiDot::consecutive(CellPosition::new(3, 2), CellPosition::new(3, 3)),
    KropkiDot::double(CellPosition::new(1, 1), CellPosition::new(1, 2)),
    KropkiDot::double(CellPosition::new(1, 2), CellPosition::new(1, 3)),
    KropkiDot::double(CellPosition::new(2, 0), CellPosition::new(2, 1)),
    KropkiDot::double(CellPosition::new(2, 1), CellPosition::new(2, 2)),
  ];

  let mut solver = Solver::new(constraints, None).with_hint_mode(true);
  let result = solver.logical_solve();

  assert_eq!(result.solution_type, SolutionType::Partial);
  // at most 5 steps (1 + 3 kropki + single)
  assert!(result.steps.len() <= 5);

  let last_step = result.steps.last().unwrap();
  assert!(last_step.is_grid_step());
  let cell = last_step.cells[0];

  let previous_step = &result.steps[result.steps.len() - 2];
  assert!(previous_step.affected_cells.contains(&cell));
}

#[test]
fn check_hint_mode_palindrome() {
  let grid_size = 4;
  let fixed_numbers = vec![ FixedNumber::new(1, 2, 3) ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.palindromes = vec![
    Palindrome(vec![
      CellPosition::new(1, 2), CellPosition::new(2, 2),
      CellPosition::new(3, 1), CellPosition::new(2, 0),
    ]),
  ];

  let mut solver = Solver::new(constraints, None).with_hint_mode(true);
  let result = solver.logical_solve();

  assert_eq!(result.solution_type, SolutionType::Partial);
  assert_eq!(result.steps.len(), 1); // because palindrome values is a grid step
  let last_step = result.steps.last().unwrap();
  assert_eq!(last_step.rule, Rule::PalindromeValues);
  assert_eq!(last_step.cells[0], CellPosition::new(2, 0));
}

// https://github.com/lisudoku/lisudoku_solver/issues/68
// Same as check_9x9_thermo_hard_solve but 2 given digits and in hint mode
#[test]
fn check_hint_mode_nishio() {
  let grid_size = 9;
  let fixed_numbers = vec![
    FixedNumber::new(1, 2, 1), FixedNumber::new(2, 4, 1),
  ];
  let mut constraints = SudokuConstraints::new(grid_size, fixed_numbers);
  constraints.thermos = vec![
    Thermo(vec![
      CellPosition::new(0, 6), CellPosition::new(1, 5), CellPosition::new(1, 4),
      CellPosition::new(2, 3), CellPosition::new(3, 3),
    ]),
    Thermo(vec![
      CellPosition::new(1, 2), CellPosition::new(0, 2), CellPosition::new(0, 1),
      CellPosition::new(0, 0), CellPosition::new(1, 0), CellPosition::new(2, 0),
    ]),
    Thermo(vec![
      CellPosition::new(1, 2), CellPosition::new(0, 2), CellPosition::new(0, 3),
      CellPosition::new(0, 4), CellPosition::new(0, 5),
    ]),
    Thermo(vec![
      CellPosition::new(4, 3), CellPosition::new(4, 4), CellPosition::new(3, 4),
      CellPosition::new(3, 5), CellPosition::new(2, 5), CellPosition::new(2, 6),
      CellPosition::new(1, 6), CellPosition::new(1, 7),
    ]),
    Thermo(vec![
      CellPosition::new(5, 0), CellPosition::new(5, 1), CellPosition::new(4, 2),
      CellPosition::new(3, 2), CellPosition::new(2, 2), CellPosition::new(1, 1),
    ]),
    Thermo(vec![
      CellPosition::new(6, 5), CellPosition::new(5, 6),
    ]),
    Thermo(vec![
      CellPosition::new(7, 7), CellPosition::new(6, 7), CellPosition::new(5, 7),
      CellPosition::new(4, 7), CellPosition::new(3, 7),
    ]),
    Thermo(vec![
      CellPosition::new(8, 1), CellPosition::new(7, 2), CellPosition::new(7, 3),
      CellPosition::new(6, 3), CellPosition::new(6, 4), CellPosition::new(5, 4),
      CellPosition::new(5, 5), CellPosition::new(4, 5),
    ]),
    Thermo(vec![
      CellPosition::new(8, 5), CellPosition::new(8, 6), CellPosition::new(8, 7),
      CellPosition::new(8, 8), CellPosition::new(7, 8), CellPosition::new(6, 8),
      CellPosition::new(5, 8),
    ]),
  ];

  let mut solver = Solver::new(constraints, None).with_hint_mode(true);
  let result = solver.logical_solve();

  assert_eq!(result.solution_type, SolutionType::Partial);
  assert!(result.steps.len() >= 10); // ballpark check

  let last_step = result.steps.last().unwrap();
  assert!(last_step.is_grid_step());
}
