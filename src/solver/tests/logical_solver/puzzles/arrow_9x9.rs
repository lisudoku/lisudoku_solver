use crate::{solver::Solver, types::{Arrow, CellPosition, FixedNumber, Grid, SolutionType, SudokuConstraints}};

// https://gp.worldpuzzle.org/sites/default/files/Puzzles/2023/2023_SudokuRound7_IB.pdf puzzle 8
#[test]
fn check_arrow_9x9_1_solve() {
  let grid_size = 9;
  let empty_cells = grid_size * grid_size;
  let mut constraints = SudokuConstraints::new(grid_size, vec![]);
  constraints.arrows = vec![
    Arrow {
      circle_cells: vec![ CellPosition::new(0, 0), CellPosition::new(0, 1) ],
      arrow_cells: vec![ CellPosition::new(0, 2), CellPosition::new(0, 3) ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(0, 5) ],
      arrow_cells: vec![
        CellPosition::new(0, 6), CellPosition::new(0, 7), CellPosition::new(0, 8),
      ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(1, 0), CellPosition::new(2, 0) ],
      arrow_cells: vec![
        CellPosition::new(3, 0), CellPosition::new(4, 0), CellPosition::new(5, 0),
      ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(1, 3) ],
      arrow_cells: vec![
        CellPosition::new(1, 4), CellPosition::new(1, 5), CellPosition::new(1, 6),
      ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(2, 2), CellPosition::new(2, 3) ],
      arrow_cells: vec![
        CellPosition::new(2, 4), CellPosition::new(2, 5), CellPosition::new(2, 6),
        CellPosition::new(2, 7), CellPosition::new(2, 8),
      ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(3, 5) ],
      arrow_cells: vec![ CellPosition::new(4, 6), CellPosition::new(4, 7) ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(4, 4) ],
      arrow_cells: vec![
        CellPosition::new(4, 3), CellPosition::new(4, 2), CellPosition::new(4, 1),
      ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(5, 6) ],
      arrow_cells: vec![
        CellPosition::new(5, 5), CellPosition::new(5, 4), CellPosition::new(5, 3),
      ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(6, 5), CellPosition::new(6, 6) ],
      arrow_cells: vec![
        CellPosition::new(6, 4), CellPosition::new(6, 3), CellPosition::new(6, 2),
        CellPosition::new(6, 1), CellPosition::new(6, 0),
      ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(6, 8), CellPosition::new(7, 8) ],
      arrow_cells: vec![ CellPosition::new(5, 8), CellPosition::new(4, 8) ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(7, 1) ],
      arrow_cells: vec![ CellPosition::new(7, 0), CellPosition::new(8, 0) ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(7, 1) ],
      arrow_cells: vec![ CellPosition::new(7, 2), CellPosition::new(7, 3) ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(8, 8), CellPosition::new(8, 7) ], // reversed
      arrow_cells: vec![
        CellPosition::new(8, 6), CellPosition::new(8, 5), CellPosition::new(8, 4),
      ],
    },
  ];
  let mut solver = Solver::new(constraints, None);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(
    result.solution.unwrap(),
    Grid(vec![
      vec![ 1, 5, 8, 7, 6, 9, 2, 3, 4 ],
      vec![ 2, 9, 7, 8, 3, 4, 1, 5, 6 ],
      vec![ 4, 6, 3, 1, 2, 5, 8, 7, 9 ],
      vec![ 8, 3, 6, 9, 4, 7, 5, 1, 2 ],
      vec![ 9, 1, 5, 2, 8, 6, 3, 4, 7 ],
      vec![ 7, 2, 4, 5, 1, 3, 9, 6, 8 ],
      vec![ 6, 4, 9, 3, 5, 2, 7, 8, 1 ],
      vec![ 3, 8, 2, 6, 7, 1, 4, 9, 5 ],
      vec![ 5, 7, 1, 4, 9, 8, 6, 2, 3 ],
    ])
  );
  assert!(result.steps.len() >= empty_cells);
  insta::assert_yaml_snapshot!(result.steps);
}

// https://www.youtube.com/watch?v=vKEmgKgYg_U
// Reaaally long arrow with 2 circle cells and 12 arrow cells
#[test]
fn check_arrow_9x9_2_solve() {
  let grid_size = 9;
  let empty_cells = grid_size * grid_size;
  let mut constraints = SudokuConstraints::new(grid_size, vec![]);
  constraints.arrows = vec![
    Arrow {
      circle_cells: vec![ CellPosition::new(1, 6) ],
      arrow_cells: vec![ CellPosition::new(0, 6), CellPosition::new(0, 5) ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(1, 6) ],
      arrow_cells: vec![
        CellPosition::new(1, 5), CellPosition::new(1, 4), CellPosition::new(1, 3),
      ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(2, 0), CellPosition::new(2, 1) ],
      arrow_cells: vec![
        CellPosition::new(3, 0), CellPosition::new(3, 1), CellPosition::new(3, 2),
        CellPosition::new(4, 3),
      ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(2, 3), CellPosition::new(2, 4) ],
      arrow_cells: vec![
        CellPosition::new(3, 4), CellPosition::new(3, 3), CellPosition::new(4, 2),
        CellPosition::new(5, 2),
      ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(2, 5) ],
      arrow_cells: vec![
        CellPosition::new(3, 5), CellPosition::new(4, 5), CellPosition::new(5, 5),
      ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(2, 6), CellPosition::new(2, 7) ],
      arrow_cells: vec![ CellPosition::new(2, 8), CellPosition::new(1, 8) ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(5, 6) ],
      arrow_cells: vec![ CellPosition::new(6, 5), CellPosition::new(7, 5) ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(5, 7), CellPosition::new(5, 8) ],
      arrow_cells: vec![
        CellPosition::new(6, 7), CellPosition::new(7, 8), CellPosition::new(8, 8),
        CellPosition::new(8, 7), CellPosition::new(8, 6), CellPosition::new(8, 5),
        CellPosition::new(8, 4), CellPosition::new(8, 3), CellPosition::new(8, 2),
        CellPosition::new(8, 1), CellPosition::new(8, 0), CellPosition::new(7, 0),
      ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(6, 0) ],
      arrow_cells: vec![ CellPosition::new(5, 1), CellPosition::new(4, 1) ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(7, 1) ],
      arrow_cells: vec![
        CellPosition::new(6, 2), CellPosition::new(6, 3), CellPosition::new(6, 4),
      ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(7, 4) ],
      arrow_cells: vec![ CellPosition::new(7, 3), CellPosition::new(7, 2) ],
    },
  ];
  let mut solver = Solver::new(constraints, None);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(
    result.solution.unwrap(),
    Grid(vec![
      vec![ 8, 1, 4, 9, 7, 6, 2, 5, 3 ],
      vec![ 3, 9, 6, 2, 1, 5, 8, 4, 7 ],
      vec![ 2, 5, 7, 3, 4, 8, 1, 6, 9 ],
      vec![ 6, 7, 5, 8, 9, 1, 3, 2, 4 ],
      vec![ 1, 3, 9, 7, 2, 4, 6, 8, 5 ],
      vec![ 4, 2, 8, 5, 6, 3, 9, 7, 1 ],
      vec![ 5, 8, 2, 1, 3, 7, 4, 9, 6 ],
      vec![ 9, 6, 1, 4, 5, 2, 7, 3, 8 ],
      vec![ 7, 4, 3, 6, 8, 9, 5, 1, 2 ],
    ])
  );
  assert!(result.steps.len() >= empty_cells);
  insta::assert_yaml_snapshot!(result.steps);
}

// https://gp.worldpuzzle.org/sites/default/files/Puzzles/2023/2023_SudokuRound7.pdf puzzle 8
#[test]
fn check_arrow_9x9_3_solve() {
  let grid_size = 9;
  let fixed_number = vec![ FixedNumber::new(0, 3, 1), FixedNumber::new(5, 8, 1) ];
  let empty_cells = grid_size * grid_size - fixed_number.len();
  let mut constraints = SudokuConstraints::new(grid_size, fixed_number);
  constraints.arrows = vec![
    Arrow {
      circle_cells: vec![ CellPosition::new(0, 0) ],
      arrow_cells: vec![
        CellPosition::new(1, 1), CellPosition::new(2, 2), CellPosition::new(3, 3),
        CellPosition::new(4, 4), CellPosition::new(5, 5),
      ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(1, 2) ],
      arrow_cells: vec![ CellPosition::new(2, 3) ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(1, 7) ],
      arrow_cells: vec![
        CellPosition::new(2, 8), CellPosition::new(2, 7), CellPosition::new(2, 6),
      ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(2, 4) ],
      arrow_cells: vec![ CellPosition::new(3, 4), CellPosition::new(4, 5) ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(3, 2), CellPosition::new(4, 2) ],
      arrow_cells: vec![
        CellPosition::new(4, 3), CellPosition::new(5, 3), CellPosition::new(5, 4),
        CellPosition::new(6, 5),
      ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(5, 1) ],
      arrow_cells: vec![ CellPosition::new(6, 2) ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(5, 6) ],
      arrow_cells: vec![ CellPosition::new(5, 7), CellPosition::new(4, 8) ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(6, 0) ],
      arrow_cells: vec![
        CellPosition::new(5, 0), CellPosition::new(4, 0), CellPosition::new(3, 0),
      ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(7, 5) ],
      arrow_cells: vec![ CellPosition::new(6, 6) ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(7, 6) ],
      arrow_cells: vec![ CellPosition::new(6, 7), CellPosition::new(7, 8) ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(7, 7) ],
      arrow_cells: vec![
        CellPosition::new(8, 8), CellPosition::new(8, 7), CellPosition::new(8, 6),
      ],
    },
    Arrow {
      circle_cells: vec![ CellPosition::new(8, 2) ],
      arrow_cells: vec![
        CellPosition::new(8, 3), CellPosition::new(7, 2), CellPosition::new(7, 1),
        CellPosition::new(7, 0),
      ],
    },
  ];
  let mut solver = Solver::new(constraints, None);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(
    result.solution.unwrap(),
    Grid(vec![
      vec![ 9, 4, 3, 1, 2, 7, 6, 5, 8 ],
      vec![ 5, 2, 8, 4, 6, 3, 1, 9, 7 ],
      vec![ 7, 6, 1, 8, 9, 5, 2, 4, 3 ],
      vec![ 1, 8, 2, 3, 5, 6, 4, 7, 9 ],
      vec![ 3, 9, 5, 7, 1, 4, 8, 6, 2 ],
      vec![ 4, 7, 6, 9, 8, 2, 5, 3, 1 ],
      vec![ 8, 3, 7, 5, 4, 1, 9, 2, 6 ],
      vec![ 2, 1, 4, 6, 3, 9, 7, 8, 5 ],
      vec![ 6, 5, 9, 2, 7, 8, 3, 1, 4 ],
    ])
  );
  assert!(result.steps.len() >= empty_cells);
  insta::assert_yaml_snapshot!(result.steps);
}
