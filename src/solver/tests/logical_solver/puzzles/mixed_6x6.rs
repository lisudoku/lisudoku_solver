use crate::{solver::Solver, types::{CellPosition, Grid, KropkiDot, Region, SolutionType, SudokuConstraints}};

// https://uploads-ssl.webflow.com/62793457876c001d28edf162/6348945a45b06acb414391b7_WSC_2022_IB_v2.1.pdf
#[test]
fn check_mixed_6x6_irregular_kropki_solve() {
  // Added without negative condition, it is not needed
  let constraints = SudokuConstraints::new(6)
    .with_regions(
      vec![
        Region(vec![
          CellPosition::new(0, 0), CellPosition::new(0, 1), CellPosition::new(0, 2),
          CellPosition::new(0, 3), CellPosition::new(1, 0), CellPosition::new(1, 1),
        ]),
        Region(vec![
          CellPosition::new(0, 4), CellPosition::new(0, 5), CellPosition::new(1, 2),
          CellPosition::new(1, 3), CellPosition::new(1, 4), CellPosition::new(1, 5),
        ]),
        Region(vec![
          CellPosition::new(2, 0), CellPosition::new(2, 1), CellPosition::new(2, 2),
          CellPosition::new(3, 0), CellPosition::new(3, 1), CellPosition::new(3, 2),
        ]),
        Region(vec![
          CellPosition::new(2, 3), CellPosition::new(2, 4), CellPosition::new(2, 5),
          CellPosition::new(3, 3), CellPosition::new(3, 4), CellPosition::new(3, 5),
        ]),
        Region(vec![
          CellPosition::new(4, 0), CellPosition::new(4, 1), CellPosition::new(4, 2),
          CellPosition::new(5, 0), CellPosition::new(5, 1), CellPosition::new(5, 2),
        ]),
        Region(vec![
          CellPosition::new(4, 3), CellPosition::new(4, 4), CellPosition::new(4, 5),
          CellPosition::new(5, 3), CellPosition::new(5, 4), CellPosition::new(5, 5),
        ]),
      ]
    )
    .with_kropki_dots(
      vec![
        KropkiDot::consecutive(CellPosition::new(0, 0), CellPosition::new(0, 1)),
        KropkiDot::consecutive(CellPosition::new(0, 1), CellPosition::new(0, 2)),
        KropkiDot::consecutive(CellPosition::new(0, 2), CellPosition::new(0, 3)),
        KropkiDot::consecutive(CellPosition::new(0, 2), CellPosition::new(1, 2)),
        KropkiDot::consecutive(CellPosition::new(0, 3), CellPosition::new(0, 4)),
        KropkiDot::consecutive(CellPosition::new(0, 4), CellPosition::new(0, 5)),
        KropkiDot::consecutive(CellPosition::new(1, 4), CellPosition::new(1, 5)),
        KropkiDot::consecutive(CellPosition::new(1, 4), CellPosition::new(2, 4)),
        KropkiDot::double(CellPosition::new(0, 3), CellPosition::new(1, 3)),
        KropkiDot::double(CellPosition::new(0, 4), CellPosition::new(1, 4)),
        KropkiDot::double(CellPosition::new(1, 0), CellPosition::new(1, 1)),
        KropkiDot::double(CellPosition::new(1, 0), CellPosition::new(2, 0)),
        KropkiDot::double(CellPosition::new(1, 2), CellPosition::new(1, 3)),
        KropkiDot::consecutive(CellPosition::new(2, 1), CellPosition::new(2, 2)),
        KropkiDot::consecutive(CellPosition::new(2, 4), CellPosition::new(2, 5)),
        KropkiDot::consecutive(CellPosition::new(3, 1), CellPosition::new(3, 2)),
        KropkiDot::consecutive(CellPosition::new(3, 1), CellPosition::new(4, 1)),
        KropkiDot::consecutive(CellPosition::new(3, 3), CellPosition::new(3, 4)),
        KropkiDot::consecutive(CellPosition::new(3, 3), CellPosition::new(4, 3)),
        KropkiDot::consecutive(CellPosition::new(3, 4), CellPosition::new(3, 5)),
        KropkiDot::double(CellPosition::new(2, 1), CellPosition::new(3, 1)),
        KropkiDot::double(CellPosition::new(3, 2), CellPosition::new(3, 3)),
        KropkiDot::consecutive(CellPosition::new(4, 0), CellPosition::new(4, 1)),
        KropkiDot::consecutive(CellPosition::new(4, 2), CellPosition::new(4, 3)),
        KropkiDot::consecutive(CellPosition::new(4, 5), CellPosition::new(5, 5)),
        KropkiDot::consecutive(CellPosition::new(5, 0), CellPosition::new(5, 1)),
        KropkiDot::double(CellPosition::new(4, 1), CellPosition::new(5, 1)),
        KropkiDot::double(CellPosition::new(5, 2), CellPosition::new(5, 3)),
        KropkiDot::double(CellPosition::new(5, 4), CellPosition::new(5, 5)),
      ]
    );
  let mut solver = Solver::new(constraints);
  let result = solver.logical_solve();
  assert_eq!(result.solution_type, SolutionType::Full);
  assert_eq!(
    result.solution.unwrap(),
    Grid(vec![
      vec![ 6, 5, 4, 3, 2, 1 ],
      vec![ 2, 1, 3, 6, 4, 5 ],
      vec![ 4, 6, 5, 1, 3, 2 ],
      vec![ 1, 3, 2, 4, 5, 6 ],
      vec![ 3, 2, 6, 5, 1, 4 ],
      vec![ 5, 4, 1, 2, 6, 3 ],
    ])
  );
  insta::assert_yaml_snapshot!(result.steps);
}
