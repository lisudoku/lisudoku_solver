use crate::{types::{SudokuConstraints, SudokuGrid, CellPosition, Arrow}, solver::Solver};

#[test]
fn check_wrong_arrow() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.arrows = vec![
    Arrow {
      circle_cells: vec![ CellPosition::new(1, 0) ],
      arrow_cells: vec![
        CellPosition::new(0, 0), CellPosition::new(1, 1),
      ],
    },
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
  assert_eq!(solved, false);
}

#[test]
fn check_correct_arrow() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.arrows = vec![
    Arrow {
      circle_cells: vec![ CellPosition::new(1, 0) ],
      arrow_cells: vec![
        CellPosition::new(0, 0), CellPosition::new(0, 1),
      ],
    },
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
  assert_eq!(solved, true);
}

#[test]
fn check_wrong_oval_arrow() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.arrows = vec![
    Arrow {
      circle_cells: vec![
        CellPosition::new(1, 2), CellPosition::new(1, 3),
      ],
      arrow_cells: vec![
        CellPosition::new(2, 3), CellPosition::new(2, 2),
        CellPosition::new(2, 1), CellPosition::new(3, 0),
      ],
    },
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
  assert_eq!(solved, false);
}

#[test]
fn check_correct_oval_arrow() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.arrows = vec![
    Arrow {
      circle_cells: vec![
        CellPosition::new(1, 2), CellPosition::new(1, 3),
      ],
      arrow_cells: vec![
        CellPosition::new(2, 3), CellPosition::new(2, 2),
        CellPosition::new(2, 1), CellPosition::new(3, 1),
      ],
    },
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
  assert_eq!(solved, true);
}

#[test]
fn check_partial_arrow() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.arrows = vec![
    Arrow {
      circle_cells: vec![ CellPosition::new(3, 0) ],
      arrow_cells: vec![
        CellPosition::new(2, 1), CellPosition::new(1, 2), CellPosition::new(0, 1),
      ],
    },
  ];
  let grid = SudokuGrid {
    values: vec![
      vec![ 0, 1, 0, 3 ],
      vec![ 0, 0, 0, 0 ],
      vec![ 1, 0, 0, 0 ],
      vec![ 0, 0, 0, 0 ],
    ]
  };
  let solver = Solver::new(constraints, Some(grid));
  let solved = solver.check_partially_solved();
  assert_eq!(solved, true);
}
