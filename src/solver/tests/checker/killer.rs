use crate::{types::{SudokuConstraints, SudokuGrid, CellPosition, KillerCage}, solver::Solver};

#[test]
fn check_killer_low_sum() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.killer_cages = vec![
    KillerCage {
      sum: Some(10),
      region: vec![
        CellPosition::new(0, 0),
        CellPosition::new(1, 0),
        CellPosition::new(2, 0),
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
fn check_killer_high_sum() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.killer_cages = vec![
    KillerCage {
      sum: Some(9),
      region: vec![
        CellPosition::new(0, 0),
        CellPosition::new(1, 0),
        CellPosition::new(1, 1),
        CellPosition::new(1, 2),
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
fn check_killer_unique_digits() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.killer_cages = vec![
    KillerCage {
      sum: Some(9),
      region: vec![
        CellPosition::new(0, 0),
        CellPosition::new(1, 0),
        CellPosition::new(1, 1),
        CellPosition::new(2, 1),
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
fn check_killer_correct() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.killer_cages = vec![
    KillerCage {
      sum: Some(10),
      region: vec![
        CellPosition::new(1, 1),
        CellPosition::new(1, 2),
        CellPosition::new(2, 1),
        CellPosition::new(2, 2),
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
fn check_killer_partially_correct() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.killer_cages = vec![
    KillerCage {
      sum: Some(10),
      region: vec![
        CellPosition::new(1, 1),
        CellPosition::new(1, 2),
        CellPosition::new(2, 1),
        CellPosition::new(2, 2),
      ],
    },
  ];
  let grid = SudokuGrid {
    values: vec![
      vec![ 2, 1, 4, 3 ],
      vec![ 3, 4, 1, 2 ],
      vec![ 1, 2, 0, 4 ],
      vec![ 4, 3, 2, 1 ],
    ]
  };
  let solver = Solver::new(constraints, Some(grid));
  let solved = solver.check_partially_solved();
  assert_eq!(solved, true);
}

#[test]
fn check_killer_partially_correct_exact_sum() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.killer_cages = vec![
    KillerCage {
      sum: Some(7),
      region: vec![
        CellPosition::new(1, 1),
        CellPosition::new(1, 2),
        CellPosition::new(2, 1),
        CellPosition::new(2, 2),
      ],
    },
  ];
  let grid = SudokuGrid {
    values: vec![
      vec![ 2, 1, 4, 3 ],
      vec![ 3, 4, 1, 2 ],
      vec![ 1, 2, 0, 4 ],
      vec![ 4, 3, 2, 1 ],
    ]
  };
  let solver = Solver::new(constraints, Some(grid));
  let solved = solver.check_partially_solved();
  assert_eq!(solved, true);
}

#[test]
fn check_killer_partially_incorrect_high_sum() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.killer_cages = vec![
    KillerCage {
      sum: Some(6),
      region: vec![
        CellPosition::new(1, 1),
        CellPosition::new(1, 2),
        CellPosition::new(2, 1),
        CellPosition::new(2, 2),
      ],
    },
  ];
  let grid = SudokuGrid {
    values: vec![
      vec![ 2, 1, 4, 3 ],
      vec![ 3, 4, 1, 2 ],
      vec![ 1, 2, 0, 4 ],
      vec![ 4, 3, 2, 1 ],
    ]
  };
  let solver = Solver::new(constraints, Some(grid));
  let solved = solver.check_partially_solved();
  assert_eq!(solved, false);
}
