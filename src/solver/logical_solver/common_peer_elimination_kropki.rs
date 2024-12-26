use crate::solver::Solver;
use crate::types::{SolutionStep, Rule, KropkiDotType, Area};
use super::common_peer_elimination::CommonPeerElimination;
use super::kropki_chain_candidates::KropkiChainCandidates;
use super::technique::Technique;

// One of the kropki chain combinations would eliminate a cell's candidates, so that combination is invalid
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

    let mut steps: Vec<SolutionStep> = vec![];

    for area in &solver.get_all_proper_areas() {
      let dot_types = vec![ KropkiDotType::Consecutive, KropkiDotType::Double ];
      for dot_type in dot_types {
        let kropki_ccs = KropkiChainCandidates::compute_area_kropki_ccs(solver, area, dot_type, false);
        for (cells, kropki_dot_indices) in kropki_ccs {
          let combinations: Vec<Vec<u32>> = KropkiChainCandidates::find_kropki_ccs_combinations(solver, &cells);

          let cell_eliminations = CommonPeerElimination::find_cell_eliminations(cells, combinations, solver);

          if cell_eliminations.is_empty() {
            continue
          }

          // TODO: may eliminate same candidate twice?
          steps.push(
            self.build_simple_solution_step(
              cell_eliminations.iter().map(|(_, c)| c).copied().collect(), // values in the same order as affected_cells
              kropki_dot_indices.into_iter().map(|index| Area::KropkiDot(index)).collect::<Vec<Area>>(),
              cell_eliminations.iter().map(|(cell, _)| cell).copied().collect(),
            )
          );
        }
      }
    }

    steps
  }
}
