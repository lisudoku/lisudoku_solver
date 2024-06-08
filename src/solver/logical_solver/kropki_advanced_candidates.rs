use crate::solver::Solver;
use crate::solver::logical_solver::kropki_chain_candidates::KropkiChainCandidates;
use crate::types::{SolutionStep, Rule, Area, KropkiDotType};
use super::technique::Technique;

// Eliminate kropki chain combinations that remove all candidates from 1 cell
pub struct KropkiAdvancedCandidates;

impl Technique for KropkiAdvancedCandidates {
  fn is_candidate_validity_update_step(&self) -> bool { true }
  fn get_rule(&self) -> Rule { Rule::KropkiAdvancedCandidates }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep> {
    if !solver.candidates_active {
      return vec![]
    }
    if solver.constraints.kropki_dots.is_empty() {
      return vec![]
    }

    let mut steps: Vec<SolutionStep> = vec![];

    for area in &solver.get_all_areas(false, false, false, false) {
      let dot_types = vec![ KropkiDotType::Consecutive, KropkiDotType::Double ];
      for dot_type in dot_types {
        let kropki_ccs = KropkiChainCandidates::compute_area_kropki_ccs(solver, area, dot_type, false);
        for (cells, kropki_dot_indices) in kropki_ccs {
          let combinations: Vec<Vec<u32>> = KropkiChainCandidates::find_kropki_ccs_combinations(solver, &cells);

          let invalid_candidates = solver.eliminate_combinations(&combinations, &cells);

          // TODO: may eliminate same candidate twice?
          let current_steps = invalid_candidates.into_iter().map(|(cell, invalid_values)| {
            self.build_simple_solution_step(
              invalid_values,
              kropki_dot_indices.iter().map(|&index| Area::KropkiDot(index)).collect(),
              vec![ cell ],
            )
          }).collect::<Vec<_>>();

          steps.extend(current_steps);
        }
      }
    }

    steps
  }
}
