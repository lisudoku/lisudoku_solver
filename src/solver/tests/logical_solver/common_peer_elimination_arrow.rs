use crate::{solver::logical::{logical_solver::LogicalSolver, techniques::{Technique, arrow_candidates::ArrowCandidates, candidates::Candidates, common_peer_elimination_arrow::CommonPeerEliminationArrow}}, types::{Area, Arrow, CellPosition, FixedNumber, Rule, SolutionStep, SudokuConstraints}};

#[test]
fn check_arrow_common_peer_elimination() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 5, 1), FixedNumber::new(1, 5, 9),
        FixedNumber::new(5, 1, 1), FixedNumber::new(6, 0, 1),
      ]
    )
    .with_arrows(
      vec![
        Arrow {
          circle_cells: vec![ CellPosition::new(1, 7) ],
          arrow_cells: vec![
            CellPosition::new(2, 8), CellPosition::new(2, 7), CellPosition::new(2, 6),
          ],
        },
      ]
    );
  let mut solver = LogicalSolver::new(constraints);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = ArrowCandidates.run(&solver);
  solver.apply_rules(&steps);

  let steps = CommonPeerEliminationArrow.run(&solver);
  assert_eq!(steps.len(), 1);

  let mut step = steps.first().unwrap();
  assert_eq!(
    *step,
    SolutionStep::new(
      Rule::CommonPeerEliminationArrow,
      vec![],
      vec![1; 3],
      vec![Area::Arrow(0)],
      vec![CellPosition::new(1, 6), CellPosition::new(1, 8), CellPosition::new(2, 2)],
    )
  );

  assert!(solver.state.candidates[2][2].contains(&1));

  solver.apply_rule(&mut step);
  assert!(!solver.state.candidates[2][2].contains(&1));
}

// N4IghgThD2DuDOIBcBtUkawMIFMA2eiqoAxtHsgIwA0ImyADAL7WnlW31KUtsXec4yAExMAurRIBLCCTw5cBImhBl%2BwwbEbje4KHEWFkKtcgAcm5ABZdppAHZLSG61XskANic2JqmXIV8I2I3fgs6ISQAVh1fADMpAA8cABMAOQBXAFsAIxwIZVAAB2h4KQAXKWgAO2Q%2BcycY2gA3MDwMnHNdErLKmrrQkSdmFraOrt8AcwgpFIBlKQAvTqQATiYgA%3D%3D
#[test]
fn arrow_common_peer_elimination_no_duplicates() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![FixedNumber::new(0, 2, 8), FixedNumber::new(5, 8, 8)]
    )
    .with_arrows(
      vec![
        Arrow {
          circle_cells: vec![CellPosition::new(0, 2)],
          arrow_cells: vec![
            CellPosition::new(0, 1), CellPosition::new(1, 1), CellPosition::new(2, 1),
          ],
        },
        Arrow {
          circle_cells: vec![CellPosition::new(5, 8)],
          arrow_cells: vec![
            CellPosition::new(4, 8), CellPosition::new(4, 7), CellPosition::new(4, 6),
          ],
        },
      ]
    );
  let mut solver = LogicalSolver::new(constraints);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = ArrowCandidates.run(&solver);
  solver.apply_rules(&steps);

  let steps = CommonPeerEliminationArrow.run(&solver);
  assert_eq!(steps.len(), 2);

  let step = &steps[0];
  assert_eq!(
    *step,
    SolutionStep::new(
      Rule::CommonPeerEliminationArrow,
      vec![],
      vec![1; 11],
      vec![Area::Arrow(0)],
      vec![
        CellPosition::new(0, 0), CellPosition::new(1, 0), CellPosition::new(1, 2),
        CellPosition::new(2, 0), CellPosition::new(2, 2), CellPosition::new(3, 1),
        CellPosition::new(4, 1), CellPosition::new(5, 1), CellPosition::new(6, 1),
        CellPosition::new(7, 1), CellPosition::new(8, 1),
      ],
    )
  );

  // Should not repeat (4, 1)
  let step = &steps[1];
  assert_eq!(
    *step,
    SolutionStep::new(
      Rule::CommonPeerEliminationArrow,
      vec![],
      vec![1; 10],
      vec![Area::Arrow(1)],
      vec![
        CellPosition::new(3, 6), CellPosition::new(3, 7), CellPosition::new(3, 8),
        CellPosition::new(4, 0), CellPosition::new(4, 2), CellPosition::new(4, 3),
        CellPosition::new(4, 4), CellPosition::new(4, 5), CellPosition::new(5, 6),
        CellPosition::new(5, 7),
      ],
    )
  );
}
