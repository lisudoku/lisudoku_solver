use crate::solver::Solver;
use crate::types::{SolutionStep, CellPosition, Rule, Area};
use super::technique::Technique;
use combinations::Combinations;

pub struct NakedSingle;

impl Technique for NakedSingle {
  fn is_grid_step(&self) -> bool { true }
  fn get_rule(&self) -> Rule { Rule::NakedSingle }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep> {
    for cell in solver.get_all_empty_cells() {
      let step = self.find_naked_single_in_cell(solver, cell);
      if step.is_some() {
        return vec![ step.unwrap() ]
      }
    }

    vec![]
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
        SolutionStep {
          rule: Rule::NakedSingle,
          cells: vec![ *cell ],
          values: vec![ value ],
          areas: areas.into_iter().map(|x| *x).collect(),
          affected_cells: vec![],
          candidates: None,
        }
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
      SolutionStep {
        rule: self.get_rule(),
        cells: vec![ CellPosition { row, col } ],
        values: vec![ value ],
        areas: vec![],
        affected_cells: vec![],
        candidates: None,
      }
    )
  }
}
