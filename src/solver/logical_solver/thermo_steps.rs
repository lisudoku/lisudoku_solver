use crate::solver::Solver;
use crate::types::{SolutionStep, Rule, Area};
use super::technique::Technique;
use super::thermo_candidates::ThermoCandidates;

pub struct Thermo;

impl Technique for Thermo {
  fn is_grid_step(&self) -> bool { true }
  fn get_rule(&self) -> Rule { Rule::Thermo }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep> {
    for (thermo_index, thermo) in solver.constraints.thermos.iter().enumerate() {
      let lower_bounds = ThermoCandidates::find_thermo_lower_bounds(solver, &thermo);
      let upper_bounds = ThermoCandidates::find_thermo_upper_bounds(solver, &thermo);

      for (cell_index, cell) in thermo.iter().enumerate() {
        let valid_values: Vec<u32> = solver.compute_cell_candidates(cell)
          .into_iter()
          .filter(|&value| lower_bounds[cell_index] <= value && value <= upper_bounds[cell_index])
          .collect();

        if valid_values.len() == 1 {
          return vec![
            SolutionStep {
              rule: self.get_rule(),
              cells: vec![ *cell ],
              values: valid_values,
              areas: vec![ Area::Thermo(thermo_index) ],
              affected_cells: vec![],
              candidates: None,
            }
          ]
        }
      }
    }

    vec![]
  }
}
