use crate::solver::Solver;
use crate::types::{Area, Rule, SolutionStep};
use super::technique::Technique;
use super::thermo_candidates::ThermoCandidates;

pub struct Thermo;

impl Technique for Thermo {
  fn is_grid_step(&self) -> bool { true }
  fn get_rule(&self) -> Rule { Rule::Thermo }

  fn run(&self, solver: &Solver) -> Vec<SolutionStep> {
    let Some(constraint) = solver.puzzle.thermo() else { return vec![] };

    for (thermo_index, thermo) in constraint.thermos.iter().enumerate() {
      // TODO: duplicate logic in constraint definition
      let lower_bounds = ThermoCandidates::find_thermo_lower_bounds(solver, &thermo);
      let upper_bounds = ThermoCandidates::find_thermo_upper_bounds(solver, &thermo);

      for (cell_index, cell) in thermo.iter().enumerate() {
        let valid_values: Vec<u32> = solver.compute_cell_candidates(cell)
          .into_iter()
          .filter(|&value| lower_bounds[cell_index] <= value && value <= upper_bounds[cell_index])
          .collect();

        if valid_values.len() == 1 {
          return vec![
            self.build_grid_solution_step(
              vec![ *cell ],
              valid_values,
              vec![ Area::Thermo(thermo_index) ],
              &solver,
            )
          ]
        }
      }
    }

    vec![]
  }
}
