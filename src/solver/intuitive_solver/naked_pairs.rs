use std::collections::HashSet;
use crate::solver::Solver;
use crate::types::{SolutionStep, Rule};

impl Solver {
  pub fn find_naked_pairs(&self) -> Option<SolutionStep> {
    return None

    // if !self.candidates_active {
    //   return None
    // }

    // let areas = self.get_all_areas();
    // for area in areas {
    //   for cell1 in self.get_area_cells(area) {
    //     for cell2 in self.get_area_cells(area) {
    //       let candidates1 = &self.candidates[cell1.row][cell1.col];
    //       let candidates2 = &self.candidates[cell2.row][cell2.col];
    //       let common: HashSet<_> = candidates1.intersection(candidates2).collect();

    //       if common.len() >= 2 {
    //         // check if any other cell has any of the candidates
    //         // if yes, remove them (actually just provide the info so that intuitive_solver can remove them)
    //       }
    //     }
    //   }
    // }

    // return Some(
    //   SolutionStep {
    //     rule: Rule::Candidates,
    //     cells: vec![], // the 2 cells
    //     values: vec![], // the 2 values that they both have as candidates
    //     areas: vec![], // what the 2 cells have in common
    //     affected_cells: vec![],
    //     candidates: None,
    //   }
    // )
  }
}
