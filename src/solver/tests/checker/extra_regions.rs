use crate::{solver::{Solver, checker::SolvedState}, types::{Area, CellPosition, Grid, InvalidStateReason, InvalidStateType, Region, SudokuConstraints}};

#[test]
fn check_wrong_region() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.extra_regions = vec![
    Region(vec![
      CellPosition::new(1, 0), CellPosition::new(1, 1),
      CellPosition::new(1, 2), CellPosition::new(2, 2), 
    ]),
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
        state_type: InvalidStateType::AreaValueConflict,
        area: Area::Region(4),
        values: vec![3],
      }
    )
  );
}

#[test]
fn check_solved_grid() {
  let mut constraints = SudokuConstraints::new(4, vec![]);
  constraints.extra_regions = vec![
    Region(vec![
      CellPosition::new(1, 0), CellPosition::new(1, 1),
      CellPosition::new(0, 1), CellPosition::new(2, 1),
    ]),
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
