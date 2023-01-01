use crate::solver::Solver;
use crate::types::{SolutionStep, Rule, Area};

impl Solver {
  pub fn find_thermo_steps(&self) -> Option<SolutionStep> {
    for (thermo_index, thermo) in self.constraints.thermos.iter().enumerate() {
      let lower_bounds = self.find_thermo_lower_bounds(&thermo);
      let upper_bounds = self.find_thermo_upper_bounds(&thermo);

      for (cell_index, cell) in thermo.iter().enumerate() {
        let valid_values: Vec<u32> = self.compute_cell_candidates(cell)
          .into_iter()
          .filter(|&value| lower_bounds[cell_index] <= value && value <= upper_bounds[cell_index])
          .collect();

        if valid_values.len() == 1 {
          return Some(
            SolutionStep {
              rule: Rule::Thermo,
              cells: vec![ *cell ],
              values: valid_values,
              areas: vec![ Area::Thermo(thermo_index) ],
              affected_cells: vec![],
              candidates: None,
            }
          )
        }
      }
    }

    None
  }
}
