use crate::solver::logical::logical_solver::LogicalSolver;
use crate::solver::logical::techniques::{StepApplication, TechniqueDefinition, TechniqueKind, TechniquePhase};
use crate::solver::types::CandidateSet;
use crate::types::{SolutionStep, KropkiDotType, Area};
use super::common_peer_elimination::CommonPeerElimination;
use super::kropki_chain_candidate_update::KropkiChainCandidateUpdate;

// One of the kropki chain combinations would eliminate a cell's candidates, so that combination is invalid

#[derive(Clone, Debug)]
pub struct KropkiCommonPeerElimination;

const DOT_TYPES: [KropkiDotType; 2] = [KropkiDotType::Consecutive, KropkiDotType::Double];

impl TechniqueDefinition for KropkiCommonPeerElimination {
  fn kind(&self) -> TechniqueKind { TechniqueKind::KropkiCommonPeerElimination }

  fn phase(&self) -> TechniquePhase { TechniquePhase::CandidateElimination }

  fn step_application(&self) -> StepApplication {
    StepApplication::RemoveCorrespondingCandidates
  }

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

          let cell_eliminations = CommonPeerElimination::find_cell_eliminations(cells, combinations, solver);

          if cell_eliminations.is_empty() {
            continue
          }

          let (cells, values): (Vec<_>, Vec<_>) = solver
            .deduplicate_flat_eliminations(cell_eliminations, &mut cell_previous_invalid_values)
            .unzip();

          if cells.is_empty() {
            continue
          }

          steps.push(
            self.build_simple_solution_step(
              values,
              kropki_dot_indices.into_iter().map(|index| Area::KropkiDot(index)).collect::<Vec<Area>>(),
              cells,
            )
          );
        }
      }
    }

    steps
  }
}
