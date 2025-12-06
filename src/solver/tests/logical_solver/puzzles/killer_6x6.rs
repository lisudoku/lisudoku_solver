use crate::{solver::Solver, types::{CellPosition, FixedNumber, Grid, KillerCage, Region, SolutionType, SudokuConstraints}};

// https://uploads-ssl.webflow.com/62793457876c001d28edf162/6348945a45b06acb414391b7_WSC_2022_IB_v2.1.pdf
#[test]
fn check_killer_6x6_1_solve() {
  let constraints = SudokuConstraints::new(6)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 0, 1),
        FixedNumber::new(0, 2, 3),
        FixedNumber::new(5, 3, 3),
        FixedNumber::new(5, 5, 2),
      ]
    )
    .with_killer_cages(
      vec![
        KillerCage {
          sum: Some(5),
          region: Region(vec![
            CellPosition::new(0, 0), CellPosition::new(1, 0),
          ]),
        },
        KillerCage {
          sum: Some(15),
          region: Region(vec![
            CellPosition::new(0, 1), CellPosition::new(0, 2),
            CellPosition::new(0, 3), CellPosition::new(0, 4),
            CellPosition::new(1, 3),
          ]),
        },
        KillerCage {
          sum: Some(5),
          region: Region(vec![ CellPosition::new(1, 1) ]),
        },
        KillerCage {
          sum: Some(5),
          region: Region(vec![
            CellPosition::new(1, 4), CellPosition::new(1, 5),
          ]),
        },
        KillerCage {
          sum: Some(5),
          region: Region(vec![
            CellPosition::new(2, 0), CellPosition::new(2, 1),
          ]),
        },
        KillerCage {
          sum: Some(5),
          region: Region(vec![
            CellPosition::new(2, 2), CellPosition::new(3, 2),
          ]),
        },
        KillerCage {
          sum: Some(5),
          region: Region(vec![ CellPosition::new(3, 0) ]),
        },
        KillerCage {
          sum: Some(15),
          region: Region(vec![
            CellPosition::new(2, 3), CellPosition::new(2, 4), CellPosition::new(2, 5),
          ]),
        },
        KillerCage {
          sum: Some(5),
          region: Region(vec![
            CellPosition::new(3, 3), CellPosition::new(3, 4),
          ]),
        },
        KillerCage {
          sum: Some(15),
          region: Region(vec![
            CellPosition::new(4, 0), CellPosition::new(4, 1), CellPosition::new(4, 2),
            CellPosition::new(5, 0),
          ]),
        },
        KillerCage {
          sum: Some(5),
          region: Region(vec![ CellPosition::new(4, 5) ]),
        },
        KillerCage {
          sum: Some(15),
          region: Region(vec![
            CellPosition::new(5, 1), CellPosition::new(5, 2), CellPosition::new(5, 3),
            CellPosition::new(5, 4), CellPosition::new(5, 5),
          ]),
        },
      ]
    );
  let mut solver = Solver::new(constraints);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(
    result.solution.unwrap(),
    Grid(vec![
      vec![ 1, 2, 3, 4, 5, 6 ],
      vec![ 4, 5, 6, 1, 2, 3 ],
      vec![ 2, 3, 1, 5, 6, 4 ],
      vec![ 5, 6, 4, 2, 3, 1 ],
      vec![ 3, 4, 2, 6, 1, 5 ],
      vec![ 6, 1, 5, 3, 4, 2 ],
    ])
  );
  insta::assert_yaml_snapshot!(result.steps);
}
