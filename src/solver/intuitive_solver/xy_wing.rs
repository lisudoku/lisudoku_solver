use std::collections::HashSet;
use itertools::Itertools;
use crate::solver::Solver;
use crate::types::{SolutionStep, Rule, Area, CellPosition};

impl Solver {
  pub fn find_xy_wing(&self) -> Option<SolutionStep> {
    if !self.candidates_active {
      return None;
    }

    let cells_with_2: Vec<CellPosition> = self.get_empty_area_cells(&Area::Grid)
      .into_iter()
      .filter(|cell| self.candidates[cell.row][cell.col].len() == 2)
      .collect();

    for xy_cell in cells_with_2 {
      let candidates = &self.candidates[xy_cell.row][xy_cell.col];
      let values: Vec<u32> = candidates.iter().copied().sorted().collect();
      let x = values[0];
      let y = values[1];
      let seen_cells = self.get_affected_by_cell(&xy_cell, candidates);

      for xz_cell in &seen_cells {
        let candidates = &self.candidates[xz_cell.row][xz_cell.col];
        if candidates.len() != 2 || !candidates.contains(&x) {
          continue
        }
        let z = *candidates.iter().filter(|&&value| value != x).next().unwrap();
        let need_candidates = &HashSet::from([ y, z ]);

        for yz_cell in &seen_cells {
          let candidates = &self.candidates[yz_cell.row][yz_cell.col];
          if yz_cell == xz_cell || candidates != need_candidates {
            continue
          }

          // Found the XYZ triplet, now search for cells with candidate Z that both XZ and YZ see
          let affected_cells = self.get_affected_by_cells(&vec![ *xz_cell, *yz_cell ], &HashSet::from([z]));

          if !affected_cells.is_empty() {
            return Some(
              SolutionStep {
                rule: Rule::XYWing,
                cells: vec![ xy_cell, *xz_cell, *yz_cell ],
                values: vec![ x, y, z ],
                areas: vec![],
                affected_cells,
                candidates: None,
              }
            )
          }
        }
      }
    }

    None
  }
}
