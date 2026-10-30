use crate::solver::logical::logical_solver::LogicalSolver;
use crate::solver::logical::techniques::Technique;
use crate::solver::logical::techniques::kropki_chain_candidates::KropkiChainCandidates;
use crate::types::{SolutionStep, Rule, Area, KropkiDotType};

// Eliminate kropki chain combinations that remove all candidates from 1 cell
pub struct KropkiAdvancedCandidates;

impl Technique for KropkiAdvancedCandidates {
  fn is_candidate_validity_update_step(&self) -> bool { true }
  fn get_rule(&self) -> Rule { Rule::KropkiAdvancedCandidates }

  fn run(&self, solver: &LogicalSolver) -> Vec<SolutionStep> {
    if !solver.state.candidates_active {
      return vec![]
    }

    let Some(constraint) = solver.state.puzzle.kropki() else { return vec![] };
    if constraint.kropki_dots.is_empty() {
      return vec![]
    }

    let mut steps: Vec<SolutionStep> = vec![];

    for area in solver.state.puzzle.all_region_areas() {
      let dot_types = vec![ KropkiDotType::Consecutive, KropkiDotType::Double ];
      for dot_type in dot_types {
        let kropki_ccs = KropkiChainCandidates::compute_area_kropki_ccs(solver, &area, dot_type, false);
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
