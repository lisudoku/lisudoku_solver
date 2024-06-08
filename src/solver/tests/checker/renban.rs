use crate::{solver::Solver, types::{Area, CellPosition, InvalidStateReason, InvalidStateType, SudokuConstraints, SudokuGrid}};

#[test]
fn check_renban_correct() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.renbans = vec![
    vec![CellPosition::new(1, 2), CellPosition::new(0, 2), CellPosition::new(0, 3)],
  ];
  let grid = SudokuGrid {
    values: vec![
      vec![ 1, 2, 4, 3 ],
      vec![ 3, 4, 2, 1 ],
      vec![ 4, 3, 1, 2 ],
      vec![ 2, 1, 3, 4 ],
    ]
  };
  let solver = Solver::new(constraints, Some(grid));
  let solved = solver.check_solved();
  assert_eq!(solved, (true, None));
}

#[test]
fn check_renban_wrong_value_conflict() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.renbans = vec![
    vec![CellPosition::new(1, 3), CellPosition::new(2, 2)],
  ];
  let grid = SudokuGrid {
    values: vec![
      vec![ 1, 2, 4, 3 ],
      vec![ 3, 4, 2, 1 ],
      vec![ 4, 3, 1, 2 ],
      vec![ 2, 1, 3, 4 ],
    ]
  };
  let solver = Solver::new(constraints, Some(grid));
  let solved = solver.check_solved();
  assert_eq!(
    solved,
    (
      false,
      Some(InvalidStateReason {
        state_type: InvalidStateType::AreaValueConflict,
        area: Area::Renban(0),
        values: vec![1],
      }),
    )
  );
}

#[test]
fn check_renban_wrong_value_conflict_2() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.renbans = vec![
    vec![
      CellPosition::new(0, 0), CellPosition::new(0, 1),
      CellPosition::new(1, 2), CellPosition::new(2, 2),
    ],
  ];
  let grid = SudokuGrid {
    values: vec![
      vec![ 2, 1, 4, 3 ],
      vec![ 3, 4, 1, 2 ],
      vec![ 1, 2, 3, 4 ],
      vec![ 4, 3, 2, 1 ],
    ]
  };
  let solver = Solver::new(constraints, Some(grid));
  let solved = solver.check_solved();
  assert_eq!(
    solved,
    (
      false,
      Some(InvalidStateReason {
        state_type: InvalidStateType::AreaValueConflict,
        area: Area::Renban(0),
        values: vec![1],
      }),
    )
  );
}

#[test]
fn check_renban_wrong_not_consecutive() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.renbans = vec![
    vec![CellPosition::new(0, 0), CellPosition::new(1, 0), CellPosition::new(1, 1)],
  ];
  let grid = SudokuGrid {
    values: vec![
      vec![ 1, 2, 4, 3 ],
      vec![ 3, 4, 2, 1 ],
      vec![ 4, 3, 1, 2 ],
      vec![ 2, 1, 3, 4 ],
    ]
  };
  let solver = Solver::new(constraints, Some(grid));
  let solved = solver.check_solved();
  assert_eq!(
    solved,
    (
      false,
      Some(InvalidStateReason {
        state_type: InvalidStateType::AreaConstraint,
        area: Area::Renban(0),
        values: vec![],
      }),
    )
  );
}

#[test]
fn check_renban_wrong_partial_not_consecutive() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.renbans = vec![
    vec![CellPosition::new(0, 2), CellPosition::new(1, 3), CellPosition::new(0, 1)],
  ];
  let grid = SudokuGrid {
    values: vec![
      vec![ 0, 0, 4, 3 ],
      vec![ 3, 4, 2, 1 ],
      vec![ 4, 3, 1, 2 ],
      vec![ 2, 1, 3, 4 ],
    ]
  };
  let solver = Solver::new(constraints, Some(grid));
  let partially_solved = solver.check_partially_solved();
  assert_eq!(
    partially_solved,
    (
      false,
      Some(InvalidStateReason {
        state_type: InvalidStateType::AreaConstraint,
        area: Area::Renban(0),
        values: vec![],
      }),
    )
  );
}

#[test]
fn check_renban_correct_partial_with_fixed_value() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.renbans = vec![
    vec![
      CellPosition::new(1, 1), CellPosition::new(2, 1),
      CellPosition::new(3, 1), CellPosition::new(3, 0),
    ],
  ];
  let grid = SudokuGrid {
    values: vec![
      vec![ 1, 2, 4, 3 ],
      vec![ 3, 4, 2, 1 ],
      vec![ 4, 0, 1, 2 ],
      vec![ 0, 0, 3, 4 ],
    ]
  };
  let solver = Solver::new(constraints, Some(grid));
  let partially_solved = solver.check_partially_solved();
  assert_eq!(
    partially_solved,
    (
      true,
      None,
    )
  );
}

#[test]
fn check_renban_correct_partial_with_no_value() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.renbans = vec![
    vec![
      CellPosition::new(1, 1), CellPosition::new(2, 1),
      CellPosition::new(3, 1), CellPosition::new(3, 0),
    ],
  ];
  let grid = SudokuGrid {
    values: vec![
      vec![ 1, 2, 4, 3 ],
      vec![ 3, 0, 2, 1 ],
      vec![ 4, 0, 1, 2 ],
      vec![ 0, 0, 3, 4 ],
    ]
  };
  let solver = Solver::new(constraints, Some(grid));
  let partially_solved = solver.check_partially_solved();
  assert_eq!(
    partially_solved,
    (
      true,
      None,
    )
  );
}
