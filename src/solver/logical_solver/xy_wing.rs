use std::collections::HashSet;
use itertools::Itertools;
use crate::solver::Solver;
use crate::types::{CellPosition, InvalidStateReason, Rule, SolutionStep};
use super::technique::Technique;

pub struct XYWing;

impl Technique for XYWing {
  fn get_rule(&self) -> Rule { Rule::XYWing }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep> {
    if !solver.candidates_active {
      return vec![]
    }

    let cells_with_2: Vec<CellPosition> = solver
      .get_all_empty_cells()
      .into_iter()
      .filter(|cell| solver.candidates[cell.row][cell.col].len() == 2)
      .collect();

    for xy_cell in cells_with_2 {
      let candidates = &solver.candidates[xy_cell.row][xy_cell.col];
      let values: Vec<u32> = candidates.iter().copied().sorted().collect();
      let x = values[0];
      let y = values[1];
      let seen_cells = solver.get_affected_by_cell(&xy_cell, candidates);

      for xz_cell in &seen_cells {
        let candidates = &solver.candidates[xz_cell.row][xz_cell.col];
        if candidates.len() != 2 || !candidates.contains(&x) {
          continue
        }
        let z = *candidates.iter().filter(|&&value| value != x).next().unwrap();
        let need_candidates = &HashSet::from([ y, z ]);

        for yz_cell in &seen_cells {
          let candidates = &solver.candidates[yz_cell.row][yz_cell.col];
          if yz_cell == xz_cell || candidates != need_candidates {
            continue
          }

          // Found the XYZ triplet, now search for cells with candidate Z that both XZ and YZ see
          let affected_cells = solver.get_affected_by_cells(&vec![ *xz_cell, *yz_cell ], &HashSet::from([z]));

          if !affected_cells.is_empty() {
            return vec![
              self.build_solution_step(
                vec![ xy_cell, *xz_cell, *yz_cell ],
                vec![ x, y, z ],
                vec![],
                affected_cells,
              )
            ]
          }
        }
      }
    }

    vec![]
  }

  fn apply(&self, step: &SolutionStep, solver: &mut Solver) -> (bool, Option<InvalidStateReason>) {
    for &CellPosition { row, col } in &step.affected_cells {
      // Remove Z as candidate
      solver.candidates[row][col].remove(&step.values[2]);
    }
    (true, None)
  }
}
