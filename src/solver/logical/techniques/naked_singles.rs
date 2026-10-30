use crate::solver::constraints::{CandidateMode, PeerMode};
use crate::solver::logical::logical_solver::LogicalSolver;
use crate::solver::logical::techniques::Technique;
use crate::solver::types::CandidateSet;
use crate::types::{SolutionStep, CellPosition, Rule, Area};
use combinations::Combinations;
use itertools::Itertools;

pub struct NakedSingle;

impl Technique for NakedSingle {
  fn is_grid_step(&self) -> bool { true }
  fn get_rule(&self) -> Rule { Rule::NakedSingle }
  fn complexity(&self, step: &SolutionStep, _solver: &LogicalSolver) -> u32 {
    step.areas.len() as u32
  }

  fn run(&self, solver: &LogicalSolver) -> Vec<SolutionStep> {
    let mut steps = vec![];
    for cell in solver.state.all_empty_cells() {
      let step_opt = self.find_naked_single_in_cell(cell, solver);
      let Some(step) = step_opt else { continue };
      steps.push(step);
      if solver.performance_mode {
        return steps
      }
    }
    steps
  }
}

impl NakedSingle {
  fn find_naked_single_in_cell(&self, cell: CellPosition, solver: &LogicalSolver) -> Option<SolutionStep> {
    let cell_candidates = solver.state.compute_cell_candidates(&cell);

    if solver.performance_mode && cell_candidates.is_empty() {
      // Break early since cell has no candidates.
      // Only relevant in performance_mode because otherwise we run the checker on each step.
      return Some(self.build_invalid_cell_step(&cell))
    }

    if !cell_candidates.has_count_one() {
      return None
    }

    // Compute areas even if candidates are active
    let areas = if solver.performance_mode { vec![] } else { self.compute_naked_single_areas(&cell, solver) };

    let value = cell_candidates.into_iter().next().unwrap();
    return Some(
      self.build_grid_solution_step(
        vec![ cell ],
        vec![ value ],
        areas,
        &solver,
      )
    );
  }

  // Try to use as few areas as possible to cover all candidates
  fn compute_naked_single_areas(&self, cell: &CellPosition, solver: &LogicalSolver) -> Vec<Area> {
    let peers = solver.state.puzzle.all_cell_peers(cell, PeerMode::Full, CandidateMode::Simple);

    let areas: Vec<Area> = peers
      .into_iter()
      .flat_map(|peer| {
        let cells = vec![*cell, peer];
        let areas = solver.state.puzzle.find_common_region_areas(&cells);
        if areas.is_empty() {
          vec![Area::Cell(peer.row, peer.col)]
        } else {
          areas
        }
      })
      .unique()
      .collect();

    let areas_values: Vec<(Area, CandidateSet)> = areas
      .iter()
      .map(|area| {
        let candidates: CandidateSet = solver
          .state
          .area_values(&area)
          .into_iter()
          .filter_map(|value| if value != 0 { Some(value) } else { None })
          .collect();
        (area.clone(), candidates)
      })
      .sorted_by_key(|(_, candidates)| -(candidates.len() as i32))
      .collect();

    let mut areas_unique_values: Vec<(Area, CandidateSet)> = vec![];
    for area_with_candidates in areas_values.into_iter() {
      if !areas_unique_values.iter().any(|(_, candidates)| area_with_candidates.1.is_subset(candidates)) {
        areas_unique_values.push(area_with_candidates);
      }
    }

    let areas_with_candidates: Vec<(Area, CandidateSet)> = areas_unique_values
      .into_iter()
      .take(4)
      .collect();

    for area_count in 1..=areas_with_candidates.len() {
      let area_indices: Vec<usize> = (0..areas_with_candidates.len()).collect();
      let area_combinations: Vec<_> = if area_count < area_indices.len() {
        Combinations::new(area_indices, area_count).collect()
      } else {
        // Has to be handled separately because of stupid crate
        vec![area_indices]
      };

      for area_combination in area_combinations {
        let selected_areas: Vec<_> = area_combination.into_iter().map(|index| &areas_with_candidates[index]).collect();

        let cell_candidates = selected_areas.iter().fold(
          solver.state.full_candidate_set(),
          |cell_candidates, (_, area_candidates)| { cell_candidates.difference(area_candidates) },
        );

        if cell_candidates.len() == 1 {
          return selected_areas
            .into_iter()
            .map(|(area, _)| area)
            .sorted()
            .cloned()
            .collect()
        }
      }
    }

    vec![]
  }
}
