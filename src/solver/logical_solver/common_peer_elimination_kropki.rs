use crate::solver::Solver;
use crate::types::{SolutionStep, CellPosition, Rule, KropkiDotType, Area};
use std::collections::HashSet;
use super::common_peer_elimination::CommonPeerElimination;
use super::kropki_chain_candidates::KropkiChainCandidates;
use super::technique::Technique;

// One of the kropki chain combinations would eliminate a cell's candidates, so that combination is invalid
// Note: this rule also handles naked sets and locked candidates for kropki chains
pub struct CommonPeerEliminationKropki;

impl Technique for CommonPeerEliminationKropki {
  fn get_rule(&self) -> Rule { Rule::CommonPeerEliminationKropki }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep> {
    if !solver.candidates_active {
      return vec![]
    }
    if solver.constraints.kropki_dots.is_empty() {
      return vec![]
    }

    for area in &solver.get_all_areas(false, false, false) {
      let dot_types = vec![ KropkiDotType::Consecutive, KropkiDotType::Double ];
      for dot_type in dot_types {
        let kropki_ccs = KropkiChainCandidates::compute_area_kropki_ccs(solver, area, dot_type, false);
        for (cells, kropki_dot_indices) in kropki_ccs {
          let combinations: Vec<Vec<u32>> = KropkiChainCandidates::find_kropki_ccs_combinations(solver, &cells);
          for combination in &combinations {
            // Find the values that are unique to this combination (on their position)
            let unique_indices: Vec<usize> = (0..cells.len()).filter(|&index| {
              !combinations.iter().any(|c| c[index] == combination[index] && c != combination)
            }).collect();
            if unique_indices.is_empty() {
              continue
            }

            let combination_set: HashSet<u32> = combination.iter().copied().collect();
            let common_peer_cells: Vec<CellPosition> = CommonPeerElimination::find_common_peers_for_cells_with_subset_values(
              solver, &cells, &combination_set
            );

            if common_peer_cells.is_empty() {
              continue
            }

            let mut areas = vec![ *area ];
            areas.extend(&kropki_dot_indices.into_iter().map(|index| Area::KropkiDot(index)).collect::<Vec<Area>>());

            let unique_combination: Vec<u32> = unique_indices.iter().map(|&index| combination[index]).collect();
            let unique_cells: Vec<CellPosition> = unique_indices.iter().map(|&index| cells[index]).collect();

            // TODO: may eliminate same candidate twice?
            return vec![
              SolutionStep {
                rule: self.get_rule(),
                cells: common_peer_cells, // cells that have all candidates in the combination
                values: unique_combination, // values in the same order as affected_cells
                areas,
                affected_cells: unique_cells, // cells in the chain
                candidates: None,
              }
            ]
          }
        }
      }
    }

    vec![]
  }

  fn apply_corresponding_indices(&self) -> bool { true }
}
