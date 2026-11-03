use crate::{solver::logical::{logical_solver::LogicalSolver, techniques::{Technique, arrow_advanced_candidates::ArrowAdvancedCandidates, arrow_candidates::ArrowCandidates, candidates::Candidates}}, types::{Area, Arrow, CellPosition, FixedNumber, Rule, SolutionStep, SudokuConstraints}};

// N4IghgThD2DuDOIBcBtUkawMIFMA2eiqoAxtHsgBwA0ImVAvtaecgOy31KVMsVIA2TnEYBdWiQCWEEnhy4CRNCDL8OdEUjYNRO2gDNJADxwATAHIBXALYAjHBCWgADtHiSALpOgA7ZH2QABmFYIKYQADcwPEscZABGXhBXdy9ffxVWJGCNUKRE2iiYuKQAFiSUz28%2FJADskOQAJnCi2OQAVgq3KvTazP4crgBmFui2wS7U6ozVIIay0eL2SZ6ausHNTsKxkp5mZO60tf653OQBRfGAThWjmaz1LgvtpaQtl0PpvtnueefInbnPQgADmEEkpgAypIAF4lG5AA
#[test]
fn check_arrow_advanced_candidates() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 0, 1), FixedNumber::new(1, 0, 4), FixedNumber::new(2, 0, 5),
        FixedNumber::new(3, 0, 6), FixedNumber::new(4, 0, 7), FixedNumber::new(5, 0, 8),
        FixedNumber::new(6, 0, 9), FixedNumber::new(6, 7, 5), FixedNumber::new(6, 8, 6),
      ]
    )
    .with_arrows(
      vec![
        Arrow {
          circle_cells: vec![ CellPosition::new(7, 7) ],
          arrow_cells: vec![
            CellPosition::new(8, 8), CellPosition::new(8, 7), CellPosition::new(8, 6),
          ],
        },
      ]
    );
  let mut solver = LogicalSolver::new(constraints);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = ArrowCandidates.run(&solver);
  solver.apply_rules(&steps);

  let steps = ArrowAdvancedCandidates.run(&solver);
  assert_eq!(steps.len(), 1);
  let mut step = steps.first().unwrap();
  assert_eq!(
    *step,
    SolutionStep::new(
      Rule::ArrowAdvancedCandidates,
      vec![],
      vec![9],
      vec![Area::Arrow(0)],
      vec![CellPosition::new(7, 7)],
    )
  );
  assert!(solver.state.candidates[7][7].contains(&9));

  solver.apply_rule(&mut step);
  assert!(!solver.state.candidates[7][7].contains(&9));
}

// N4IghgThD2DuDOIBcBtUkawMIFMA2eiqoAxtHsgGwA0ImyAzAL7WnlW31IAsTAurRIBLCCTw5cBImhBkKSAOyc4yAEz8W6KHEmFkMucgAcy2I02z2SE3RU9%2BgkWIn49xS%2FKW2zSdXwcgAGZCAB44ACYAcgCuALYARjgQ0qAADtDwQgAuQtAAdshsnqbIAAwsIABuYHjROGoW6Zk5%2BYUeZSVIzLTVtfX2rCBN2bkFSEXIAIyd3VU1dcgArI0ZI63j7b4zFb0LSJQrzaNthl3bPfP9CodrYxM853N9xjctd5uLj7v9AJwBAOYQIThADKQgAXr8mEA%3D
#[test]
fn arrow_advanced_candidates_no_duplicates() {
  let constraints = SudokuConstraints::new(9)
    .with_fixed_numbers(
      vec![
        FixedNumber::new(0, 7, 2), FixedNumber::new(3, 0, 4), FixedNumber::new(3, 1, 5),
        FixedNumber::new(3, 2, 6), FixedNumber::new(3, 3, 7), FixedNumber::new(3, 4, 8),
        FixedNumber::new(3, 5, 9),
      ]
    )
    .with_arrows(
      vec![
        Arrow {
          circle_cells: vec![CellPosition::new(2, 7)],
          arrow_cells: vec![CellPosition::new(3, 6), CellPosition::new(4, 6)],
        },
        Arrow {
          circle_cells: vec![CellPosition::new(2, 7)],
          arrow_cells: vec![CellPosition::new(3, 8), CellPosition::new(4, 8)],
        },
      ]
    );
  let mut solver = LogicalSolver::new(constraints);
  solver.apply_rule(&mut Candidates.run(&solver).first().unwrap());

  let steps = ArrowCandidates.run(&solver);
  solver.apply_rules(&steps);

  let steps = ArrowAdvancedCandidates.run(&solver);
  assert_eq!(steps.len(), 3);

  let step = &steps[0];
  assert_eq!(
    *step,
    SolutionStep::new(
      Rule::ArrowAdvancedCandidates,
      vec![],
      vec![1],
      vec![Area::Arrow(0)],
      vec![CellPosition::new(4, 6)],
    )
  );

  let step = &steps[1];
  assert_eq!(
    *step,
    SolutionStep::new(
      Rule::ArrowAdvancedCandidates,
      vec![],
      vec![3, 4],
      vec![Area::Arrow(0)],
      vec![CellPosition::new(2, 7)],
    )
  );

  let step = &steps[2];
  assert_eq!(
    *step,
    SolutionStep::new(
      Rule::ArrowAdvancedCandidates,
      vec![],
      vec![1],
      vec![Area::Arrow(1)],
      vec![CellPosition::new(4, 8)],
    )
  );
}
