use std::collections::{HashMap, HashSet};
use crate::solver::{GridCheckResult, SolveState};
use crate::solver::logical::logical_solver::LogicalSolver;
use crate::solver::logical::techniques::{StepApplication, TechniqueDefinition, TechniqueKind, TechniquePhase};
use crate::solver::types::CandidateSet;
use crate::types::{CellPosition, SolutionStep};
use combinations::Combinations;
use itertools::Itertools;

// Within a house, X and Y are candidates in only 2 cells, so you can remove any other
// candidate from those 2 cells.

#[derive(Clone, Debug)]
pub(crate) struct HiddenSet {
  set_size: usize,
}

impl TechniqueDefinition for HiddenSet {
  fn kind(&self) -> TechniqueKind { if self.set_size == 2 { TechniqueKind::HiddenPair } else { TechniqueKind::HiddenTriple } }

  fn phase(&self) -> TechniquePhase { TechniquePhase::CandidateElimination }

  fn step_application(&self) -> StepApplication { StepApplication::Custom }

  fn run(&self, solver: &LogicalSolver) -> Vec<SolutionStep> {
    if !solver.state.candidates_active {
      return vec![]
    }

    for area in solver.state.puzzle.all_house_areas() {
      let value_cells: HashMap<u32, Vec<CellPosition>> = solver.state.compute_cells_by_value_in_area(&area, &solver.state.candidates).collect();

      let area_values: Vec<u32> = solver.state.compute_area_candidates_union(&area).into_iter().sorted().collect();
      let value_combinations: Vec<_> = if self.set_size < area_values.len() {
        Combinations::new(area_values, self.set_size).collect()
      } else {
        // Has to be handled separately because of stupid crate
        vec![area_values]
      };

      for values in value_combinations {
        let mut combined_cells: HashSet<CellPosition> = HashSet::new();
        for value in &values {
          let cells: HashSet<CellPosition> = value_cells[value].iter().copied().collect();
          combined_cells = combined_cells.union(&cells).cloned().collect();
          if combined_cells.len() > self.set_size {
            break
          }
        }

        if combined_cells.len() != self.set_size {
          continue
        }

        let cells_array: Vec<CellPosition> = combined_cells.into_iter().sorted().collect();
        let values_set = CandidateSet::from_iter(values.iter().copied());
        if !solver.any_cells_with_other_candidates(&cells_array, &values_set) {
          continue
        }

        return vec![
          self.build_solution_step(
            cells_array,
            values,
            vec![area],
            vec![],
          )
        ]
      }
    }

    vec![]
  }

  fn apply(&self, step: &SolutionStep, state: &mut SolveState, _step_count_limit: Option<usize>) -> GridCheckResult {
    for &CellPosition { row, col } in &step.cells {
      let value_set = CandidateSet::from_iter(step.values.iter().copied());
      state.candidates[row][col] = state.candidates[row][col].intersection(&value_set);
    }
    GridCheckResult::solved()
  }
}

impl HiddenSet {
  pub(crate) fn new(set_size: usize) -> HiddenSet {
    HiddenSet { set_size }
  }
}
