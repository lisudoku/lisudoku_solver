use crate::solver::Solver;
use crate::types::{SolutionStep, CellPosition, Rule, KropkiDotType, Area};
use std::collections::HashSet;

// One of the kropki chain combinations would eliminate a cell's candidates, so that combination is invalid
// Note: this rule also handles naked sets and locked candidates for kropki chains
impl Solver {
  pub fn find_common_peer_elimination_kropki(&self) -> Option<SolutionStep> {
    if !self.candidates_active {
      return None
    }
    if self.constraints.kropki_dots.is_empty() {
      return None
    }

    for area in &self.get_all_areas(false, false, false) {
      let dot_types = vec![ KropkiDotType::Consecutive, KropkiDotType::Double ];
      for dot_type in dot_types {
        let kropki_ccs = self.compute_area_kropki_ccs(area, dot_type, false);
        for (cells, kropki_dot_indices) in kropki_ccs {
          let combinations: Vec<Vec<u32>> = self.find_kropki_ccs_combinations(&cells);
          for combination in &combinations {
            // Find the values that are unique to this combination (on their position)
            let unique_indices: Vec<usize> = (0..cells.len()).filter(|&index| {
              !combinations.iter().any(|c| c[index] == combination[index] && c != combination)
            }).collect();
            if unique_indices.is_empty() {
              continue
            }

            let combination_set: HashSet<u32> = combination.iter().copied().collect();
            let common_peer_cells: Vec<CellPosition> = self.find_common_peers_for_cells_with_subset_values(
              &cells, &combination_set
            );

            if common_peer_cells.is_empty() {
              continue
            }

            let mut areas = vec![ *area ];
            areas.extend(&kropki_dot_indices.into_iter().map(|index| Area::KropkiDot(index)).collect::<Vec<Area>>());

            let unique_combination: Vec<u32> = unique_indices.iter().map(|&index| combination[index]).collect();
            let unique_cells: Vec<CellPosition> = unique_indices.iter().map(|&index| cells[index]).collect();

            return Some(
              SolutionStep {
                rule: Rule::CommonPeerEliminationKropki,
                cells: common_peer_cells, // cells that have all candidates in the combination
                values: unique_combination, // values in the same order as affected_cells
                areas,
                affected_cells: unique_cells, // cells in the chain
                candidates: None,
              }
            )
          }
        }
      }
    }

    None
  }

  fn find_common_peers_for_cells_with_subset_values(&self, cells: &Vec<CellPosition>, values: &HashSet<u32>) -> Vec<CellPosition> {
    let common_peers = self.find_common_peers_for_cells(cells);
    let common_peers_with_values: Vec<CellPosition> = self.filter_cells_with_subset_candidates(&common_peers, &values);
    common_peers_with_values
  }
}
