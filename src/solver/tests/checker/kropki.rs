use crate::{solver::{Solver, checker::SolvedState}, types::{Area, CellPosition, Grid, InvalidStateReason, InvalidStateType, KropkiDot, SudokuConstraints}};

#[test]
fn check_kropki_fully_correct() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.kropki_dots = vec![
    KropkiDot::consecutive(CellPosition::new(0, 0), CellPosition::new(0, 1)),
    KropkiDot::consecutive(CellPosition::new(0, 0), CellPosition::new(1, 0)),
    KropkiDot::consecutive(CellPosition::new(0, 2), CellPosition::new(0, 3)),
    KropkiDot::consecutive(CellPosition::new(1, 0), CellPosition::new(1, 1)),
    KropkiDot::consecutive(CellPosition::new(1, 2), CellPosition::new(1, 3)),
    KropkiDot::consecutive(CellPosition::new(2, 0), CellPosition::new(2, 1)),
    KropkiDot::consecutive(CellPosition::new(2, 1), CellPosition::new(3, 1)),
    KropkiDot::consecutive(CellPosition::new(2, 1), CellPosition::new(2, 2)),
    KropkiDot::consecutive(CellPosition::new(2, 2), CellPosition::new(2, 3)),
    KropkiDot::consecutive(CellPosition::new(2, 2), CellPosition::new(3, 2)),
    KropkiDot::consecutive(CellPosition::new(3, 0), CellPosition::new(3, 1)),
    KropkiDot::consecutive(CellPosition::new(3, 1), CellPosition::new(3, 2)),
    KropkiDot::double(CellPosition::new(1, 1), CellPosition::new(2, 1)),
    KropkiDot::double(CellPosition::new(1, 3), CellPosition::new(2, 3)),
    KropkiDot::double(CellPosition::new(3, 2), CellPosition::new(3, 3)),
  ];
  let grid = Grid(vec![
    vec![ 2, 1, 4, 3 ],
    vec![ 3, 4, 1, 2 ],
    vec![ 1, 2, 3, 4 ],
    vec![ 4, 3, 2, 1 ],
  ]);
  let solver = Solver::new(constraints, Some(grid));
  let solved = solver.check_solved();
  assert_eq!(solved, SolvedState::solved());
}

#[test]
fn check_kropki_partially_correct() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.kropki_dots = vec![
    KropkiDot::consecutive(CellPosition::new(0, 0), CellPosition::new(0, 1)),
    KropkiDot::consecutive(CellPosition::new(0, 0), CellPosition::new(1, 0)),
    KropkiDot::consecutive(CellPosition::new(0, 2), CellPosition::new(0, 3)),
    KropkiDot::consecutive(CellPosition::new(1, 0), CellPosition::new(1, 1)),
    KropkiDot::consecutive(CellPosition::new(1, 2), CellPosition::new(1, 3)),
    KropkiDot::consecutive(CellPosition::new(2, 0), CellPosition::new(2, 1)),
    KropkiDot::consecutive(CellPosition::new(2, 1), CellPosition::new(3, 1)),
    KropkiDot::consecutive(CellPosition::new(2, 1), CellPosition::new(2, 2)),
    KropkiDot::consecutive(CellPosition::new(2, 2), CellPosition::new(2, 3)),
    KropkiDot::consecutive(CellPosition::new(2, 2), CellPosition::new(3, 2)),
    KropkiDot::consecutive(CellPosition::new(3, 0), CellPosition::new(3, 1)),
    KropkiDot::consecutive(CellPosition::new(3, 1), CellPosition::new(3, 2)),
    KropkiDot::double(CellPosition::new(1, 1), CellPosition::new(2, 1)),
    KropkiDot::double(CellPosition::new(1, 3), CellPosition::new(2, 3)),
    KropkiDot::double(CellPosition::new(3, 2), CellPosition::new(3, 3)),
  ];
  let grid = Grid(vec![
    vec![ 2, 0, 4, 0 ],
    vec![ 3, 4, 1, 2 ],
    vec![ 1, 0, 0, 4 ],
    vec![ 4, 3, 2, 1 ],
  ]);
  let solver = Solver::new(constraints, Some(grid));
  let solved = solver.check_partially_solved();
  assert_eq!(solved, SolvedState::solved());
}

#[test]
fn check_kropki_consecutive_incorrect() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.kropki_dots = vec![
    KropkiDot::consecutive(CellPosition::new(0, 1), CellPosition::new(0, 2)),
  ];
  let grid = Grid(vec![
    vec![ 2, 1, 4, 3 ],
    vec![ 3, 4, 1, 2 ],
    vec![ 1, 2, 3, 4 ],
    vec![ 4, 3, 2, 1 ],
  ]);
  let solver = Solver::new(constraints, Some(grid));
  let solved = solver.check_solved();
  assert_eq!(
    solved,
    SolvedState::unsolved(
      InvalidStateReason {
        state_type: InvalidStateType::AreaConstraint,
        area: Area::KropkiDot(0),
        values: vec![],
      }
    )
  );
}

#[test]
fn check_kropki_double_incorrect() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.kropki_dots = vec![
    KropkiDot::double(CellPosition::new(0, 1), CellPosition::new(0, 2)),
  ];
  let grid = Grid(vec![
    vec![ 2, 1, 4, 3 ],
    vec![ 3, 4, 1, 2 ],
    vec![ 1, 2, 3, 4 ],
    vec![ 4, 3, 2, 1 ],
  ]);
  let solver = Solver::new(constraints, Some(grid));
  let solved = solver.check_solved();
  assert_eq!(
    solved,
    SolvedState::unsolved(
      InvalidStateReason {
        state_type: InvalidStateType::AreaConstraint,
        area: Area::KropkiDot(0),
        values: vec![],
      }
    )
  );
}

#[test]
fn check_kropki_negative_condition_incorrect() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.kropki_dots = vec![
    KropkiDot::consecutive(CellPosition::new(0, 0), CellPosition::new(0, 1)),
  ];
  constraints.kropki_negative = true;
  let grid = Grid(vec![
    vec![ 2, 1, 4, 3 ],
    vec![ 3, 4, 1, 2 ],
    vec![ 1, 2, 3, 4 ],
    vec![ 4, 3, 2, 1 ],
  ]);
  let solver = Solver::new(constraints, Some(grid));
  let solved = solver.check_solved();
  assert_eq!(
    solved,
    SolvedState::unsolved(
      InvalidStateReason {
        state_type: InvalidStateType::AreaConstraint,
        area: Area::KropkiDot(1),
        values: vec![],
      }
    )
  );
}

#[test]
fn check_kropki_negative_condition_correct() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.kropki_dots = vec![
    KropkiDot::consecutive(CellPosition::new(0, 0), CellPosition::new(0, 1)),
    KropkiDot::consecutive(CellPosition::new(0, 0), CellPosition::new(1, 0)),
    KropkiDot::consecutive(CellPosition::new(0, 2), CellPosition::new(0, 3)),
    KropkiDot::consecutive(CellPosition::new(0, 3), CellPosition::new(1, 3)),
    KropkiDot::consecutive(CellPosition::new(1, 0), CellPosition::new(1, 1)),
    KropkiDot::consecutive(CellPosition::new(1, 2), CellPosition::new(1, 3)),
    KropkiDot::consecutive(CellPosition::new(2, 0), CellPosition::new(2, 1)),
    KropkiDot::consecutive(CellPosition::new(2, 1), CellPosition::new(3, 1)),
    KropkiDot::consecutive(CellPosition::new(2, 1), CellPosition::new(2, 2)),
    KropkiDot::consecutive(CellPosition::new(2, 2), CellPosition::new(2, 3)),
    KropkiDot::consecutive(CellPosition::new(2, 2), CellPosition::new(3, 2)),
    KropkiDot::consecutive(CellPosition::new(3, 0), CellPosition::new(3, 1)),
    KropkiDot::consecutive(CellPosition::new(3, 1), CellPosition::new(3, 2)),
    KropkiDot::double(CellPosition::new(1, 1), CellPosition::new(2, 1)),
    KropkiDot::double(CellPosition::new(1, 3), CellPosition::new(2, 3)),
    KropkiDot::double(CellPosition::new(3, 2), CellPosition::new(3, 3)),
  ];
  constraints.kropki_negative = true;
  let grid = Grid(vec![
    vec![ 2, 1, 4, 3 ],
    vec![ 3, 4, 1, 2 ],
    vec![ 1, 2, 3, 4 ],
    vec![ 4, 3, 2, 1 ],
  ]);
  let solver = Solver::new(constraints, Some(grid));
  let solved = solver.check_solved();
  assert_eq!(solved, SolvedState::solved());
}
