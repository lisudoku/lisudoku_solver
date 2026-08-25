use crate::{solver::Solver, types::{Arrow, CellPosition, FixedNumber, Grid, SolutionType, SudokuConstraints}};

#[test]
fn check_arrow_4x4_1_solve() {
  let constraints = SudokuConstraints::new(4)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 3, 3), FixedNumber::new(2, 0, 1),
      ]
    )
    .with_arrows(
      vec![
        Arrow {
          circle_cells: vec![ CellPosition::new(3, 0) ],
          arrow_cells: vec![
            CellPosition::new(2, 1), CellPosition::new(1, 2), CellPosition::new(0, 1),
          ],
        },
        Arrow {
          circle_cells: vec![ CellPosition::new(3, 1) ],
          arrow_cells: vec![ CellPosition::new(3, 2), CellPosition::new(3, 3) ],
        },
      ]
    );
  let mut solver = Solver::new(constraints);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(
    result.solution.unwrap(),
    Grid(vec![
      vec![ 2, 1, 4, 3 ],
      vec![ 3, 4, 1, 2 ],
      vec![ 1, 2, 3, 4 ],
      vec![ 4, 3, 2, 1 ],
    ])
  );
  insta::assert_yaml_snapshot!(result.steps);
}
