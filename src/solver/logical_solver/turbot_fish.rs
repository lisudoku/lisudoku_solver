use std::collections::{HashMap, HashSet};
use std::mem::swap;
use itertools::Itertools;
use crate::solver::Solver;
use crate::types::{SolutionStep, Rule, Area, CellPosition};
use super::technique::Technique;

pub struct TurbotFish;

// Finds 2 strong links that have one of their ends see each other.
impl Technique for TurbotFish {
  fn get_rule(&self) -> Rule { Rule::TurbotFish }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep> {
    if !solver.candidates_active {
      return vec![]
    }

    let strong_links: Vec<(Area, u32, CellPosition, CellPosition)> = solver.get_all_areas(false, false, false).iter().flat_map(|area| {
      let value_cells = solver.compute_cells_by_value_in_area(area, &solver.candidates);

      value_cells.into_iter().filter_map(|(value, cells)| {
        if cells.len() != 2 {
          return None
        }
        return Some(
          (*area, value, cells[0], cells[1])
        )
      })
    }).collect();

    let strong_links_by_value: HashMap<u32, Vec<(Area, u32, CellPosition, CellPosition)>> = strong_links
      .iter()
      .cloned()
      .sorted_by_key(|link| (link.1, link.0, link.2, link.3))
      .group_by(|link| link.1)
      .into_iter()
      .map(|(value, group)| (value, group.collect()))
      .collect();

    for (area1, value, a1, a2) in &strong_links {
      for (area2, _, b1, b2) in &strong_links_by_value[value] {
        if area1 == area2 || a1 == b1 || a1 == b2 || a2 == b1 || a2 == b2 {
          continue
        }
        let a1b1 = solver.cells_affect_eachother(a1, b1);
        let a1b2 = solver.cells_affect_eachother(a1, b2);
        let a2b1 = solver.cells_affect_eachother(a2, b1);
        let a2b2 = solver.cells_affect_eachother(a2, b2);

        // There has to be exactly one pair of cells that see each other
        if a1b1 as u32 + a1b2 as u32 + a2b1 as u32 + a2b2 as u32 != 1 {
          continue
        }

        // The cells that see each other should be a1 - b1
        let mut a1 = *a1;
        let mut a2 = *a2;
        let mut b1 = *b1;
        let mut b2 = *b2;
        if a2b1 || a2b2 {
          swap(&mut a1, &mut a2);
        }
        if a1b2 || a2b2 {
          swap(&mut b1, &mut b2);
        }

        let a_peers: HashSet<CellPosition> = solver.get_cell_peers_with_candidate(&a2, *value).into_iter().collect();
        let b_peers: HashSet<CellPosition> = solver.get_cell_peers_with_candidate(&b2, *value).into_iter().collect();
        let common_peers: Vec<CellPosition> = a_peers.intersection(&b_peers).cloned().collect();

        if common_peers.is_empty() {
          continue
        }

        return vec![
          SolutionStep {
            rule: self.get_rule(),
            cells: vec![ a1, a2, b1, b2 ],
            values: vec![ *value ],
            areas: vec![ *area1, *area2 ],
            affected_cells: common_peers,
            candidates: None,
          }
        ]
      }
    }

    vec![]
  }
}
