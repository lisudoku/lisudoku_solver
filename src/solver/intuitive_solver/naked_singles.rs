use crate::solver::Solver;
use crate::types::{SolutionStep, CellPosition, Rule, Area};
use combinations::Combinations;

impl Solver {
  pub fn find_naked_singles(&self) -> Option<SolutionStep> {
    for row in 0..self.constraints.grid_size {
      for col in 0..self.constraints.grid_size {
        if self.grid[row][col] != 0 {
          continue
        }

        let step = self.find_naked_single_in_cell(row, col);
        if step.is_some() {
          return step
        }
      }
    }

    None
  }

  fn find_naked_single_in_cell(&self, row: usize, col: usize) -> Option<SolutionStep> {
    let region_index = self.grid_to_region[row][col];
    let candidate_areas = [ Area::Row(row), Area::Column(col), Area::Region(region_index) ];

    for area_count in 1..candidate_areas.len()+1 {
      let area_indexes: Vec<usize> = (0..candidate_areas.len()).collect();
      let area_combinations: Vec<_> = if area_count < area_indexes.len() {
        Combinations::new(area_indexes, area_count).collect()
      } else {
        // Has to be handled separately because of stupid crate
        vec![area_indexes]
      };

      for area_combination in area_combinations {
        let selected_areas = area_combination.iter().map(|index| &candidate_areas[*index]).collect();
        let step = self.find_naked_single_in_cell_and_areas(row, col, selected_areas);
        if step.is_some() {
          return step
        }
      }
    }

    None
  }

  fn find_naked_single_in_cell_and_areas(&self, row: usize, col: usize, areas: Vec<&Area>) -> Option<SolutionStep> {
    let mut areas_set = self.compute_all_candidates();
    for area in &areas {
      let area_set = self.compute_area_cell_candidates(area, row, col);
      areas_set = areas_set.intersection(&area_set).cloned().collect();
    }
    if areas_set.len() == 1 {
      let value = *areas_set.iter().next().unwrap();
      return Some(
        SolutionStep {
          rule: Rule::NakedSingle,
          cells: vec![ CellPosition { row, col } ],
          values: vec![ value ],
          areas: areas.into_iter().map(|x| *x).collect(),
          affected_cells: vec![],
        }
      )
    }

    None
  }
}
