use crate::solver::Solver;
use crate::types::{SolutionStep, CellPosition, Rule, Area};
use super::technique::Technique;
use combinations::Combinations;

pub struct NakedSingle;

impl Technique for NakedSingle {
  fn is_grid_step(&self) -> bool { true }
  fn get_rule(&self) -> Rule { Rule::NakedSingle }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep> {
    solver.get_all_empty_cells().into_iter().filter_map(|cell| {
      self.find_naked_single_in_cell(solver, cell)
    }).collect()
  }
}

impl NakedSingle {
  fn find_naked_single_in_cell(&self, solver: &Solver, cell: CellPosition) -> Option<SolutionStep> {
    let CellPosition { row, col } = cell;
    if solver.candidates_active {
      return self.find_naked_single_with_candidates(solver, row, col);
    }

    let candidate_areas = solver.get_cell_areas(&cell, false);

    // TODO: we may want to also consider anti knight cells
    // currently that case will be caught when candidates_active

    // Try to use as few areas as possible to cover all candidates
    for area_count in 1..=candidate_areas.len() {
      let area_indexes: Vec<usize> = (0..candidate_areas.len()).collect();
      let area_combinations: Vec<_> = if area_count < area_indexes.len() {
        Combinations::new(area_indexes, area_count).collect()
      } else {
        // Has to be handled separately because of stupid crate
        vec![area_indexes]
      };

      for area_combination in area_combinations {
        let selected_areas = area_combination.into_iter().map(|index| &candidate_areas[index]).collect();
        let step = self.find_naked_single_in_cell_and_areas(solver, &cell, selected_areas);
        if step.is_some() {
          return step
        }
      }
    }

    None
  }

  fn find_naked_single_in_cell_and_areas(&self, solver: &Solver, cell: &CellPosition, areas: Vec<&Area>) -> Option<SolutionStep> {
    let mut areas_set = solver.compute_all_candidates();
    for area in &areas {
      let area_set = solver.compute_area_cell_candidates(area, cell);
      areas_set = areas_set.intersection(&area_set).cloned().collect();
    }
    if areas_set.len() == 1 {
      let value = *areas_set.iter().next().unwrap();
      return Some(
        self.build_grid_solution_step(
          vec![ *cell ],
          vec![ value ],
          areas.into_iter().map(|area| area.clone()).collect(),
          &solver,
        )
      )
    }

    None
  }

  fn find_naked_single_with_candidates(&self, solver: &Solver, row: usize, col: usize) -> Option<SolutionStep> {
    if solver.candidates[row][col].len() != 1 {
      return None
    }

    let value = *solver.candidates[row][col].iter().next().unwrap();
    Some(
      self.build_grid_solution_step(
        vec![ CellPosition { row, col } ],
        vec![ value ],
        vec![],
        &solver,
      )
    )
  }
}
