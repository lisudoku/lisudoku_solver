use crate::solver::logical::logical_solver::LogicalSolver;
use crate::solver::logical::techniques::{TechniqueDefinition, TechniqueKind, TechniquePhase};
use crate::solver::logical::techniques::kropki_chain_candidate_update::KropkiChainCandidateUpdate;
use crate::solver::types::CandidateSet;
use crate::types::{SolutionStep, Area, KropkiDotType};
use itertools::Itertools;

// Eliminate kropki chain combinations that remove all candidates from 1 cell

#[derive(Clone, Debug)]
pub(crate) struct KropkiAdvancedCandidateUpdate;

const DOT_TYPES: [KropkiDotType; 2] = [KropkiDotType::Consecutive, KropkiDotType::Double];

impl TechniqueDefinition for KropkiAdvancedCandidateUpdate {
  fn kind(&self) -> TechniqueKind { TechniqueKind::KropkiAdvancedCandidateUpdate }

  fn phase(&self) -> TechniquePhase { TechniquePhase::CandidateValidity }

  fn run(&self, solver: &LogicalSolver) -> Vec<SolutionStep> {
    if !solver.state.candidates_active {
      return vec![]
    }

    let Some(constraint) = solver.state.puzzle.kropki() else { return vec![] };
    if constraint.kropki_dots.is_empty() {
      return vec![]
    }

    let mut steps: Vec<SolutionStep> = vec![];
    let grid_size = solver.state.puzzle.grid_size;
    let mut cell_previous_invalid_values: Vec<CandidateSet> = vec![CandidateSet::new(); grid_size * grid_size];

    for area in solver.state.puzzle.all_region_areas() {
      for dot_type in DOT_TYPES {
        let kropki_ccs = KropkiChainCandidateUpdate::compute_area_kropki_ccs(solver, &area, dot_type, false);
        for (cells, kropki_dot_indices) in kropki_ccs {
          let combinations: Vec<Vec<u32>> = KropkiChainCandidateUpdate::find_kropki_ccs_combinations(solver, &cells);

          let invalid_candidates = solver.eliminate_combinations(&combinations, &cells);

          if invalid_candidates.is_empty() {
            continue
          }

          let current_steps = solver.deduplicate_eliminations(invalid_candidates, &mut cell_previous_invalid_values)
            .map(|(cell, invalid_values)| {
              self.build_simple_solution_step(
                invalid_values.into_iter().collect_vec(),
                kropki_dot_indices.iter().map(|&index| Area::KropkiDot(index)).collect(),
                vec![ cell ],
              )
            });

          steps.extend(current_steps);
        }
      }
    }

    steps
  }
}
